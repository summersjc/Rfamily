use std::collections::HashMap;

/// GEDCOM version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GedcomVersion {
    V5_5,
    #[default]
    V5_5_1,
    V7_0,
}

/// Parse mode: strict or lenient
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParseMode {
    /// Reject invalid GEDCOM
    Strict,
    /// Accept messy real-world files
    #[default]
    Lenient,
}

/// GEDCOM header information
#[derive(Debug, Clone, Default)]
pub struct Header {
    pub version: GedcomVersion,
    pub encoding: String, // UTF-8, ANSEL, ASCII
    pub source: Option<String>,
    pub source_version: Option<String>,
    pub date: Option<String>,
    pub language: Option<String>,
}

/// Parsed individual record
#[derive(Debug, Clone)]
pub struct ParsedIndividual {
    pub xref: String,         // @I1@
    pub name: Option<String>, // Full name
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub sex: Option<String>, // M, F, U
    pub birth_date: Option<String>,
    pub birth_place: Option<String>,
    pub death_date: Option<String>,
    pub death_place: Option<String>,
    pub parent_family_xrefs: Vec<String>, // FAMC
    pub spouse_family_xrefs: Vec<String>, // FAMS

    // For lossless round-trip
    pub unknown_tags: HashMap<String, Vec<String>>,
}

impl ParsedIndividual {
    pub fn new(xref: String) -> Self {
        ParsedIndividual {
            xref,
            name: None,
            given_name: None,
            surname: None,
            sex: None,
            birth_date: None,
            birth_place: None,
            death_date: None,
            death_place: None,
            parent_family_xrefs: Vec::new(),
            spouse_family_xrefs: Vec::new(),
            unknown_tags: HashMap::new(),
        }
    }
}

/// Parsed family record
#[derive(Debug, Clone)]
pub struct ParsedFamily {
    pub xref: String, // @F1@
    pub husband_xref: Option<String>,
    pub wife_xref: Option<String>,
    pub children_xrefs: Vec<String>,
    pub marriage_date: Option<String>,
    pub marriage_place: Option<String>,
    pub divorce_date: Option<String>,

    // For lossless round-trip
    pub unknown_tags: HashMap<String, Vec<String>>,
}

impl ParsedFamily {
    pub fn new(xref: String) -> Self {
        ParsedFamily {
            xref,
            husband_xref: None,
            wife_xref: None,
            children_xrefs: Vec::new(),
            marriage_date: None,
            marriage_place: None,
            divorce_date: None,
            unknown_tags: HashMap::new(),
        }
    }
}

/// Complete parsed GEDCOM file
#[derive(Debug, Clone, Default)]
pub struct GedcomFile {
    pub header: Header,
    pub individuals: HashMap<String, ParsedIndividual>,
    pub families: HashMap<String, ParsedFamily>,

    // For lossless round-trip (preserve order and unknown records)
    pub raw_lines: Option<Vec<String>>,
    pub record_order: Vec<String>, // [@I1@, @F1@, @I2@, ...]
}

impl GedcomFile {
    pub fn new() -> Self {
        GedcomFile {
            header: Header::default(),
            individuals: HashMap::new(),
            families: HashMap::new(),
            raw_lines: None,
            record_order: Vec::new(),
        }
    }
}

/// GEDCOM line structure
#[derive(Debug, Clone)]
pub struct GedcomLine {
    pub level: usize,
    pub xref: Option<String>, // @I1@
    pub tag: String,
    pub value: Option<String>,
    pub line_num: usize,
}

impl GedcomLine {
    pub fn new(level: usize, tag: String, line_num: usize) -> Self {
        GedcomLine {
            level,
            xref: None,
            tag,
            value: None,
            line_num,
        }
    }
}
