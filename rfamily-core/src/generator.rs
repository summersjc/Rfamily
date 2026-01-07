use crate::ruleset::*;
use chrono::{Datelike, NaiveDate};
use rand::Rng;
use std::collections::HashMap;
use std::io::{BufWriter, Write};

pub struct GedcomGenerator {
    pub(crate) ruleset: Ruleset,
    pub(crate) individuals: HashMap<usize, Individual>,
    pub(crate) families: Vec<Family>,
    pub(crate) next_indi_id: usize,
    pub(crate) next_fam_id: usize,
}

#[derive(Debug, Clone)]
pub struct Individual {
    pub id: usize,
    pub given_name: String,
    pub surname: String,
    pub sex: Sex,
    pub birth_date: NaiveDate,
    pub birth_place: String,
    pub death_date: Option<NaiveDate>,
    pub death_place: Option<String>,
    pub language: String,
    pub parent_family_id: Option<usize>,
    pub spouse_family_ids: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Family {
    pub id: usize,
    pub husband_id: Option<usize>,
    pub wife_id: Option<usize>,
    pub children_ids: Vec<usize>,
    pub marriage_date: Option<NaiveDate>,
    pub marriage_place: Option<String>,
    pub divorce_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Copy)]
pub enum Sex {
    Male,
    Female,
}

impl GedcomGenerator {
    pub fn new(ruleset: Ruleset) -> Self {
        GedcomGenerator {
            ruleset,
            individuals: HashMap::new(),
            families: Vec::new(),
            next_indi_id: 1,
            next_fam_id: 1,
        }
    }

    pub fn generate(&mut self, count: usize, rng: &mut impl Rng) {
        if self.ruleset.relationships.generate_families {
            self.generate_families(count, rng);
        } else {
            // Use parallel generation for better performance
            self.generate_individuals_parallel(count);
        }
    }

    #[allow(dead_code)]
    fn generate_individuals(&mut self, count: usize, rng: &mut impl Rng) {
        for _ in 0..count {
            let individual = self.create_individual(None, None, rng);
            self.individuals.insert(individual.id, individual);
        }
    }

    fn generate_individuals_parallel(&mut self, count: usize) {
        use rayon::prelude::*;
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::Arc;

        let id_counter = AtomicUsize::new(self.next_indi_id);
        let ruleset = Arc::new(self.ruleset.clone());

        // Generate in parallel
        let individuals: Vec<Individual> = (0..count)
            .into_par_iter()
            .map(|_| {
                let id = id_counter.fetch_add(1, Ordering::Relaxed);
                let mut rng = rand::thread_rng();
                Self::create_individual_static(id, None, None, &ruleset, &mut rng)
            })
            .collect();

        // Insert into HashMap (sequential, but fast)
        for indi in individuals {
            self.individuals.insert(indi.id, indi);
        }

        self.next_indi_id = id_counter.load(Ordering::Relaxed);
    }

    // Static method for parallel individual creation
    fn create_individual_static(
        id: usize,
        _father_id: Option<usize>,
        _mother_id: Option<usize>,
        ruleset: &Ruleset,
        rng: &mut impl Rng,
    ) -> Individual {
        let sex = if rng.gen_bool(ruleset.demographics.sex_ratio) {
            Sex::Male
        } else {
            Sex::Female
        };

        let given_name = Self::select_given_name_static(&sex, ruleset, rng);
        let surname = Self::select_surname_static(ruleset, rng);
        let birth_date = Self::generate_birth_date_static(ruleset, rng);
        let birth_place = Self::select_location_static(ruleset, rng);
        let language = Self::select_language_static(ruleset, rng);

        let death_date = if ruleset.dates.include_death_dates {
            Self::generate_death_date_static(birth_date, ruleset, rng)
        } else {
            None
        };

        let death_place = death_date.map(|_| Self::select_location_static(ruleset, rng));

        Individual {
            id,
            given_name,
            surname,
            sex,
            birth_date,
            birth_place,
            death_date,
            death_place,
            language,
            parent_family_id: None,
            spouse_family_ids: Vec::new(),
        }
    }

    fn select_given_name_static(sex: &Sex, ruleset: &Ruleset, rng: &mut impl Rng) -> String {
        let names = match sex {
            Sex::Male => &ruleset.names.male_given_names,
            Sex::Female => &ruleset.names.female_given_names,
        };
        names[rng.gen_range(0..names.len())].clone()
    }

    fn select_surname_static(ruleset: &Ruleset, rng: &mut impl Rng) -> String {
        if !ruleset.names.surnames.is_empty() {
            ruleset.names.surnames[rng.gen_range(0..ruleset.names.surnames.len())].clone()
        } else {
            "Unknown".to_string()
        }
    }

    fn select_location_static(ruleset: &Ruleset, rng: &mut impl Rng) -> String {
        let country = &ruleset.locations.countries[0];
        if country.cities.is_empty() {
            country.name.clone()
        } else {
            format!(
                "{}, {}",
                country.cities[rng.gen_range(0..country.cities.len())],
                country.name
            )
        }
    }

    fn select_language_static(ruleset: &Ruleset, rng: &mut impl Rng) -> String {
        ruleset.demographics.languages[rng.gen_range(0..ruleset.demographics.languages.len())]
            .clone()
    }

    fn generate_birth_date_static(ruleset: &Ruleset, rng: &mut impl Rng) -> NaiveDate {
        let year = rng.gen_range(ruleset.dates.birth_year_start..=ruleset.dates.birth_year_end);
        let month = rng.gen_range(1..=12);
        let day = rng.gen_range(1..=28);
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    fn generate_death_date_static(
        birth_date: NaiveDate,
        ruleset: &Ruleset,
        rng: &mut impl Rng,
    ) -> Option<NaiveDate> {
        use rand_distr::{Distribution, Normal};

        let normal = Normal::new(
            ruleset.dates.life_expectancy_mean as f64,
            ruleset.dates.life_expectancy_mean as f64 / 6.0,
        )
        .unwrap();

        let age = normal.sample(rng).clamp(0.0, 120.0);
        let death_year = birth_date.year() + age as i32;

        if death_year > 2024 {
            return None;
        }

        let month = rng.gen_range(1..=12);
        let day = rng.gen_range(1..=28);
        NaiveDate::from_ymd_opt(death_year, month, day)
    }

    fn generate_families(&mut self, target_count: usize, rng: &mut impl Rng) {
        // Generate founding generation
        let founding_couples = (target_count as f64
            / (self.ruleset.relationships.generations as f64 * 2.0))
            .ceil() as usize;

        for _ in 0..founding_couples {
            self.create_family(None, rng);
        }

        // Generate subsequent generations
        for _gen in 1..self.ruleset.relationships.generations {
            let current_individuals: Vec<usize> = self
                .individuals
                .values()
                .filter(|i| i.parent_family_id.is_some())
                .map(|i| i.id)
                .collect();

            for &person_id in &current_individuals {
                if self.individuals.len() >= target_count {
                    return;
                }

                // O(1) lookup instead of O(n) search
                let person = self.individuals.get(&person_id).unwrap().clone();
                let age = 2024 - person.birth_date.year();

                if age >= self.ruleset.dates.min_marriage_age
                    && rng.gen_bool(self.ruleset.relationships.marriage_probability)
                {
                    self.create_family(Some(person_id), rng);
                }
            }
        }
    }

    fn create_family(&mut self, person_id: Option<usize>, rng: &mut impl Rng) -> usize {
        let family_id = self.next_fam_id;
        self.next_fam_id += 1;

        let (husband, wife) = if let Some(pid) = person_id {
            // O(1) lookup instead of O(n) search
            let person = self.individuals.get(&pid).unwrap().clone();
            match person.sex {
                Sex::Male => {
                    let wife = self.create_spouse(&person, rng);
                    let wife_id = wife.id;
                    self.individuals.insert(wife_id, wife);
                    (pid, wife_id)
                }
                Sex::Female => {
                    let husband = self.create_spouse(&person, rng);
                    let husband_id = husband.id;
                    self.individuals.insert(husband_id, husband);
                    (husband_id, pid)
                }
            }
        } else {
            let husband = self.create_individual(None, None, rng);
            let husband_id = husband.id;
            self.individuals.insert(husband_id, husband);

            // O(1) lookup instead of O(n) search
            let husband_ref = self.individuals.get(&husband_id).unwrap().clone();
            let wife = self.create_spouse(&husband_ref, rng);
            let wife_id = wife.id;
            self.individuals.insert(wife_id, wife);

            (husband_id, wife_id)
        };

        // Update individuals with family reference - O(1) lookup instead of O(n) search
        if let Some(h) = self.individuals.get_mut(&husband) {
            h.spouse_family_ids.push(family_id);
        }
        if let Some(w) = self.individuals.get_mut(&wife) {
            w.spouse_family_ids.push(family_id);
        }

        // O(1) lookups instead of O(n) searches
        let husband_birth = self.individuals.get(&husband).unwrap().birth_date;
        let wife_birth = self.individuals.get(&wife).unwrap().birth_date;

        let marriage_date = self.generate_marriage_date(husband_birth, wife_birth, rng);
        let marriage_place = self.select_location(rng);

        // Generate children
        let num_children = self.calculate_num_children(rng);
        let mut children_ids = Vec::new();

        for _ in 0..num_children {
            let mut child = self.create_individual(Some(husband), Some(wife), rng);
            child.parent_family_id = Some(family_id);
            children_ids.push(child.id);
            self.individuals.insert(child.id, child);
        }

        let divorce_date = if rng.gen_bool(self.ruleset.relationships.divorce_probability) {
            Some(self.generate_divorce_date(marriage_date, rng))
        } else {
            None
        };

        let family = Family {
            id: family_id,
            husband_id: Some(husband),
            wife_id: Some(wife),
            children_ids,
            marriage_date: Some(marriage_date),
            marriage_place: Some(marriage_place),
            divorce_date,
        };

        self.families.push(family);
        family_id
    }

    fn create_individual(
        &mut self,
        father_id: Option<usize>,
        mother_id: Option<usize>,
        rng: &mut impl Rng,
    ) -> Individual {
        let id = self.next_indi_id;
        self.next_indi_id += 1;

        let sex = if rng.gen_bool(self.ruleset.demographics.sex_ratio) {
            Sex::Male
        } else {
            Sex::Female
        };

        let given_name = self.select_given_name(&sex, rng);
        let surname = self.select_surname(father_id, mother_id, &sex, rng);
        let birth_date = self.generate_birth_date(father_id, mother_id, rng);
        let birth_place = self.select_location(rng);
        let language = self.select_language(rng);

        let death_date = if self.ruleset.dates.include_death_dates {
            self.generate_death_date(birth_date, rng)
        } else {
            None
        };

        let death_place = death_date.map(|_| self.select_location(rng));

        Individual {
            id,
            given_name,
            surname,
            sex,
            birth_date,
            birth_place,
            death_date,
            death_place,
            language,
            parent_family_id: None,
            spouse_family_ids: Vec::new(),
        }
    }

    fn create_spouse(&mut self, person: &Individual, rng: &mut impl Rng) -> Individual {
        let id = self.next_indi_id;
        self.next_indi_id += 1;

        let sex = match person.sex {
            Sex::Male => Sex::Female,
            Sex::Female => Sex::Male,
        };

        let given_name = self.select_given_name(&sex, rng);
        let surname = if matches!(sex, Sex::Female) && !self.ruleset.names.use_patronymic {
            self.select_surname(None, None, &sex, rng)
        } else {
            person.surname.clone()
        };

        let age_diff = rng.gen_range(-5..6);
        let birth_date = person.birth_date + chrono::Duration::days(age_diff * 365);

        Individual {
            id,
            given_name,
            surname,
            sex,
            birth_date,
            birth_place: person.birth_place.clone(),
            death_date: None,
            death_place: None,
            language: person.language.clone(),
            parent_family_id: None,
            spouse_family_ids: Vec::new(),
        }
    }

    fn select_given_name(&self, sex: &Sex, rng: &mut impl Rng) -> String {
        let names = match sex {
            Sex::Male => &self.ruleset.names.male_given_names,
            Sex::Female => &self.ruleset.names.female_given_names,
        };
        names[rng.gen_range(0..names.len())].clone()
    }

    fn select_surname(
        &self,
        father_id: Option<usize>,
        _mother_id: Option<usize>,
        _sex: &Sex,
        rng: &mut impl Rng,
    ) -> String {
        if self.ruleset.names.use_patronymic {
            if let Some(fid) = father_id {
                // O(1) lookup instead of O(n) search
                let father = self.individuals.get(&fid).unwrap();
                return format!("{}son", father.given_name);
            }
        }

        if !self.ruleset.names.surnames.is_empty() {
            self.ruleset.names.surnames[rng.gen_range(0..self.ruleset.names.surnames.len())].clone()
        } else {
            "Unknown".to_string()
        }
    }

    fn select_location(&self, rng: &mut impl Rng) -> String {
        let country = &self.ruleset.locations.countries[0];
        if country.cities.is_empty() {
            country.name.clone()
        } else {
            let city = &country.cities[rng.gen_range(0..country.cities.len())];
            // Check if city already contains country name (for full location strings like CSV data)
            if city.contains(&country.name) {
                city.clone()
            } else {
                format!("{}, {}", city, country.name)
            }
        }
    }

    fn select_language(&self, rng: &mut impl Rng) -> String {
        if self.ruleset.demographics.languages.is_empty() {
            "English".to_string()
        } else {
            self.ruleset.demographics.languages
                [rng.gen_range(0..self.ruleset.demographics.languages.len())]
            .clone()
        }
    }

    fn generate_birth_date(
        &self,
        father_id: Option<usize>,
        _mother_id: Option<usize>,
        rng: &mut impl Rng,
    ) -> NaiveDate {
        if let Some(fid) = father_id {
            // O(1) lookup instead of O(n) search
            if let Some(father) = self.individuals.get(&fid) {
                let parent_age = rng.gen_range(
                    self.ruleset.dates.min_parent_age..self.ruleset.dates.max_parent_age,
                );
                return father.birth_date + chrono::Duration::days(parent_age as i64 * 365);
            }
        }

        let year =
            rng.gen_range(self.ruleset.dates.birth_year_start..=self.ruleset.dates.birth_year_end);
        let month = rng.gen_range(1..=12);
        let day = rng.gen_range(1..=28);
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    fn generate_death_date(&self, birth_date: NaiveDate, rng: &mut impl Rng) -> Option<NaiveDate> {
        use rand_distr::{Distribution, Normal};

        let normal = Normal::new(
            self.ruleset.dates.life_expectancy_mean as f64,
            self.ruleset.dates.life_expectancy_stddev as f64,
        )
        .ok()?;

        let lifespan_years = normal.sample(rng).max(0.0);
        let lifespan = lifespan_years as i64;
        let current_year = chrono::Utc::now().year();
        let max_death_year = current_year - 110; // Deaths must be at least 110 years ago
        let death_year = birth_date.year() + lifespan as i32;

        if death_year > max_death_year {
            None
        } else {
            Some(birth_date + chrono::Duration::days(lifespan * 365))
        }
    }

    fn generate_marriage_date(
        &self,
        husband_birth: NaiveDate,
        wife_birth: NaiveDate,
        rng: &mut impl Rng,
    ) -> NaiveDate {
        let later_birth = if husband_birth > wife_birth {
            husband_birth
        } else {
            wife_birth
        };
        let marriage_age = rng
            .gen_range(self.ruleset.dates.min_marriage_age..=self.ruleset.dates.max_marriage_age);
        later_birth + chrono::Duration::days(marriage_age as i64 * 365)
    }

    fn generate_divorce_date(&self, marriage_date: NaiveDate, rng: &mut impl Rng) -> NaiveDate {
        let years_married = rng.gen_range(2..25);
        marriage_date + chrono::Duration::days(years_married * 365)
    }

    fn calculate_num_children(&self, rng: &mut impl Rng) -> usize {
        use rand_distr::{Distribution, Normal};

        let normal = Normal::new(
            self.ruleset.relationships.children_mean,
            self.ruleset.relationships.children_stddev,
        )
        .unwrap();

        let num = normal.sample(rng).round() as i32;
        num.clamp(
            self.ruleset.relationships.min_children as i32,
            self.ruleset.relationships.max_children as i32,
        ) as usize
    }

    /// Get a reference to the generated individuals
    pub fn individuals(&self) -> &HashMap<usize, Individual> {
        &self.individuals
    }

    /// Get a reference to the generated families
    pub fn families(&self) -> &[Family] {
        &self.families
    }

    pub fn write_gedcom<W: Write>(&self, writer: &mut BufWriter<W>) -> std::io::Result<()> {
        self.write_header(writer)?;

        // Iterate over HashMap values
        for individual in self.individuals.values() {
            self.write_individual(writer, individual)?;
        }

        for family in &self.families {
            self.write_family(writer, family)?;
        }

        self.write_trailer(writer)?;
        Ok(())
    }

    fn write_header<W: Write>(&self, writer: &mut BufWriter<W>) -> std::io::Result<()> {
        writeln!(writer, "0 HEAD")?;
        writeln!(writer, "1 SOUR Rfamily")?;
        writeln!(writer, "2 VERS 0.2.0")?;
        writeln!(writer, "2 NAME Rfamily - GEDCOM Generator with Rulesets")?;
        writeln!(writer, "1 DEST ANY")?;
        writeln!(writer, "1 DATE {}", chrono::Utc::now().format("%d %b %Y"))?;
        writeln!(writer, "1 CHAR UTF-8")?;
        writeln!(writer, "1 GEDC")?;
        writeln!(writer, "2 VERS 5.5.1")?;
        writeln!(writer, "2 FORM LINEAGE-LINKED")?;
        writeln!(
            writer,
            "1 LANG {}",
            self.ruleset
                .demographics
                .languages
                .first()
                .unwrap_or(&"English".to_string())
        )?;
        Ok(())
    }

    fn write_individual<W: Write>(
        &self,
        writer: &mut BufWriter<W>,
        indi: &Individual,
    ) -> std::io::Result<()> {
        writeln!(writer, "0 @I{}@ INDI", indi.id)?;
        writeln!(writer, "1 NAME {} /{}/", indi.given_name, indi.surname)?;
        writeln!(writer, "2 GIVN {}", indi.given_name)?;
        writeln!(writer, "2 SURN {}", indi.surname)?;

        match indi.sex {
            Sex::Male => writeln!(writer, "1 SEX M")?,
            Sex::Female => writeln!(writer, "1 SEX F")?,
        }

        writeln!(writer, "1 BIRT")?;
        writeln!(writer, "2 DATE {}", format_date(indi.birth_date))?;
        writeln!(writer, "2 PLAC {}", indi.birth_place)?;

        if let Some(death_date) = indi.death_date {
            writeln!(writer, "1 DEAT")?;
            writeln!(writer, "2 DATE {}", format_date(death_date))?;
            if let Some(ref death_place) = indi.death_place {
                writeln!(writer, "2 PLAC {}", death_place)?;
            }
        }

        if let Some(fam_id) = indi.parent_family_id {
            writeln!(writer, "1 FAMC @F{}@", fam_id)?;
        }

        for &fam_id in &indi.spouse_family_ids {
            writeln!(writer, "1 FAMS @F{}@", fam_id)?;
        }

        // LDS Ordinances
        if self.ruleset.ordinances.include_lds_ordinances {
            use rand::thread_rng;
            let mut rng = thread_rng();

            if rng.gen_bool(self.ruleset.ordinances.baptism_probability) {
                self.write_baptism(writer, indi, &mut rng)?;
            }
            if rng.gen_bool(self.ruleset.ordinances.endowment_probability) {
                self.write_endowment(writer, indi, &mut rng)?;
            }
        }

        Ok(())
    }

    fn write_baptism<W: Write>(
        &self,
        writer: &mut BufWriter<W>,
        indi: &Individual,
        rng: &mut impl Rng,
    ) -> std::io::Result<()> {
        let bapt_date = indi.birth_date + chrono::Duration::days(8 * 365); // Age 8
        let temple = &self.ruleset.ordinances.temples
            [rng.gen_range(0..self.ruleset.ordinances.temples.len())];
        writeln!(writer, "1 BAPL")?;
        writeln!(writer, "2 DATE {}", format_date(bapt_date))?;
        writeln!(writer, "2 TEMP {}", temple)?;
        writeln!(writer, "2 STAT COMPLETED")?;
        Ok(())
    }

    fn write_endowment<W: Write>(
        &self,
        writer: &mut BufWriter<W>,
        indi: &Individual,
        rng: &mut impl Rng,
    ) -> std::io::Result<()> {
        let endow_date = indi.birth_date + chrono::Duration::days(19 * 365); // Age 19
        let temple = &self.ruleset.ordinances.temples
            [rng.gen_range(0..self.ruleset.ordinances.temples.len())];
        writeln!(writer, "1 ENDL")?;
        writeln!(writer, "2 DATE {}", format_date(endow_date))?;
        writeln!(writer, "2 TEMP {}", temple)?;
        writeln!(writer, "2 STAT COMPLETED")?;
        Ok(())
    }

    fn write_family<W: Write>(
        &self,
        writer: &mut BufWriter<W>,
        fam: &Family,
    ) -> std::io::Result<()> {
        writeln!(writer, "0 @F{}@ FAM", fam.id)?;

        if let Some(husband_id) = fam.husband_id {
            writeln!(writer, "1 HUSB @I{}@", husband_id)?;
        }
        if let Some(wife_id) = fam.wife_id {
            writeln!(writer, "1 WIFE @I{}@", wife_id)?;
        }

        for &child_id in &fam.children_ids {
            writeln!(writer, "1 CHIL @I{}@", child_id)?;
        }

        if let Some(marriage_date) = fam.marriage_date {
            writeln!(writer, "1 MARR")?;
            writeln!(writer, "2 DATE {}", format_date(marriage_date))?;
            if let Some(ref place) = fam.marriage_place {
                writeln!(writer, "2 PLAC {}", place)?;
            }
        }

        if let Some(divorce_date) = fam.divorce_date {
            writeln!(writer, "1 DIV")?;
            writeln!(writer, "2 DATE {}", format_date(divorce_date))?;
        }

        Ok(())
    }

    fn write_trailer<W: Write>(&self, writer: &mut BufWriter<W>) -> std::io::Result<()> {
        writeln!(writer, "0 TRLR")?;
        Ok(())
    }
}

fn format_date(date: NaiveDate) -> String {
    format!(
        "{} {} {}",
        date.day(),
        month_name(date.month()),
        date.year()
    )
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "JAN",
        2 => "FEB",
        3 => "MAR",
        4 => "APR",
        5 => "MAY",
        6 => "JUN",
        7 => "JUL",
        8 => "AUG",
        9 => "SEP",
        10 => "OCT",
        11 => "NOV",
        12 => "DEC",
        _ => "JAN",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ruleset::Ruleset;

    #[test]
    fn test_generator_creation() {
        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset);

        assert_eq!(generator.individuals.len(), 0);
        assert_eq!(generator.families.len(), 0);
        assert_eq!(generator.next_indi_id, 1);
        assert_eq!(generator.next_fam_id, 1);
    }

    #[test]
    fn test_generate_individuals() {
        let mut ruleset = Ruleset::default_english();
        ruleset.relationships.generate_families = false; // Disable families to test exact count
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(10, &mut rng);

        assert_eq!(generator.individuals.len(), 10);
    }

    #[test]
    fn test_individual_has_required_fields() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(5, &mut rng);

        for individual in generator.individuals.values() {
            assert!(!individual.given_name.is_empty());
            assert!(!individual.surname.is_empty());
            assert!(!individual.birth_place.is_empty());
            assert!(!individual.language.is_empty());
        }
    }

    #[test]
    fn test_sex_assignment() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(100, &mut rng);

        let males = generator
            .individuals
            .values()
            .filter(|i| matches!(i.sex, Sex::Male))
            .count();
        let females = generator
            .individuals
            .values()
            .filter(|i| matches!(i.sex, Sex::Female))
            .count();

        assert!(males > 0);
        assert!(females > 0);
        assert_eq!(males + females, generator.individuals.len());
    }

    #[test]
    fn test_death_date_constraint() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(100, &mut rng);

        let current_year = chrono::Utc::now().year();
        let max_death_year = current_year - 110;

        for individual in generator.individuals.values() {
            if let Some(death_date) = individual.death_date {
                assert!(
                    death_date.year() <= max_death_year,
                    "Death date {} should be before {}",
                    death_date.year(),
                    max_death_year
                );
            }
        }
    }

    #[test]
    fn test_birth_date_in_range() {
        let mut ruleset = Ruleset::default_english();
        let birth_year_start = ruleset.dates.birth_year_start;
        let birth_year_end = ruleset.dates.birth_year_end;
        ruleset.relationships.generate_families = false; // Disable families to test initial generation only
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(50, &mut rng);

        for individual in generator.individuals.values() {
            assert!(individual.birth_date.year() >= birth_year_start);
            assert!(individual.birth_date.year() <= birth_year_end);
        }
    }

    #[test]
    fn test_family_generation() {
        let mut ruleset = Ruleset::default_english();
        ruleset.relationships.generate_families = true;
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(20, &mut rng);

        // Should have created some families
        assert!(!generator.families.is_empty());
    }

    #[test]
    fn test_family_has_parents() {
        let mut ruleset = Ruleset::default_english();
        ruleset.relationships.generate_families = true;
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(20, &mut rng);

        for family in &generator.families {
            // Most families should have both parents
            assert!(family.husband_id.is_some() || family.wife_id.is_some());
        }
    }

    #[test]
    fn test_format_date() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let formatted = format_date(date);

        assert_eq!(formatted, "15 MAR 2024");
    }

    #[test]
    fn test_month_name() {
        assert_eq!(month_name(1), "JAN");
        assert_eq!(month_name(6), "JUN");
        assert_eq!(month_name(12), "DEC");
        assert_eq!(month_name(0), "JAN"); // Invalid month defaults to JAN
        assert_eq!(month_name(13), "JAN"); // Invalid month defaults to JAN
    }

    #[test]
    fn test_gedcom_output_format() {
        use std::io::Cursor;

        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(5, &mut rng);

        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = BufWriter::new(cursor);

            generator.write_gedcom(&mut writer).unwrap();
            writer.flush().unwrap();
        }

        let output = String::from_utf8(buffer).unwrap();

        // Check GEDCOM structure
        assert!(output.contains("0 HEAD"));
        assert!(output.contains("1 SOUR Rfamily"));
        assert!(output.contains("0 TRLR"));
        assert!(output.contains("INDI"));
    }

    #[test]
    fn test_spanish_names_in_spanish_ruleset() {
        let ruleset = Ruleset::default_spanish();
        let mut generator = GedcomGenerator::new(ruleset.clone());
        let mut rng = rand::thread_rng();

        generator.generate(10, &mut rng);

        // Check that generated individuals use Spanish names
        let has_spanish_name = generator.individuals.values().any(|i| {
            ruleset.names.male_given_names.contains(&i.given_name)
                || ruleset.names.female_given_names.contains(&i.given_name)
        });

        assert!(has_spanish_name);
    }

    #[test]
    fn test_lds_ordinances_when_enabled() {
        use std::io::Cursor;

        let ruleset = Ruleset::default_lds();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(10, &mut rng);

        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = BufWriter::new(cursor);

            generator.write_gedcom(&mut writer).unwrap();
            writer.flush().unwrap();
        }

        let output = String::from_utf8(buffer).unwrap();

        // Should contain LDS ordinance tags
        assert!(output.contains("BAPL") || output.contains("ENDL"));
    }

    #[test]
    fn test_no_lds_ordinances_when_disabled() {
        use std::io::Cursor;

        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(10, &mut rng);

        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = BufWriter::new(cursor);

            generator.write_gedcom(&mut writer).unwrap();
            writer.flush().unwrap();
        }

        let output = String::from_utf8(buffer).unwrap();

        // Should NOT contain LDS ordinance tags
        assert!(!output.contains("BAPL"));
        assert!(!output.contains("ENDL"));
    }

    #[test]
    fn test_select_given_name_male() {
        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset.clone());
        let mut rng = rand::thread_rng();

        let name = generator.select_given_name(&Sex::Male, &mut rng);

        assert!(
            ruleset.names.male_given_names.contains(&name),
            "Generated male name '{}' should be in male names list",
            name
        );
    }

    #[test]
    fn test_select_given_name_female() {
        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset.clone());
        let mut rng = rand::thread_rng();

        let name = generator.select_given_name(&Sex::Female, &mut rng);

        assert!(
            ruleset.names.female_given_names.contains(&name),
            "Generated female name '{}' should be in female names list",
            name
        );
    }

    #[test]
    fn test_select_surname_without_parents() {
        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset.clone());
        let mut rng = rand::thread_rng();

        let surname = generator.select_surname(None, None, &Sex::Male, &mut rng);

        assert!(
            ruleset.names.surnames.contains(&surname),
            "Generated surname '{}' should be in surnames list",
            surname
        );
    }

    #[test]
    fn test_select_surname_inherits_from_father() {
        // Use Icelandic ruleset which uses patronymic naming
        let ruleset = Ruleset::default_icelandic();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        // Create a father
        let father = generator.create_individual(None, None, &mut rng);
        let father_given_name = father.given_name.clone();
        let father_id = father.id;
        generator.individuals.insert(father_id, father);

        // Create child with father (using patronymic system)
        let surname = generator.select_surname(Some(father_id), None, &Sex::Male, &mut rng);

        // Should be patronymic (father's given name + "son")
        assert_eq!(
            surname,
            format!("{}son", father_given_name),
            "Child should inherit patronymic from father"
        );
    }

    #[test]
    fn test_select_location() {
        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset.clone());
        let mut rng = rand::thread_rng();

        let location = generator.select_location(&mut rng);

        // Location should be from one of the countries
        let found = ruleset
            .locations
            .countries
            .iter()
            .any(|country| country.cities.iter().any(|city| location.contains(city)));

        assert!(
            found,
            "Location '{}' should be from configured countries",
            location
        );
    }

    #[test]
    fn test_select_language() {
        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset.clone());
        let mut rng = rand::thread_rng();

        let language = generator.select_language(&mut rng);

        assert!(
            ruleset.demographics.languages.contains(&language),
            "Language '{}' should be in languages list",
            language
        );
    }

    #[test]
    fn test_generate_birth_date_in_range() {
        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset.clone());
        let mut rng = rand::thread_rng();

        let birth_date = generator.generate_birth_date(None, None, &mut rng);

        assert!(
            birth_date.year() >= ruleset.dates.birth_year_start,
            "Birth year {} should be >= {}",
            birth_date.year(),
            ruleset.dates.birth_year_start
        );
        assert!(
            birth_date.year() <= ruleset.dates.birth_year_end,
            "Birth year {} should be <= {}",
            birth_date.year(),
            ruleset.dates.birth_year_end
        );
    }

    #[test]
    fn test_generate_birth_date_child_after_parent() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset.clone());
        let mut rng = rand::thread_rng();

        // Create a parent
        let parent = generator.create_individual(None, None, &mut rng);
        let parent_birth = parent.birth_date;
        let parent_id = parent.id;
        generator.individuals.insert(parent_id, parent);

        // Generate child birth date
        let child_birth = generator.generate_birth_date(Some(parent_id), None, &mut rng);

        assert!(
            child_birth.year() >= parent_birth.year() + ruleset.dates.min_parent_age,
            "Child birth year should be at least {} years after parent",
            ruleset.dates.min_parent_age
        );
    }

    #[test]
    fn test_generate_death_date_after_birth() {
        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        let birth_date = NaiveDate::from_ymd_opt(1950, 5, 15).unwrap();

        if let Some(death_date) = generator.generate_death_date(birth_date, &mut rng) {
            assert!(
                death_date > birth_date,
                "Death date should be after birth date"
            );
        }
    }

    #[test]
    fn test_generate_marriage_date_after_births() {
        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset.clone());
        let mut rng = rand::thread_rng();

        let husband_birth = NaiveDate::from_ymd_opt(1980, 1, 1).unwrap();
        let wife_birth = NaiveDate::from_ymd_opt(1982, 6, 15).unwrap();

        let marriage_date = generator.generate_marriage_date(husband_birth, wife_birth, &mut rng);

        let husband_age = marriage_date.year() - husband_birth.year();
        let wife_age = marriage_date.year() - wife_birth.year();

        assert!(
            husband_age >= ruleset.dates.min_marriage_age,
            "Husband should be at least {} at marriage",
            ruleset.dates.min_marriage_age
        );
        assert!(
            wife_age >= ruleset.dates.min_marriage_age,
            "Wife should be at least {} at marriage",
            ruleset.dates.min_marriage_age
        );
    }

    #[test]
    fn test_generate_divorce_date_after_marriage() {
        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        let marriage_date = NaiveDate::from_ymd_opt(2000, 6, 15).unwrap();
        let divorce_date = generator.generate_divorce_date(marriage_date, &mut rng);

        assert!(
            divorce_date > marriage_date,
            "Divorce date should be after marriage date"
        );
    }

    #[test]
    fn test_calculate_num_children_in_range() {
        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset.clone());
        let mut rng = rand::thread_rng();

        // Test multiple times to ensure range
        for _ in 0..10 {
            let num_children = generator.calculate_num_children(&mut rng);

            assert!(
                num_children >= ruleset.relationships.min_children,
                "Number of children should be >= {}",
                ruleset.relationships.min_children
            );
            assert!(
                num_children <= ruleset.relationships.max_children,
                "Number of children should be <= {}",
                ruleset.relationships.max_children
            );
        }
    }

    #[test]
    fn test_create_spouse_opposite_sex() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        // Create a male individual
        let mut male = generator.create_individual(None, None, &mut rng);
        male.sex = Sex::Male;

        let spouse = generator.create_spouse(&male, &mut rng);

        // Spouse should be female
        assert!(
            matches!(spouse.sex, Sex::Female),
            "Spouse of male should be female"
        );
    }

    #[test]
    fn test_create_spouse_similar_age() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        let person = generator.create_individual(None, None, &mut rng);
        let person_birth_year = person.birth_date.year();

        let spouse = generator.create_spouse(&person, &mut rng);
        let spouse_birth_year = spouse.birth_date.year();

        let age_diff = (person_birth_year - spouse_birth_year).abs();

        // Spouses should be within 20 years of each other
        assert!(
            age_diff <= 20,
            "Spouses should have similar ages, got {} year difference",
            age_diff
        );
    }

    #[test]
    fn test_format_date_normal() {
        let date = NaiveDate::from_ymd_opt(1985, 7, 4).unwrap();
        let formatted = format_date(date);

        assert_eq!(formatted, "4 JUL 1985");
    }

    #[test]
    fn test_format_date_single_digit_day() {
        let date = NaiveDate::from_ymd_opt(2000, 1, 5).unwrap();
        let formatted = format_date(date);

        assert_eq!(formatted, "5 JAN 2000");
    }

    #[test]
    fn test_month_name_all_months() {
        let expected = [
            "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
        ];

        for (i, expected_name) in expected.iter().enumerate() {
            let month = (i + 1) as u32;
            assert_eq!(
                month_name(month),
                *expected_name,
                "Month {} should be {}",
                month,
                expected_name
            );
        }
    }

    #[test]
    fn test_individual_has_parent_family() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        // Create a family first
        let family_id = generator.create_family(None, &mut rng);

        // Children should have parent_family_id set
        let has_children_with_family = generator
            .individuals
            .values()
            .any(|i| i.parent_family_id == Some(family_id));

        assert!(
            has_children_with_family,
            "Children should have parent_family_id set"
        );
    }

    #[test]
    fn test_individual_has_spouse_family() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        // Create a family
        generator.create_family(None, &mut rng);

        // Parents should have spouse_family_ids
        let has_spouse_family = generator
            .individuals
            .values()
            .any(|i| !i.spouse_family_ids.is_empty());

        assert!(
            has_spouse_family,
            "Married individuals should have spouse_family_ids"
        );
    }

    #[test]
    fn test_family_has_children() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        // Create multiple families to ensure at least one has children (probabilistic)
        for _ in 0..10 {
            generator.create_family(None, &mut rng);
        }

        // At least one family should have children
        let has_children = generator
            .families
            .iter()
            .any(|f| !f.children_ids.is_empty());

        assert!(has_children, "At least one family should have children");
    }

    #[test]
    fn test_family_has_both_parents() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.create_family(None, &mut rng);

        // Family should have both husband and wife
        let has_complete_family = generator
            .families
            .iter()
            .any(|f| f.husband_id.is_some() && f.wife_id.is_some());

        assert!(
            has_complete_family,
            "Families should have both husband and wife"
        );
    }

    #[test]
    fn test_generator_increments_ids() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        let initial_indi_id = generator.next_indi_id;
        let initial_fam_id = generator.next_fam_id;

        generator.create_individual(None, None, &mut rng);
        generator.create_family(None, &mut rng);

        assert!(
            generator.next_indi_id > initial_indi_id,
            "Individual ID should increment"
        );
        assert!(
            generator.next_fam_id > initial_fam_id,
            "Family ID should increment"
        );
    }

    #[test]
    fn test_generate_with_zero_count() {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(0, &mut rng);

        assert_eq!(
            generator.individuals.len(),
            0,
            "Should generate 0 individuals"
        );
    }

    #[test]
    fn test_generate_exact_count_without_families() {
        let mut ruleset = Ruleset::default_english();
        ruleset.relationships.generate_families = false;

        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(25, &mut rng);

        assert_eq!(
            generator.individuals.len(),
            25,
            "Should generate exactly 25 individuals"
        );
    }

    #[test]
    fn test_write_gedcom_header_contains_version() {
        use std::io::Cursor;

        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset);

        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = BufWriter::new(cursor);

            generator.write_gedcom(&mut writer).unwrap();
            writer.flush().unwrap();
        }

        let output = String::from_utf8(buffer).unwrap();

        assert!(
            output.contains("2 VERS 5.5.1"),
            "Should contain GEDCOM version 5.5.1"
        );
    }

    #[test]
    fn test_write_gedcom_header_contains_charset() {
        use std::io::Cursor;

        let ruleset = Ruleset::default_english();
        let generator = GedcomGenerator::new(ruleset);

        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = BufWriter::new(cursor);

            generator.write_gedcom(&mut writer).unwrap();
            writer.flush().unwrap();
        }

        let output = String::from_utf8(buffer).unwrap();

        assert!(
            output.contains("1 CHAR UTF-8"),
            "Should specify UTF-8 charset"
        );
    }

    #[test]
    fn test_write_gedcom_individual_has_name() {
        use std::io::Cursor;

        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(1, &mut rng);

        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = BufWriter::new(cursor);

            generator.write_gedcom(&mut writer).unwrap();
            writer.flush().unwrap();
        }

        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("1 NAME"), "Individual should have NAME tag");
    }

    #[test]
    fn test_write_gedcom_individual_has_sex() {
        use std::io::Cursor;

        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(1, &mut rng);

        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = BufWriter::new(cursor);

            generator.write_gedcom(&mut writer).unwrap();
            writer.flush().unwrap();
        }

        let output = String::from_utf8(buffer).unwrap();

        assert!(
            output.contains("1 SEX M") || output.contains("1 SEX F"),
            "Individual should have SEX tag"
        );
    }

    #[test]
    fn test_write_gedcom_individual_has_birth() {
        use std::io::Cursor;

        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        generator.generate(1, &mut rng);

        let mut buffer = Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut writer = BufWriter::new(cursor);

            generator.write_gedcom(&mut writer).unwrap();
            writer.flush().unwrap();
        }

        let output = String::from_utf8(buffer).unwrap();

        assert!(output.contains("1 BIRT"), "Individual should have BIRT tag");
        assert!(output.contains("2 DATE"), "Birth should have DATE");
        assert!(output.contains("2 PLAC"), "Birth should have PLAC");
    }
}
