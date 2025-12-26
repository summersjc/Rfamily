use crate::generator::{Family, GedcomGenerator, Individual, Sex};
use crate::ruleset::Ruleset;
use chrono::NaiveDate;
use rand::Rng;
use rand_distr::{Distribution, Poisson};

/// Configuration for IOUS (Individual of Unusual Size) generation
#[derive(Debug, Clone)]
pub struct IOUSConfig {
    /// Number of marriages for the IOUS individual
    pub marriages: usize,
    /// Mean number of children per marriage
    pub children_per_marriage_mean: f64,
    /// Number of siblings for the IOUS
    pub siblings: usize,
    /// Number of generations of descendants to generate
    pub descendant_generations: usize,
    /// Target total number of descendants (approximate)
    pub target_descendants: Option<usize>,
}

impl Default for IOUSConfig {
    fn default() -> Self {
        IOUSConfig {
            marriages: 3,
            children_per_marriage_mean: 4.0,
            siblings: 5,
            descendant_generations: 5,
            target_descendants: None,
        }
    }
}

/// IOUS Generator - Creates an Individual of Unusual Size
/// A highly connected person with multiple marriages, many children, and extensive descendants
pub struct IOUSGenerator {
    generator: GedcomGenerator,
    config: IOUSConfig,
    individuals_generated: usize,
}

impl IOUSGenerator {
    pub fn new(ruleset: Ruleset, config: IOUSConfig) -> Self {
        IOUSGenerator {
            generator: GedcomGenerator::new(ruleset),
            config,
            individuals_generated: 0,
        }
    }

    /// Generate the complete IOUS family tree
    pub fn generate(&mut self, rng: &mut impl Rng) -> usize {
        // Create the central IOUS individual
        let ious = self.create_ious_individual(rng);
        let ious_id = ious.id;
        self.generator.individuals.insert(ious_id, ious);
        self.individuals_generated += 1;

        // Generate siblings for the IOUS
        self.generate_siblings(ious_id, rng);

        // Generate marriages and descendants
        self.generate_marriages_and_descendants(ious_id, rng);

        self.individuals_generated
    }

    /// Create the central IOUS individual
    fn create_ious_individual(&mut self, rng: &mut impl Rng) -> Individual {
        let id = self.generator.next_indi_id;
        self.generator.next_indi_id += 1;

        let sex = if rng.gen_bool(0.5) {
            Sex::Male
        } else {
            Sex::Female
        };

        let given_name = self.select_given_name(&sex, rng);
        let surname = self.select_surname(rng);

        // IOUS is usually from an earlier generation (born 1920-1950 for more descendants)
        let birth_year = rng.gen_range(1920..=1950);
        let birth_date =
            NaiveDate::from_ymd_opt(birth_year, rng.gen_range(1..=12), rng.gen_range(1..=28))
                .unwrap();

        let birth_place = self.select_location(rng);
        let language = self.select_language(rng);

        Individual {
            id,
            given_name,
            surname,
            sex,
            birth_date,
            birth_place,
            death_date: None,
            death_place: None,
            language,
            parent_family_id: None,
            spouse_family_ids: Vec::new(),
        }
    }

    /// Generate siblings for the IOUS
    fn generate_siblings(&mut self, ious_id: usize, rng: &mut impl Rng) {
        let ious = self.generator.individuals.get(&ious_id).unwrap().clone();

        // Create parent family for IOUS and siblings
        let family_id = self.generator.next_fam_id;
        self.generator.next_fam_id += 1;

        let mut children_ids = vec![ious_id];

        // Generate siblings
        for _ in 0..self.config.siblings {
            if let Some(target) = self.config.target_descendants {
                if self.individuals_generated >= target {
                    break;
                }
            }

            let mut sibling = self.create_sibling(&ious, rng);
            let sibling_id = sibling.id;

            sibling.parent_family_id = Some(family_id);
            self.generator.individuals.insert(sibling_id, sibling);
            children_ids.push(sibling_id);
            self.individuals_generated += 1;
        }

        // Update IOUS with parent family
        if let Some(ious_mut) = self.generator.individuals.get_mut(&ious_id) {
            ious_mut.parent_family_id = Some(family_id);
        }

        // Create a family record for the parents (we won't detail the parents themselves)
        let family = Family {
            id: family_id,
            husband_id: None,
            wife_id: None,
            children_ids,
            marriage_date: None,
            marriage_place: None,
            divorce_date: None,
        };

        self.generator.families.push(family);
    }

    /// Generate marriages and descendants for the IOUS
    fn generate_marriages_and_descendants(&mut self, ious_id: usize, rng: &mut impl Rng) {
        for marriage_num in 0..self.config.marriages {
            if let Some(target) = self.config.target_descendants {
                if self.individuals_generated >= target {
                    break;
                }
            }

            let family_id = self.create_marriage(ious_id, marriage_num, rng);

            // Generate descendants from this marriage
            self.generate_descendants_recursive(family_id, 1, rng);
        }
    }

    /// Create a marriage for the IOUS
    fn create_marriage(
        &mut self,
        person_id: usize,
        marriage_num: usize,
        rng: &mut impl Rng,
    ) -> usize {
        let family_id = self.generator.next_fam_id;
        self.generator.next_fam_id += 1;

        let person = self.generator.individuals.get(&person_id).unwrap().clone();

        // Create spouse
        let spouse = self.create_spouse(&person, marriage_num, rng);
        let spouse_id = spouse.id;
        self.generator.individuals.insert(spouse_id, spouse);
        self.individuals_generated += 1;

        // Determine husband and wife
        let (husband_id, wife_id) = match person.sex {
            Sex::Male => (person_id, spouse_id),
            Sex::Female => (spouse_id, person_id),
        };

        // Update spouse family references
        if let Some(p) = self.generator.individuals.get_mut(&person_id) {
            p.spouse_family_ids.push(family_id);
        }
        if let Some(s) = self.generator.individuals.get_mut(&spouse_id) {
            s.spouse_family_ids.push(family_id);
        }

        // Calculate marriage date
        let person_birth = self
            .generator
            .individuals
            .get(&person_id)
            .unwrap()
            .birth_date;
        let spouse_birth = self
            .generator
            .individuals
            .get(&spouse_id)
            .unwrap()
            .birth_date;
        let marriage_date =
            self.calculate_marriage_date(person_birth, spouse_birth, marriage_num, rng);
        let marriage_place = self.select_location(rng);

        // Generate children for this marriage
        let num_children = self.calculate_children_count(rng);
        let mut children_ids = Vec::new();

        for child_num in 0..num_children {
            if let Some(target) = self.config.target_descendants {
                if self.individuals_generated >= target {
                    break;
                }
            }

            let child = self.create_child(
                husband_id,
                wife_id,
                family_id,
                child_num,
                marriage_date,
                rng,
            );
            let child_id = child.id;
            children_ids.push(child_id);
            self.generator.individuals.insert(child_id, child);
            self.individuals_generated += 1;
        }

        // Create family record
        let family = Family {
            id: family_id,
            husband_id: Some(husband_id),
            wife_id: Some(wife_id),
            children_ids,
            marriage_date: Some(marriage_date),
            marriage_place: Some(marriage_place),
            divorce_date: None,
        };

        self.generator.families.push(family);
        family_id
    }

    /// Recursively generate descendants
    fn generate_descendants_recursive(
        &mut self,
        parent_family_id: usize,
        generation: usize,
        rng: &mut impl Rng,
    ) {
        if generation >= self.config.descendant_generations {
            return;
        }

        if let Some(target) = self.config.target_descendants {
            if self.individuals_generated >= target {
                return;
            }
        }

        // Get children from this family
        let family = self
            .generator
            .families
            .iter()
            .find(|f| f.id == parent_family_id)
            .cloned();

        if let Some(fam) = family {
            let children = fam.children_ids.clone();

            for child_id in children {
                if let Some(target) = self.config.target_descendants {
                    if self.individuals_generated >= target {
                        return;
                    }
                }

                // Probabilistically create marriages for descendants
                if rng.gen_bool(0.75) {
                    // 75% of descendants marry
                    let new_family_id = self.create_marriage(child_id, 0, rng);
                    self.generate_descendants_recursive(new_family_id, generation + 1, rng);
                }
            }
        }
    }

    /// Create a sibling with similar characteristics to the IOUS
    fn create_sibling(&mut self, ious: &Individual, rng: &mut impl Rng) -> Individual {
        let id = self.generator.next_indi_id;
        self.generator.next_indi_id += 1;

        let sex = if rng.gen_bool(0.5) {
            Sex::Male
        } else {
            Sex::Female
        };

        let given_name = self.select_given_name(&sex, rng);
        let surname = ious.surname.clone();

        // Siblings are within 15 years of IOUS
        let age_diff = rng.gen_range(-7..=7);
        let birth_date = ious.birth_date + chrono::Duration::days(age_diff * 365);

        Individual {
            id,
            given_name,
            surname,
            sex,
            birth_date,
            birth_place: ious.birth_place.clone(),
            death_date: None,
            death_place: None,
            language: ious.language.clone(),
            parent_family_id: None,
            spouse_family_ids: Vec::new(),
        }
    }

    /// Create a spouse for a person
    fn create_spouse(
        &mut self,
        person: &Individual,
        _marriage_num: usize,
        rng: &mut impl Rng,
    ) -> Individual {
        let id = self.generator.next_indi_id;
        self.generator.next_indi_id += 1;

        let sex = match person.sex {
            Sex::Male => Sex::Female,
            Sex::Female => Sex::Male,
        };

        let given_name = self.select_given_name(&sex, rng);
        let surname = self.select_surname(rng);

        // Spouse is within 10 years of person's age
        let age_diff = rng.gen_range(-5..=5);
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

    /// Create a child
    fn create_child(
        &mut self,
        father_id: usize,
        _mother_id: usize,
        family_id: usize,
        child_num: usize,
        marriage_date: NaiveDate,
        rng: &mut impl Rng,
    ) -> Individual {
        let id = self.generator.next_indi_id;
        self.generator.next_indi_id += 1;

        let sex = if rng.gen_bool(0.5) {
            Sex::Male
        } else {
            Sex::Female
        };

        let given_name = self.select_given_name(&sex, rng);

        // Child inherits father's surname
        let father = self.generator.individuals.get(&father_id).unwrap();
        let surname = father.surname.clone();

        // Children born 1-3 years apart, starting 1-2 years after marriage
        let years_after_marriage = 1 + (child_num * 2);
        let birth_date =
            marriage_date + chrono::Duration::days((years_after_marriage * 365) as i64);

        let birth_place = self.select_location(rng);
        let language = self.select_language(rng);

        Individual {
            id,
            given_name,
            surname,
            sex,
            birth_date,
            birth_place,
            death_date: None,
            death_place: None,
            language,
            parent_family_id: Some(family_id),
            spouse_family_ids: Vec::new(),
        }
    }

    /// Calculate marriage date based on person's birth and marriage number
    fn calculate_marriage_date(
        &self,
        person_birth: NaiveDate,
        spouse_birth: NaiveDate,
        marriage_num: usize,
        rng: &mut impl Rng,
    ) -> NaiveDate {
        let later_birth = if person_birth > spouse_birth {
            person_birth
        } else {
            spouse_birth
        };

        // First marriage at age 20-30, subsequent marriages 5-10 years later
        let base_age = if marriage_num == 0 {
            rng.gen_range(20..=30)
        } else {
            25 + (marriage_num * 8)
        };

        later_birth + chrono::Duration::days(base_age as i64 * 365)
    }

    /// Calculate number of children using Poisson distribution
    fn calculate_children_count(&self, rng: &mut impl Rng) -> usize {
        let poisson = Poisson::new(self.config.children_per_marriage_mean).unwrap();
        let count = poisson.sample(rng).round() as usize;
        count.clamp(1, 10) // At least 1, at most 10 children per marriage
    }

    // Helper methods using generator's ruleset
    fn select_given_name(&self, sex: &Sex, rng: &mut impl Rng) -> String {
        let names = match sex {
            Sex::Male => &self.generator.ruleset.names.male_given_names,
            Sex::Female => &self.generator.ruleset.names.female_given_names,
        };
        names[rng.gen_range(0..names.len())].clone()
    }

    fn select_surname(&self, rng: &mut impl Rng) -> String {
        let surnames = &self.generator.ruleset.names.surnames;
        if surnames.is_empty() {
            "Smith".to_string()
        } else {
            surnames[rng.gen_range(0..surnames.len())].clone()
        }
    }

    fn select_location(&self, rng: &mut impl Rng) -> String {
        let country = &self.generator.ruleset.locations.countries[0];
        if country.cities.is_empty() {
            country.name.clone()
        } else {
            let city = &country.cities[rng.gen_range(0..country.cities.len())];
            if city.contains(&country.name) {
                city.clone()
            } else {
                format!("{}, {}", city, country.name)
            }
        }
    }

    fn select_language(&self, rng: &mut impl Rng) -> String {
        if self.generator.ruleset.demographics.languages.is_empty() {
            "English".to_string()
        } else {
            self.generator.ruleset.demographics.languages
                [rng.gen_range(0..self.generator.ruleset.demographics.languages.len())]
            .clone()
        }
    }

    /// Get the underlying generator to write GEDCOM output
    pub fn into_generator(self) -> GedcomGenerator {
        self.generator
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ruleset::Ruleset;

    #[test]
    fn test_ious_generator_creation() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig::default();
        let generator = IOUSGenerator::new(ruleset, config);

        assert_eq!(generator.individuals_generated, 0);
    }

    #[test]
    fn test_ious_config_default() {
        let config = IOUSConfig::default();

        assert_eq!(config.marriages, 3);
        assert_eq!(config.siblings, 5);
        assert_eq!(config.descendant_generations, 5);
    }

    #[test]
    fn test_ious_generation() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 2,
            children_per_marriage_mean: 3.0,
            siblings: 3,
            descendant_generations: 2,
            target_descendants: Some(50),
        };

        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        let count = generator.generate(&mut rng);

        assert!(count > 0);
        assert!(count <= 50);
    }
}
