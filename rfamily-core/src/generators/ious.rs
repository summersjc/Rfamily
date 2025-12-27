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

    // Family structure validation tests
    #[test]
    fn test_ious_has_siblings() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 1,
            children_per_marriage_mean: 2.0,
            siblings: 3,
            descendant_generations: 1,
            target_descendants: None,
        };

        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        generator.generate(&mut rng);

        // IOUS should have siblings + IOUS itself in parent family
        let families_with_multiple_children: Vec<_> = generator
            .generator
            .families
            .iter()
            .filter(|f| f.children_ids.len() >= 2)
            .collect();

        assert!(
            !families_with_multiple_children.is_empty(),
            "Should have family with IOUS and siblings"
        );

        // Find the IOUS parent family (the one without husband/wife, or with most children)
        let parent_family = generator
            .generator
            .families
            .iter()
            .max_by_key(|f| f.children_ids.len());

        assert!(parent_family.is_some());
        let parent_family = parent_family.unwrap();
        assert!(
            parent_family.children_ids.len() >= 2,
            "Parent family should have IOUS + siblings"
        );
    }

    #[test]
    fn test_ious_multiple_marriages() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 3,
            children_per_marriage_mean: 2.0,
            siblings: 0,
            descendant_generations: 1,
            target_descendants: None,
        };

        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        generator.generate(&mut rng);

        // Count families where the IOUS is a spouse
        // The IOUS is the first generated individual (next_indi_id starts at 1)
        let ious_id = 1;
        let ious_spouse_families: Vec<_> = generator
            .generator
            .families
            .iter()
            .filter(|f| f.husband_id == Some(ious_id) || f.wife_id == Some(ious_id))
            .collect();

        assert!(
            ious_spouse_families.len() >= 2,
            "IOUS should have at least 2 marriages (expected 3, got {})",
            ious_spouse_families.len()
        );

        // Verify each marriage has different spouses
        let mut spouses = std::collections::HashSet::new();
        for family in &ious_spouse_families {
            if let Some(husband_id) = family.husband_id {
                if husband_id != ious_id {
                    spouses.insert(husband_id);
                }
            }
            if let Some(wife_id) = family.wife_id {
                if wife_id != ious_id {
                    spouses.insert(wife_id);
                }
            }
        }

        assert!(
            spouses.len() >= 2,
            "IOUS should have multiple different spouses"
        );
    }

    #[test]
    fn test_ious_has_children() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 2,
            children_per_marriage_mean: 3.0,
            siblings: 0,
            descendant_generations: 1,
            target_descendants: None,
        };

        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        generator.generate(&mut rng);

        // IOUS marriages should have children
        let ious_id = 1;
        let families_with_ious: Vec<_> = generator
            .generator
            .families
            .iter()
            .filter(|f| {
                (f.husband_id == Some(ious_id) || f.wife_id == Some(ious_id))
                    && !f.children_ids.is_empty()
            })
            .collect();

        assert!(
            !families_with_ious.is_empty(),
            "IOUS should have at least one family with children"
        );

        // Verify children have parent_family_id set correctly
        for family in families_with_ious {
            for child_id in &family.children_ids {
                let child = generator.generator.individuals.get(child_id).unwrap();
                assert_eq!(
                    child.parent_family_id,
                    Some(family.id),
                    "Child should reference parent family"
                );
            }
        }
    }

    #[test]
    fn test_ious_descendant_generations() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 2,
            children_per_marriage_mean: 2.0,
            siblings: 0,
            descendant_generations: 3,
            target_descendants: None,
        };

        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        generator.generate(&mut rng);

        // With 3 generations and 2 children per marriage on average,
        // we should have multiple levels
        let total_families = generator.generator.families.len();
        assert!(
            total_families >= 3,
            "With 3 descendant generations, should have multiple families (got {})",
            total_families
        );

        // Check that some children become parents
        let mut children_who_are_parents = 0;
        for individual in generator.generator.individuals.values() {
            if !individual.spouse_family_ids.is_empty() && individual.parent_family_id.is_some() {
                children_who_are_parents += 1;
            }
        }

        assert!(
            children_who_are_parents > 0,
            "Some children should become parents (descendant generations)"
        );
    }

    #[test]
    fn test_ious_target_limit_respected() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 5,
            children_per_marriage_mean: 10.0,
            siblings: 10,
            descendant_generations: 5,
            target_descendants: Some(30),
        };

        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        let count = generator.generate(&mut rng);

        assert!(
            count <= 30,
            "Generated count should respect target limit (got {})",
            count
        );
        assert!(
            generator.generator.individuals.len() <= 30,
            "Individual count should respect target limit"
        );
    }

    #[test]
    fn test_ious_marriage_dates_sequential() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 2,
            children_per_marriage_mean: 2.0,
            siblings: 0,
            descendant_generations: 1,
            target_descendants: None,
        };

        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        generator.generate(&mut rng);

        // Get IOUS marriages and check dates are ordered
        let ious_id = 1;
        let mut marriage_dates: Vec<_> = generator
            .generator
            .families
            .iter()
            .filter(|f| f.husband_id == Some(ious_id) || f.wife_id == Some(ious_id))
            .filter_map(|f| f.marriage_date)
            .collect();

        marriage_dates.sort();

        // Dates should be different (subsequent marriages happen later)
        if marriage_dates.len() >= 2 {
            assert_ne!(
                marriage_dates[0], marriage_dates[1],
                "Multiple marriages should have different dates"
            );
        }
    }

    #[test]
    fn test_ious_children_have_birth_dates() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 1,
            children_per_marriage_mean: 3.0,
            siblings: 0,
            descendant_generations: 1,
            target_descendants: None,
        };

        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        generator.generate(&mut rng);

        // Find children (individuals with parent_family_id)
        let children: Vec<_> = generator
            .generator
            .individuals
            .values()
            .filter(|i| i.parent_family_id.is_some())
            .collect();

        assert!(!children.is_empty(), "Should have generated children");

        // All children should have birth dates (chrono::NaiveDate is always valid)
        // Just verify the field exists and is reasonable
        for child in children {
            // Birth date should be after 1900 and before 2100
            let year_string = child.birth_date.format("%Y").to_string();
            let year: i32 = year_string.parse().unwrap();
            assert!(
                year > 1900 && year < 2100,
                "Child should have valid birth year: {}",
                year
            );
        }
    }

    #[test]
    fn test_ious_output_is_valid_structure() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 2,
            children_per_marriage_mean: 2.0,
            siblings: 2,
            descendant_generations: 2,
            target_descendants: Some(30),
        };

        let mut ious_generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        ious_generator.generate(&mut rng);

        let generator = ious_generator.into_generator();

        // Verify basic structure validity
        assert!(!generator.individuals.is_empty(), "Should have individuals");
        assert!(!generator.families.is_empty(), "Should have families");

        // Verify all family references are valid
        for family in &generator.families {
            if let Some(husband_id) = family.husband_id {
                assert!(
                    generator.individuals.contains_key(&husband_id),
                    "Husband reference should be valid"
                );
            }
            if let Some(wife_id) = family.wife_id {
                assert!(
                    generator.individuals.contains_key(&wife_id),
                    "Wife reference should be valid"
                );
            }
            for child_id in &family.children_ids {
                assert!(
                    generator.individuals.contains_key(child_id),
                    "Child reference should be valid"
                );
            }
        }

        // Verify individual family references are valid
        for individual in generator.individuals.values() {
            if let Some(parent_family_id) = individual.parent_family_id {
                assert!(
                    generator.families.iter().any(|f| f.id == parent_family_id),
                    "Parent family reference should be valid"
                );
            }
            for spouse_family_id in &individual.spouse_family_ids {
                assert!(
                    generator.families.iter().any(|f| f.id == *spouse_family_id),
                    "Spouse family reference should be valid"
                );
            }
        }
    }

    #[test]
    fn test_ious_minimal_config() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 1,
            children_per_marriage_mean: 1.0,
            siblings: 0,
            descendant_generations: 1,
            target_descendants: None,
        };

        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        let count = generator.generate(&mut rng);

        // Should generate at least IOUS + spouse + 1 child
        assert!(count >= 3, "Should generate at least 3 individuals");
    }

    #[test]
    fn test_ious_zero_siblings() {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 1,
            children_per_marriage_mean: 2.0,
            siblings: 0,
            descendant_generations: 1,
            target_descendants: None,
        };

        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        generator.generate(&mut rng);

        // Count individuals in IOUS parent family
        // Should be exactly 1 (just IOUS)
        let ious_id = 1;
        let ious = generator.generator.individuals.get(&ious_id).unwrap();

        if let Some(parent_family_id) = ious.parent_family_id {
            let parent_family = generator
                .generator
                .families
                .iter()
                .find(|f| f.id == parent_family_id)
                .unwrap();

            assert_eq!(
                parent_family.children_ids.len(),
                1,
                "With 0 siblings, parent family should have only IOUS"
            );
        }
    }
}
