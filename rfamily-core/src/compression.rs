use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, BufWriter, Write};

/// Enum for handling both plain and compressed output
pub enum OutputWriter {
    Plain(BufWriter<File>),
    Compressed(BufWriter<GzEncoder<File>>),
}

impl Write for OutputWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            OutputWriter::Plain(w) => w.write(buf),
            OutputWriter::Compressed(w) => w.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            OutputWriter::Plain(w) => w.flush(),
            OutputWriter::Compressed(w) => w.flush(),
        }
    }
}

impl OutputWriter {
    /// Create a new output writer (plain or compressed based on flag)
    pub fn create(path: &str, compress: bool) -> io::Result<Self> {
        let file = File::create(path)?;

        if compress {
            let encoder = GzEncoder::new(file, Compression::default());
            Ok(OutputWriter::Compressed(BufWriter::new(encoder)))
        } else {
            Ok(OutputWriter::Plain(BufWriter::new(file)))
        }
    }

    /// Finish writing and close the file
    /// This is important for compression to finalize the gzip stream
    pub fn finish(self) -> io::Result<()> {
        match self {
            OutputWriter::Plain(mut w) => w.flush(),
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

/// Add .gz extension if compressing and not already present
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
