use super::error::{ParseError, ParseResult, ParseWarning};
use super::types::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// GEDCOM Parser - reads and parses GEDCOM files
pub struct GedcomParser {
    mode: ParseMode,
    warnings: Vec<ParseWarning>,
}

impl GedcomParser {
    pub fn new(mode: ParseMode) -> Self {
        GedcomParser {
            mode,
            warnings: Vec::new(),
        }
    }

    /// Parse a GEDCOM file from a path
    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> ParseResult<GedcomFile> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        self.parse_reader(reader)
    }

    /// Parse GEDCOM from a reader
    pub fn parse_reader<R: BufRead>(&mut self, reader: R) -> ParseResult<GedcomFile> {
        // First pass: parse lines
        let lines = self.parse_lines(reader)?;

        // Second pass: build structures
        self.build_gedcom_file(lines)
    }

    /// First pass: parse all lines into GedcomLine structures
    fn parse_lines<R: BufRead>(&mut self, reader: R) -> ParseResult<Vec<GedcomLine>> {
        let mut lines: Vec<GedcomLine> = Vec::new();
        let mut line_num = 0;
        let mut continuation_buffer: Option<(usize, String)> = None;

        for line_result in reader.lines() {
            line_num += 1;
            let line = line_result?;

            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }

            // Parse the line
            let gedcom_line = self.parse_line(&line, line_num)?;

            // Handle CONC (concatenate) and CONT (continue) tags
            if gedcom_line.tag == "CONC" || gedcom_line.tag == "CONT" {
                if let Some((prev_idx, ref mut prev_value)) = continuation_buffer {
                    let separator = if gedcom_line.tag == "CONT" { "\n" } else { "" };
                    if let Some(ref value) = gedcom_line.value {
                        prev_value.push_str(separator);
                        prev_value.push_str(value);
                    }
                    // Update the previous line's value
                    if let Some(prev_line) = lines.get_mut(prev_idx) {
                        prev_line.value = Some(prev_value.clone());
                    }
                } else if self.mode == ParseMode::Strict {
                    return Err(ParseError::InvalidLineFormat {
                        line_num,
                        line: format!("CONC/CONT without preceding value at line {}", line_num),
                    });
                }
                continue;
            }

            // Track this line for potential CONC/CONT
            if gedcom_line.value.is_some() {
                continuation_buffer = Some((lines.len(), gedcom_line.value.clone().unwrap()));
            } else {
                continuation_buffer = None;
            }

            lines.push(gedcom_line);
        }

        Ok(lines)
    }

    /// Parse a single GEDCOM line
    fn parse_line(&mut self, line: &str, line_num: usize) -> ParseResult<GedcomLine> {
        let trimmed = line.trim();
        let parts: Vec<&str> = trimmed.splitn(3, ' ').collect();

        if parts.is_empty() {
            return Err(ParseError::InvalidLineFormat {
                line_num,
                line: line.to_string(),
            });
        }

        // Parse level
        let level = parts[0]
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidLineFormat {
                line_num,
                line: line.to_string(),
            })?;

        // Determine if we have an xref (starts with @)
        let (xref, tag, value) =
            if parts.len() >= 2 && parts[1].starts_with('@') && parts[1].ends_with('@') {
                // Format: LEVEL @XREF@ TAG [VALUE]
                let xref = Some(parts[1].to_string());
                let tag = if parts.len() >= 3 {
                    parts[2].split_whitespace().next().unwrap_or("").to_string()
                } else {
                    return Err(ParseError::InvalidLineFormat {
                        line_num,
                        line: line.to_string(),
                    });
                };

                let value = if parts.len() >= 3 {
                    let tag_end = parts[2].find(&tag).unwrap() + tag.len();
                    let rest = parts[2][tag_end..].trim();
                    if rest.is_empty() {
                        None
                    } else {
                        Some(rest.to_string())
                    }
                } else {
                    None
                };

                (xref, tag, value)
            } else if parts.len() >= 2 {
                // Format: LEVEL TAG [VALUE]
                let tag = parts[1].to_string();
                let value = if parts.len() >= 3 {
                    Some(parts[2].to_string())
                } else {
                    None
                };
                (None, tag, value)
            } else {
                return Err(ParseError::InvalidLineFormat {
                    line_num,
                    line: line.to_string(),
                });
            };

        Ok(GedcomLine {
            level,
            xref,
            tag,
            value,
            line_num,
        })
    }

    /// Second pass: build GedcomFile from parsed lines
    fn build_gedcom_file(&mut self, lines: Vec<GedcomLine>) -> ParseResult<GedcomFile> {
        let mut gedcom = GedcomFile::new();
        let mut current_record: Option<String> = None; // Current xref being processed
        let mut current_record_type: Option<String> = None; // INDI or FAM
        let mut in_header = false;
        let mut in_event: Option<String> = None; // BIRT, DEAT, MARR, etc.
        let mut in_name = false;

        for line in lines {
            match line.level {
                0 => {
                    // Level 0: Start of a new record
                    if line.tag == "HEAD" {
                        in_header = true;
                        current_record = None;
                    } else if line.tag == "TRLR" {
                        // End of file
                        break;
                    } else if let Some(ref xref) = line.xref {
                        // New record with xref
                        in_header = false;
                        current_record = Some(xref.clone());
                        gedcom.record_order.push(xref.clone());

                        if line.tag == "INDI" {
                            current_record_type = Some("INDI".to_string());
                            gedcom
                                .individuals
                                .insert(xref.clone(), ParsedIndividual::new(xref.clone()));
                        } else if line.tag == "FAM" {
                            current_record_type = Some("FAM".to_string());
                            gedcom
                                .families
                                .insert(xref.clone(), ParsedFamily::new(xref.clone()));
                        } else {
                            // Unknown record type - store for lossless round-trip
                            if self.mode == ParseMode::Lenient {
                                self.warnings.push(ParseWarning::UnknownTag {
                                    line_num: line.line_num,
                                    tag: line.tag.clone(),
                                });
                            }
                        }
                    }
                    in_event = None;
                    in_name = false;
                }
                1 => {
                    // Level 1: Main fields
                    if in_header {
                        self.parse_header_line(&line, &mut gedcom.header)?;
                    } else if let Some(ref xref) = current_record {
                        match current_record_type.as_deref() {
                            Some("INDI") => {
                                self.parse_individual_line(
                                    &line,
                                    xref,
                                    &mut gedcom.individuals,
                                    &mut in_event,
                                    &mut in_name,
                                )?;
                            }
                            Some("FAM") => {
                                self.parse_family_line(
                                    &line,
                                    xref,
                                    &mut gedcom.families,
                                    &mut in_event,
                                )?;
                            }
                            _ => {}
                        }
                    }
                }
                2 => {
                    // Level 2: Sub-fields (dates, places, names)
                    if let Some(ref xref) = current_record {
                        match current_record_type.as_deref() {
                            Some("INDI") => {
                                self.parse_individual_subfield(
                                    &line,
                                    xref,
                                    &mut gedcom.individuals,
                                    &in_event,
                                    in_name,
                                )?;
                            }
                            Some("FAM") => {
                                self.parse_family_subfield(
                                    &line,
                                    xref,
                                    &mut gedcom.families,
                                    &in_event,
                                )?;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {
                    // Level 3+: Ignore for now (or store for lossless round-trip)
                }
            }
        }

        Ok(gedcom)
    }

    fn parse_header_line(&mut self, line: &GedcomLine, header: &mut Header) -> ParseResult<()> {
        match line.tag.as_str() {
            "CHAR" => {
                header.encoding = line.value.clone().unwrap_or_else(|| "UTF-8".to_string());
            }
            "SOUR" => {
                header.source = line.value.clone();
            }
            "DATE" => {
                header.date = line.value.clone();
            }
            "LANG" => {
                header.language = line.value.clone();
            }
            _ => {
                // Unknown header tag
            }
        }
        Ok(())
    }

    fn parse_individual_line(
        &mut self,
        line: &GedcomLine,
        xref: &str,
        individuals: &mut HashMap<String, ParsedIndividual>,
        in_event: &mut Option<String>,
        in_name: &mut bool,
    ) -> ParseResult<()> {
        let individual = individuals.get_mut(xref).unwrap();

        match line.tag.as_str() {
            "NAME" => {
                individual.name = line.value.clone();
                *in_name = true;
                *in_event = None;
            }
            "SEX" => {
                individual.sex = line.value.clone();
                *in_event = None;
                *in_name = false;
            }
            "BIRT" => {
                *in_event = Some("BIRT".to_string());
                *in_name = false;
            }
            "DEAT" => {
                *in_event = Some("DEAT".to_string());
                *in_name = false;
            }
            "FAMC" => {
                if let Some(ref value) = line.value {
                    individual.parent_family_xrefs.push(value.clone());
                }
                *in_event = None;
                *in_name = false;
            }
            "FAMS" => {
                if let Some(ref value) = line.value {
                    individual.spouse_family_xrefs.push(value.clone());
                }
                *in_event = None;
                *in_name = false;
            }
            _ => {
                *in_event = None;
                *in_name = false;
            }
        }

        Ok(())
    }

    fn parse_individual_subfield(
        &mut self,
        line: &GedcomLine,
        xref: &str,
        individuals: &mut HashMap<String, ParsedIndividual>,
        in_event: &Option<String>,
        in_name: bool,
    ) -> ParseResult<()> {
        let individual = individuals.get_mut(xref).unwrap();

        if in_name {
            match line.tag.as_str() {
                "GIVN" => {
                    individual.given_name = line.value.clone();
                }
                "SURN" => {
                    individual.surname = line.value.clone();
                }
                _ => {}
            }
        } else if let Some(ref event) = in_event {
            match event.as_str() {
                "BIRT" => match line.tag.as_str() {
                    "DATE" => {
                        individual.birth_date = line.value.clone();
                    }
                    "PLAC" => {
                        individual.birth_place = line.value.clone();
                    }
                    _ => {}
                },
                "DEAT" => match line.tag.as_str() {
                    "DATE" => {
                        individual.death_date = line.value.clone();
                    }
                    "PLAC" => {
                        individual.death_place = line.value.clone();
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn parse_family_line(
        &mut self,
        line: &GedcomLine,
        xref: &str,
        families: &mut HashMap<String, ParsedFamily>,
        in_event: &mut Option<String>,
    ) -> ParseResult<()> {
        let family = families.get_mut(xref).unwrap();

        match line.tag.as_str() {
            "HUSB" => {
                family.husband_xref = line.value.clone();
                *in_event = None;
            }
            "WIFE" => {
                family.wife_xref = line.value.clone();
                *in_event = None;
            }
            "CHIL" => {
                if let Some(ref value) = line.value {
                    family.children_xrefs.push(value.clone());
                }
                *in_event = None;
            }
            "MARR" => {
                *in_event = Some("MARR".to_string());
            }
            "DIV" => {
                *in_event = Some("DIV".to_string());
            }
            _ => {
                *in_event = None;
            }
        }

        Ok(())
    }

    fn parse_family_subfield(
        &mut self,
        line: &GedcomLine,
        xref: &str,
        families: &mut HashMap<String, ParsedFamily>,
        in_event: &Option<String>,
    ) -> ParseResult<()> {
        let family = families.get_mut(xref).unwrap();

        if let Some(ref event) = in_event {
            match event.as_str() {
                "MARR" => match line.tag.as_str() {
                    "DATE" => {
                        family.marriage_date = line.value.clone();
                    }
                    "PLAC" => {
                        family.marriage_place = line.value.clone();
                    }
                    _ => {}
                },
                "DIV" => {
                    if line.tag.as_str() == "DATE" {
                        family.divorce_date = line.value.clone();
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Get all warnings from the parse
    pub fn warnings(&self) -> &[ParseWarning] {
        &self.warnings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_line_simple() {
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let line = parser.parse_line("0 HEAD", 1).unwrap();
        assert_eq!(line.level, 0);
        assert_eq!(line.tag, "HEAD");
        assert!(line.value.is_none());
    }

    #[test]
    fn test_parse_line_with_value() {
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let line = parser.parse_line("1 NAME John /Doe/", 1).unwrap();
        assert_eq!(line.level, 1);
        assert_eq!(line.tag, "NAME");
        assert_eq!(line.value, Some("John /Doe/".to_string()));
    }

    #[test]
    fn test_parse_line_with_xref() {
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let line = parser.parse_line("0 @I1@ INDI", 1).unwrap();
        assert_eq!(line.level, 0);
        assert_eq!(line.xref, Some("@I1@".to_string()));
        assert_eq!(line.tag, "INDI");
    }

    #[test]
    fn test_parse_simple_gedcom() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8
0 @I1@ INDI
1 NAME John /Doe/
2 GIVN John
2 SURN Doe
1 SEX M
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let gedcom = parser.parse_reader(BufReader::new(cursor)).unwrap();

        assert_eq!(gedcom.individuals.len(), 1);
        let individual = gedcom.individuals.get("@I1@").unwrap();
        assert_eq!(individual.name, Some("John /Doe/".to_string()));
        assert_eq!(individual.given_name, Some("John".to_string()));
        assert_eq!(individual.surname, Some("Doe".to_string()));
        assert_eq!(individual.sex, Some("M".to_string()));
    }
}
