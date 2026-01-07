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

    // Error handling tests
    #[test]
    fn test_parse_invalid_line_format() {
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let result = parser.parse_line("", 1);
        assert!(result.is_err());

        let result = parser.parse_line("not a number TAG", 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_missing_tag() {
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let result = parser.parse_line("0", 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_incomplete_xref() {
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let result = parser.parse_line("0 @I1@", 1);
        assert!(result.is_err(), "Should fail with incomplete xref line");
    }

    // CONC/CONT continuation tests
    #[test]
    fn test_parse_conc_continuation() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8
0 @I1@ INDI
1 NAME John
2 CONC  /Doe/
1 SEX M
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let gedcom = parser.parse_reader(BufReader::new(cursor)).unwrap();

        let individual = gedcom.individuals.get("@I1@").unwrap();
        assert_eq!(individual.name, Some("John /Doe/".to_string()));
    }

    #[test]
    fn test_parse_cont_continuation() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8
0 @N1@ NOTE This is a long note
1 CONT that continues on the next line
1 CONT and another line
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let result = parser.parse_reader(BufReader::new(cursor));
        assert!(result.is_ok(), "Should handle CONT continuation");
    }

    // Family record tests
    #[test]
    fn test_parse_family_record() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8
0 @I1@ INDI
1 NAME John /Doe/
1 SEX M
0 @I2@ INDI
1 NAME Jane /Smith/
1 SEX F
0 @F1@ FAM
1 HUSB @I1@
1 WIFE @I2@
1 MARR
2 DATE 1 JAN 2000
2 PLAC New York, USA
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let gedcom = parser.parse_reader(BufReader::new(cursor)).unwrap();

        assert_eq!(gedcom.individuals.len(), 2);
        assert_eq!(gedcom.families.len(), 1);

        let family = gedcom.families.get("@F1@").unwrap();
        assert_eq!(family.husband_xref, Some("@I1@".to_string()));
        assert_eq!(family.wife_xref, Some("@I2@".to_string()));
        assert_eq!(family.marriage_date, Some("1 JAN 2000".to_string()));
        assert_eq!(family.marriage_place, Some("New York, USA".to_string()));
    }

    #[test]
    fn test_parse_family_with_children() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8
0 @I1@ INDI
1 NAME Father /Smith/
1 SEX M
0 @I2@ INDI
1 NAME Mother /Jones/
1 SEX F
0 @I3@ INDI
1 NAME Child1 /Smith/
1 SEX M
1 FAMC @F1@
0 @I4@ INDI
1 NAME Child2 /Smith/
1 SEX F
1 FAMC @F1@
0 @F1@ FAM
1 HUSB @I1@
1 WIFE @I2@
1 CHIL @I3@
1 CHIL @I4@
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let gedcom = parser.parse_reader(BufReader::new(cursor)).unwrap();

        assert_eq!(gedcom.individuals.len(), 4);
        assert_eq!(gedcom.families.len(), 1);

        let family = gedcom.families.get("@F1@").unwrap();
        assert_eq!(family.children_xrefs.len(), 2);
        assert!(family.children_xrefs.contains(&"@I3@".to_string()));
        assert!(family.children_xrefs.contains(&"@I4@".to_string()));

        let child = gedcom.individuals.get("@I3@").unwrap();
        assert!(child.parent_family_xrefs.contains(&"@F1@".to_string()));
    }

    #[test]
    fn test_parse_individual_with_birth_death() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8
0 @I1@ INDI
1 NAME John /Doe/
1 SEX M
1 BIRT
2 DATE 15 MAR 1950
2 PLAC Boston, Massachusetts, USA
1 DEAT
2 DATE 20 DEC 2020
2 PLAC Miami, Florida, USA
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let gedcom = parser.parse_reader(BufReader::new(cursor)).unwrap();

        let individual = gedcom.individuals.get("@I1@").unwrap();
        assert_eq!(individual.birth_date, Some("15 MAR 1950".to_string()));
        assert_eq!(
            individual.birth_place,
            Some("Boston, Massachusetts, USA".to_string())
        );
        assert_eq!(individual.death_date, Some("20 DEC 2020".to_string()));
        assert_eq!(
            individual.death_place,
            Some("Miami, Florida, USA".to_string())
        );
    }

    #[test]
    fn test_parse_divorce() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8
0 @F1@ FAM
1 HUSB @I1@
1 WIFE @I2@
1 MARR
2 DATE 1 JAN 2000
1 DIV
2 DATE 15 JUN 2010
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let gedcom = parser.parse_reader(BufReader::new(cursor)).unwrap();

        let family = gedcom.families.get("@F1@").unwrap();
        assert_eq!(family.divorce_date, Some("15 JUN 2010".to_string()));
    }

    // Strict mode tests
    #[test]
    fn test_strict_mode_with_unknown_tag() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8
0 @I1@ INDI
1 NAME John /Doe/
1 CUSTOM_TAG Some Value
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Strict);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let result = parser.parse_reader(BufReader::new(cursor));
        // In strict mode, unknown tags at level 0 might be handled
        // This test documents current behavior
        assert!(result.is_ok());
    }

    #[test]
    fn test_lenient_mode_collects_warnings() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8
0 @UNKNOWN@ CUSTOM
1 NAME Test
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let _result = parser.parse_reader(BufReader::new(cursor)).unwrap();

        assert!(
            !parser.warnings().is_empty(),
            "Should collect warnings for unknown tags"
        );
    }

    // Edge cases
    #[test]
    fn test_parse_empty_lines() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8

0 @I1@ INDI
1 NAME John /Doe/

0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let gedcom = parser.parse_reader(BufReader::new(cursor)).unwrap();

        assert_eq!(gedcom.individuals.len(), 1);
    }

    #[test]
    fn test_parse_multiple_individuals() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8
0 @I1@ INDI
1 NAME Person One
1 SEX M
0 @I2@ INDI
1 NAME Person Two
1 SEX F
0 @I3@ INDI
1 NAME Person Three
1 SEX M
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let gedcom = parser.parse_reader(BufReader::new(cursor)).unwrap();

        assert_eq!(gedcom.individuals.len(), 3);
        assert_eq!(gedcom.record_order.len(), 3);
        assert_eq!(gedcom.record_order[0], "@I1@");
        assert_eq!(gedcom.record_order[1], "@I2@");
        assert_eq!(gedcom.record_order[2], "@I3@");
    }

    #[test]
    fn test_parse_header_fields() {
        let gedcom_data = "0 HEAD
1 SOUR FamilyTree
1 CHAR UTF-8
1 LANG English
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let gedcom = parser.parse_reader(BufReader::new(cursor)).unwrap();

        assert_eq!(gedcom.header.encoding, "UTF-8");
        assert_eq!(gedcom.header.source, Some("FamilyTree".to_string()));
        assert_eq!(gedcom.header.language, Some("English".to_string()));
    }

    #[test]
    fn test_parse_complex_family_structure() {
        let gedcom_data = "0 HEAD
1 CHAR UTF-8
0 @I1@ INDI
1 NAME John /Doe/
1 SEX M
1 FAMS @F1@
1 FAMS @F2@
0 @I2@ INDI
1 NAME Jane /Smith/
1 SEX F
1 FAMS @F1@
0 @I3@ INDI
1 NAME Mary /Johnson/
1 SEX F
1 FAMS @F2@
0 @I4@ INDI
1 NAME Child /Doe/
1 SEX M
1 FAMC @F1@
0 @F1@ FAM
1 HUSB @I1@
1 WIFE @I2@
1 CHIL @I4@
1 MARR
2 DATE 1 JAN 2000
1 DIV
2 DATE 1 JAN 2005
0 @F2@ FAM
1 HUSB @I1@
1 WIFE @I3@
1 MARR
2 DATE 1 JAN 2006
0 TRLR
";
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let cursor = Cursor::new(gedcom_data.as_bytes());
        let gedcom = parser.parse_reader(BufReader::new(cursor)).unwrap();

        assert_eq!(gedcom.individuals.len(), 4);
        assert_eq!(gedcom.families.len(), 2);

        let person = gedcom.individuals.get("@I1@").unwrap();
        assert_eq!(person.spouse_family_xrefs.len(), 2);

        let family1 = gedcom.families.get("@F1@").unwrap();
        assert!(family1.divorce_date.is_some());

        let family2 = gedcom.families.get("@F2@").unwrap();
        assert!(family2.divorce_date.is_none());
    }
}
