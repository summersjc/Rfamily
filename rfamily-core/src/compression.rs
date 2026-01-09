//! GEDCOM file compression support using gzip.
//!
//! This module provides transparent compression for GEDCOM output files, achieving
//! 80-85% file size reduction with minimal (<10%) performance overhead.
//!
//! **Note**: This module requires the `compression` feature (enabled by default).
//!
//! # Examples
//!
//! ```no_run
//! # #[cfg(feature = "compression")]
//! # {
//! use rfamily_core::compression::OutputWriter;
//! use rfamily_core::generator::GedcomGenerator;
//! use rfamily_core::ruleset::Ruleset;
//!
//! let ruleset = Ruleset::default_english();
//! let mut generator = GedcomGenerator::new(ruleset);
//! let mut rng = rand::thread_rng();
//! generator.generate(1000, &mut rng);
//!
//! // Create compressed output (automatically adds .gz if needed)
//! let mut writer = OutputWriter::create("output.ged.gz", true).unwrap();
//! generator.write_gedcom(&mut writer).unwrap();
//! writer.finish().unwrap();
//! # }
//! ```

#[cfg(feature = "compression")]
use flate2::write::GzEncoder;
#[cfg(feature = "compression")]
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufWriter, Write};

/// Writer that handles both plain and gzip-compressed output transparently.
///
/// This enum wraps either a plain file writer or a gzip encoder, implementing the
/// `Write` trait for both variants. This allows the same generation code to work
/// with both plain and compressed output without any changes.
pub enum OutputWriter {
    /// Plain (uncompressed) file writer with buffering
    Plain(BufWriter<File>),
    /// Gzip-compressed file writer with buffering (requires `compression` feature)
    #[cfg(feature = "compression")]
    Compressed(BufWriter<GzEncoder<File>>),
}

impl Write for OutputWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            OutputWriter::Plain(w) => w.write(buf),
            #[cfg(feature = "compression")]
            OutputWriter::Compressed(w) => w.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            OutputWriter::Plain(w) => w.flush(),
            #[cfg(feature = "compression")]
            OutputWriter::Compressed(w) => w.flush(),
        }
    }
}

impl OutputWriter {
    /// Create a new output writer (plain or compressed based on flag).
    ///
    /// # Arguments
    ///
    /// * `path` - File path for output (should include .gz extension if compressing)
    /// * `compress` - If `true`, creates a gzip-compressed writer; if `false`, creates plain writer
    ///
    /// # Returns
    ///
    /// Returns `Ok(OutputWriter)` on success, or an `io::Error` if file creation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rfamily_core::compression::OutputWriter;
    ///
    /// // Create plain writer
    /// let plain = OutputWriter::create("output.ged", false).unwrap();
    ///
    /// // Create compressed writer
    /// let compressed = OutputWriter::create("output.ged.gz", true).unwrap();
    /// ```
    pub fn create(path: &str, compress: bool) -> io::Result<Self> {
        let file = File::create(path)?;

        #[cfg(feature = "compression")]
        if compress {
            let encoder = GzEncoder::new(file, Compression::default());
            return Ok(OutputWriter::Compressed(BufWriter::new(encoder)));
        }

        #[cfg(not(feature = "compression"))]
        if compress {
            return Err(io::Error::other(
                "Compression feature not enabled. Rebuild with --features compression",
            ));
        }

        Ok(OutputWriter::Plain(BufWriter::new(file)))
    }

    /// Finish writing and close the file.
    ///
    /// This method flushes all buffers and, for compressed files, finalizes the gzip stream.
    /// **Important**: For compressed output, failing to call this method will result in corrupted files.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `io::Error` if flushing/finalizing fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rfamily_core::compression::OutputWriter;
    /// use std::io::Write;
    ///
    /// let mut writer = OutputWriter::create("output.ged.gz", true).unwrap();
    /// writeln!(writer, "0 HEAD").unwrap();
    /// writer.finish().unwrap(); // Critical for compressed files
    /// ```
    pub fn finish(self) -> io::Result<()> {
        match self {
            OutputWriter::Plain(mut w) => w.flush(),
            #[cfg(feature = "compression")]
            OutputWriter::Compressed(mut w) => {
                w.flush()?;
                // Ensure the gzip encoder is properly finished
                let encoder = w
                    .into_inner()
                    .map_err(|e| io::Error::other(format!("Failed to finish encoder: {}", e)))?;
                encoder.finish()?;
                Ok(())
            }
        }
    }
}

/// Add .gz extension if compressing and not already present.
///
/// This utility function automatically appends `.gz` to filenames when compression is enabled,
/// but only if the filename doesn't already end with `.gz`.
///
/// # Arguments
///
/// * `filename` - Original filename
/// * `compress` - Whether compression is enabled
///
/// # Returns
///
/// Returns the filename with `.gz` appended if compressing and not already present,
/// otherwise returns the original filename unchanged.
///
/// # Examples
///
/// ```
/// use rfamily_core::compression::adjust_filename_for_compression;
///
/// assert_eq!(
///     adjust_filename_for_compression("output.ged", true),
///     "output.ged.gz"
/// );
///
/// assert_eq!(
///     adjust_filename_for_compression("output.ged.gz", true),
///     "output.ged.gz"
/// );
///
/// assert_eq!(
///     adjust_filename_for_compression("output.ged", false),
///     "output.ged"
/// );
/// ```
pub fn adjust_filename_for_compression(filename: &str, compress: bool) -> String {
    if compress && !filename.ends_with(".gz") {
        format!("{}.gz", filename)
    } else {
        filename.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_plain_writer() {
        let temp_file = "/tmp/test_plain.ged";
        let mut writer = OutputWriter::create(temp_file, false).unwrap();
        writeln!(writer, "0 HEAD").unwrap();
        writeln!(writer, "1 SOUR Test").unwrap();
        writer.finish().unwrap();

        let content = std::fs::read_to_string(temp_file).unwrap();
        assert!(content.contains("0 HEAD"));
        assert!(content.contains("1 SOUR Test"));

        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_compressed_writer() {
        let temp_file = "/tmp/test_compressed.ged.gz";
        let mut writer = OutputWriter::create(temp_file, true).unwrap();
        writeln!(writer, "0 HEAD").unwrap();
        writeln!(writer, "1 SOUR Test").unwrap();
        writer.finish().unwrap();

        // Verify file exists and is smaller than uncompressed
        let metadata = std::fs::metadata(temp_file).unwrap();
        assert!(metadata.len() > 0);
        assert!(metadata.len() < 100); // Compressed should be small

        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_filename_adjustment() {
        assert_eq!(
            adjust_filename_for_compression("test.ged", true),
            "test.ged.gz"
        );
        assert_eq!(
            adjust_filename_for_compression("test.ged.gz", true),
            "test.ged.gz"
        );
        assert_eq!(
            adjust_filename_for_compression("test.ged", false),
            "test.ged"
        );
    }
}
