use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ruleset {
    pub names: NameRules,
    pub dates: DateRules,
    pub locations: LocationRules,
    pub demographics: DemographicRules,
    pub relationships: RelationshipRules,
    pub ordinances: OrdinanceRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameRules {
    pub male_given_names: Vec<String>,
    pub female_given_names: Vec<String>,
    pub surnames: Vec<String>,
    pub use_patronymic: bool,
    pub use_matronymic: bool,
    pub name_format: NameFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NameFormat {
    WesternStyle,   // Given Surname
    EasternStyle,   // Surname Given
    Patronymic,     // Given Patronymic
    IcelandicStyle, // Given Patronymic/Matronymic
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRules {
    pub birth_year_start: i32,
    pub birth_year_end: i32,
    pub min_marriage_age: i32,
    pub max_marriage_age: i32,
    pub min_parent_age: i32,
    pub max_parent_age: i32,
    pub life_expectancy_mean: i32,
    pub life_expectancy_stddev: i32,
    pub include_death_dates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationRules {
    pub countries: Vec<Country>,
    pub default_country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub language: String,
    pub cities: Vec<String>,
    pub probability_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemographicRules {
    pub sex_ratio: f64, // Probability of male (0.0-1.0)
    pub twin_rate: f64,
    pub triplet_rate: f64,
    pub languages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipRules {
    pub marriage_probability: f64,
    pub divorce_probability: f64,
    pub remarriage_probability: f64,
    pub children_mean: f64,
    pub children_stddev: f64,
    pub min_children: usize,
    pub max_children: usize,
    pub generate_families: bool,
    pub multi_generational: bool,
    pub generations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrdinanceRules {
    pub include_lds_ordinances: bool,
    pub baptism_probability: f64,
    pub confirmation_probability: f64,
    pub endowment_probability: f64,
    pub sealing_to_parents_probability: f64,
    pub sealing_to_spouse_probability: f64,
    pub temples: Vec<String>,
}

impl Ruleset {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let ruleset: Ruleset = serde_json::from_str(&contents)?;
        Ok(ruleset)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    // Legacy preset functions - kept for backward compatibility in tests
    // New code should use PresetRegistry instead
    #[allow(dead_code)]
    pub fn default_english() -> Self {
        Ruleset {
            names: NameRules {
                male_given_names: vec![
                    "James",
                    "John",
                    "Robert",
                    "Michael",
                    "William",
                    "David",
                    "Richard",
                    "Joseph",
                    "Thomas",
                    "Charles",
                    "Christopher",
                    "Daniel",
                    "Matthew",
                    "Anthony",
                    "Donald",
                    "Mark",
                    "Paul",
                    "Steven",
                    "Andrew",
                    "Kenneth",
                    "Joshua",
                    "George",
                    "Edward",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                female_given_names: vec![
                    "Mary",
                    "Patricia",
                    "Jennifer",
                    "Linda",
                    "Elizabeth",
                    "Barbara",
                    "Susan",
                    "Jessica",
                    "Sarah",
                    "Karen",
                    "Nancy",
                    "Lisa",
                    "Margaret",
                    "Betty",
                    "Dorothy",
                    "Sandra",
                    "Ashley",
                    "Kimberly",
                    "Emily",
                    "Donna",
                    "Michelle",
                    "Carol",
                    "Amanda",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                surnames: vec![
                    "Smith",
                    "Johnson",
                    "Williams",
                    "Brown",
                    "Jones",
                    "Garcia",
                    "Miller",
                    "Davis",
                    "Rodriguez",
                    "Martinez",
                    "Hernandez",
                    "Lopez",
                    "Gonzalez",
                    "Wilson",
                    "Anderson",
                    "Thomas",
                    "Taylor",
                    "Moore",
                    "Jackson",
                    "Martin",
                    "Lee",
                    "Thompson",
                    "White",
                    "Harris",
                    "Clark",
                    "Lewis",
                    "Robinson",
                    "Walker",
                    "Young",
                    "Allen",
                    "King",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                use_patronymic: false,
                use_matronymic: false,
                name_format: NameFormat::WesternStyle,
            },
            dates: DateRules {
                birth_year_start: 1700,
                birth_year_end: 2010,
                min_marriage_age: 18,
                max_marriage_age: 45,
                min_parent_age: 16,
                max_parent_age: 50,
                life_expectancy_mean: 75,
                life_expectancy_stddev: 15,
                include_death_dates: true,
            },
            locations: LocationRules {
                countries: vec![Country {
                    name: "United States".to_string(),
                    language: "English".to_string(),
                    cities: [
                        "New York, New York",
                        "Los Angeles, California",
                        "Chicago, Illinois",
                        "Houston, Texas",
                        "Phoenix, Arizona",
                        "Philadelphia, Pennsylvania",
                        "San Antonio, Texas",
                        "San Diego, California",
                        "Dallas, Texas",
                        "Boston, Massachusetts",
                        "Salt Lake City, Utah",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    probability_weight: 1.0,
                }],
                default_country: "United States".to_string(),
            },
            demographics: DemographicRules {
                sex_ratio: 0.51, // Slightly more males at birth
                twin_rate: 0.032,
                triplet_rate: 0.001,
                languages: vec!["English".to_string()],
            },
            relationships: RelationshipRules {
                marriage_probability: 0.85,
                divorce_probability: 0.40,
                remarriage_probability: 0.50,
                children_mean: 2.5,
                children_stddev: 1.5,
                min_children: 0,
                max_children: 12,
                generate_families: true,
                multi_generational: true,
                generations: 4,
            },
            ordinances: OrdinanceRules {
                include_lds_ordinances: false,
                baptism_probability: 0.0,
                confirmation_probability: 0.0,
                endowment_probability: 0.0,
                sealing_to_parents_probability: 0.0,
                sealing_to_spouse_probability: 0.0,
                temples: vec![
                    "SLAKE".to_string(), // Salt Lake Temple
                    "PROVO".to_string(), // Provo City Center Temple
                    "MANTI".to_string(), // Manti Temple
                ],
            },
        }
    }

    #[allow(dead_code)]
    pub fn default_lds() -> Self {
        let mut ruleset = Self::default_english();
        ruleset.ordinances = OrdinanceRules {
            include_lds_ordinances: true,
            baptism_probability: 0.95,
            confirmation_probability: 0.95,
            endowment_probability: 0.75,
            sealing_to_parents_probability: 0.90,
            sealing_to_spouse_probability: 0.85,
            temples: vec![
                "SLAKE".to_string(),
                "PROVO".to_string(),
                "MANTI".to_string(),
                "LOGAN".to_string(),
                "STGEO".to_string(),
                "VERNA".to_string(),
                "MONTI".to_string(),
                "BOISE".to_string(),
                "DENVE".to_string(),
            ],
        };
        ruleset
    }

    #[allow(dead_code)]
    pub fn default_icelandic() -> Self {
        Ruleset {
            names: NameRules {
                male_given_names: vec![
                    "Jón",
                    "Sigurður",
                    "Guðmundur",
                    "Gunnar",
                    "Ólafur",
                    "Einar",
                    "Kristján",
                    "Magnús",
                    "Stefán",
                    "Jóhann",
                    "Árni",
                    "Þór",
                    "Bjarni",
                    "Helgi",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                female_given_names: vec![
                    "Guðrún",
                    "Anna",
                    "Kristín",
                    "Margrét",
                    "Sigríður",
                    "Helga",
                    "Ingibjörg",
                    "María",
                    "Jóhanna",
                    "Katrín",
                    "Sigrún",
                    "Ásta",
                    "Elín",
                    "Eva",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                surnames: vec![], // Icelandic uses patronymics
                use_patronymic: true,
                use_matronymic: true,
                name_format: NameFormat::IcelandicStyle,
            },
            dates: DateRules {
                birth_year_start: 1700,
                birth_year_end: 2010,
                min_marriage_age: 20,
                max_marriage_age: 40,
                min_parent_age: 18,
                max_parent_age: 48,
                life_expectancy_mean: 82,
                life_expectancy_stddev: 10,
                include_death_dates: true,
            },
            locations: LocationRules {
                countries: vec![Country {
                    name: "Iceland".to_string(),
                    language: "Icelandic".to_string(),
                    cities: [
                        "Reykjavík",
                        "Kópavogur",
                        "Hafnarfjörður",
                        "Akureyri",
                        "Reykjanesbær",
                        "Garðabær",
                        "Mosfellsbær",
                        "Selfoss",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    probability_weight: 1.0,
                }],
                default_country: "Iceland".to_string(),
            },
            demographics: DemographicRules {
                sex_ratio: 0.51,
                twin_rate: 0.032,
                triplet_rate: 0.001,
                languages: vec!["Icelandic".to_string()],
            },
            relationships: RelationshipRules {
                marriage_probability: 0.75,
                divorce_probability: 0.30,
                remarriage_probability: 0.60,
                children_mean: 2.1,
                children_stddev: 1.2,
                min_children: 0,
                max_children: 8,
                generate_families: true,
                multi_generational: true,
                generations: 4,
            },
            ordinances: OrdinanceRules {
                include_lds_ordinances: false,
                baptism_probability: 0.0,
                confirmation_probability: 0.0,
                endowment_probability: 0.0,
                sealing_to_parents_probability: 0.0,
                sealing_to_spouse_probability: 0.0,
                temples: vec![],
            },
        }
    }

    #[allow(dead_code)]
    pub fn default_spanish() -> Self {
        Ruleset {
            names: NameRules {
                male_given_names: vec![
                    "José",
                    "Antonio",
                    "Manuel",
                    "Francisco",
                    "Juan",
                    "David",
                    "José Antonio",
                    "Carlos",
                    "Javier",
                    "Miguel",
                    "Jesús",
                    "Pedro",
                    "Alejandro",
                    "Fernando",
                    "Luis",
                    "Sergio",
                    "Pablo",
                    "Jorge",
                    "Alberto",
                    "Ángel",
                    "Rafael",
                    "Daniel",
                    "Raúl",
                    "Enrique",
                    "Ramón",
                    "Vicente",
                    "Diego",
                    "Andrés",
                    "Ricardo",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                female_given_names: vec![
                    "María",
                    "Carmen",
                    "Ana",
                    "Isabel",
                    "Dolores",
                    "Pilar",
                    "Teresa",
                    "Rosa",
                    "Francisca",
                    "Antonia",
                    "Josefa",
                    "Lucía",
                    "María Carmen",
                    "Elena",
                    "Laura",
                    "Marta",
                    "Cristina",
                    "Paula",
                    "Sara",
                    "Raquel",
                    "Patricia",
                    "Beatriz",
                    "Silvia",
                    "Natalia",
                    "Carolina",
                    "Andrea",
                    "Sofía",
                    "Claudia",
                    "Alba",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                surnames: vec![
                    "García",
                    "Rodríguez",
                    "González",
                    "Fernández",
                    "López",
                    "Martínez",
                    "Sánchez",
                    "Pérez",
                    "Gómez",
                    "Martín",
                    "Jiménez",
                    "Ruiz",
                    "Hernández",
                    "Díaz",
                    "Moreno",
                    "Muñoz",
                    "Álvarez",
                    "Romero",
                    "Alonso",
                    "Gutiérrez",
                    "Navarro",
                    "Torres",
                    "Domínguez",
                    "Vázquez",
                    "Ramos",
                    "Gil",
                    "Ramírez",
                    "Serrano",
                    "Blanco",
                    "Molina",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                use_patronymic: false,
                use_matronymic: false,
                name_format: NameFormat::WesternStyle,
            },
            dates: DateRules {
                birth_year_start: 1700,
                birth_year_end: 2010,
                min_marriage_age: 18,
                max_marriage_age: 40,
                min_parent_age: 16,
                max_parent_age: 48,
                life_expectancy_mean: 83,
                life_expectancy_stddev: 12,
                include_death_dates: true,
            },
            locations: LocationRules {
                countries: vec![Country {
                    name: "Spain".to_string(),
                    language: "Spanish".to_string(),
                    cities: vec![
                        "Madrid",
                        "Barcelona",
                        "Valencia",
                        "Sevilla",
                        "Zaragoza",
                        "Málaga",
                        "Murcia",
                        "Palma",
                        "Las Palmas",
                        "Bilbao",
                        "Alicante",
                        "Córdoba",
                        "Valladolid",
                        "Vigo",
                        "Granada",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    probability_weight: 1.0,
                }],
                default_country: "Spain".to_string(),
            },
            demographics: DemographicRules {
                sex_ratio: 0.51,
                twin_rate: 0.032,
                triplet_rate: 0.001,
                languages: vec!["Spanish".to_string()],
            },
            relationships: RelationshipRules {
                marriage_probability: 0.80,
                divorce_probability: 0.25,
                remarriage_probability: 0.55,
                children_mean: 1.9,
                children_stddev: 1.1,
                min_children: 0,
                max_children: 10,
                generate_families: true,
                multi_generational: true,
                generations: 4,
            },
            ordinances: OrdinanceRules {
                include_lds_ordinances: false,
                baptism_probability: 0.0,
                confirmation_probability: 0.0,
                endowment_probability: 0.0,
                sealing_to_parents_probability: 0.0,
                sealing_to_spouse_probability: 0.0,
                temples: vec![],
            },
        }
    }

    #[allow(dead_code)]
    pub fn default_french() -> Self {
        Ruleset {
            names: NameRules {
                male_given_names: vec![
                    "Jean",
                    "Pierre",
                    "Michel",
                    "André",
                    "Philippe",
                    "Alain",
                    "Bernard",
                    "Jacques",
                    "Claude",
                    "François",
                    "René",
                    "Louis",
                    "Robert",
                    "Christian",
                    "Daniel",
                    "Marc",
                    "Paul",
                    "Nicolas",
                    "Julien",
                    "Thomas",
                    "Alexandre",
                    "Maxime",
                    "Antoine",
                    "Laurent",
                    "Olivier",
                    "Guillaume",
                    "Sébastien",
                    "Christophe",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                female_given_names: vec![
                    "Marie",
                    "Nathalie",
                    "Isabelle",
                    "Sylvie",
                    "Catherine",
                    "Françoise",
                    "Anne",
                    "Christine",
                    "Monique",
                    "Sophie",
                    "Véronique",
                    "Martine",
                    "Nicole",
                    "Valérie",
                    "Brigitte",
                    "Céline",
                    "Sandrine",
                    "Stéphanie",
                    "Émilie",
                    "Julie",
                    "Chloé",
                    "Camille",
                    "Léa",
                    "Emma",
                    "Clara",
                    "Louise",
                    "Alice",
                    "Manon",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                surnames: vec![
                    "Martin",
                    "Bernard",
                    "Dubois",
                    "Thomas",
                    "Robert",
                    "Richard",
                    "Petit",
                    "Durand",
                    "Leroy",
                    "Moreau",
                    "Simon",
                    "Laurent",
                    "Lefebvre",
                    "Michel",
                    "Garcia",
                    "David",
                    "Bertrand",
                    "Roux",
                    "Vincent",
                    "Fournier",
                    "Morel",
                    "Girard",
                    "André",
                    "Lefevre",
                    "Mercier",
                    "Dupont",
                    "Lambert",
                    "Bonnet",
                    "François",
                    "Martinez",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                use_patronymic: false,
                use_matronymic: false,
                name_format: NameFormat::WesternStyle,
            },
            dates: DateRules {
                birth_year_start: 1700,
                birth_year_end: 2010,
                min_marriage_age: 18,
                max_marriage_age: 42,
                min_parent_age: 18,
                max_parent_age: 48,
                life_expectancy_mean: 82,
                life_expectancy_stddev: 11,
                include_death_dates: true,
            },
            locations: LocationRules {
                countries: vec![Country {
                    name: "France".to_string(),
                    language: "French".to_string(),
                    cities: vec![
                        "Paris",
                        "Marseille",
                        "Lyon",
                        "Toulouse",
                        "Nice",
                        "Nantes",
                        "Strasbourg",
                        "Montpellier",
                        "Bordeaux",
                        "Lille",
                        "Rennes",
                        "Reims",
                        "Le Havre",
                        "Saint-Étienne",
                        "Toulon",
                        "Grenoble",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    probability_weight: 1.0,
                }],
                default_country: "France".to_string(),
            },
            demographics: DemographicRules {
                sex_ratio: 0.51,
                twin_rate: 0.032,
                triplet_rate: 0.001,
                languages: vec!["French".to_string()],
            },
            relationships: RelationshipRules {
                marriage_probability: 0.78,
                divorce_probability: 0.42,
                remarriage_probability: 0.60,
                children_mean: 2.0,
                children_stddev: 1.0,
                min_children: 0,
                max_children: 8,
                generate_families: true,
                multi_generational: true,
                generations: 4,
            },
            ordinances: OrdinanceRules {
                include_lds_ordinances: false,
                baptism_probability: 0.0,
                confirmation_probability: 0.0,
                endowment_probability: 0.0,
                sealing_to_parents_probability: 0.0,
                sealing_to_spouse_probability: 0.0,
                temples: vec![],
            },
        }
    }

    #[allow(dead_code)]
    pub fn default_italian() -> Self {
        Ruleset {
            names: NameRules {
                male_given_names: vec![
                    "Giuseppe",
                    "Giovanni",
                    "Antonio",
                    "Mario",
                    "Francesco",
                    "Luigi",
                    "Angelo",
                    "Vincenzo",
                    "Pietro",
                    "Salvatore",
                    "Carlo",
                    "Franco",
                    "Domenico",
                    "Bruno",
                    "Paolo",
                    "Michele",
                    "Giorgio",
                    "Marco",
                    "Andrea",
                    "Stefano",
                    "Alessandro",
                    "Roberto",
                    "Matteo",
                    "Lorenzo",
                    "Riccardo",
                    "Davide",
                    "Simone",
                    "Luca",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                female_given_names: vec![
                    "Maria",
                    "Anna",
                    "Giuseppina",
                    "Rosa",
                    "Angela",
                    "Giovanna",
                    "Teresa",
                    "Lucia",
                    "Carmela",
                    "Caterina",
                    "Francesca",
                    "Rita",
                    "Antonia",
                    "Paola",
                    "Laura",
                    "Alessandra",
                    "Giulia",
                    "Chiara",
                    "Sara",
                    "Martina",
                    "Federica",
                    "Valentina",
                    "Elena",
                    "Silvia",
                    "Elisa",
                    "Sofia",
                    "Beatrice",
                    "Giorgia",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                surnames: vec![
                    "Rossi", "Russo", "Ferrari", "Esposito", "Bianchi", "Romano", "Colombo",
                    "Ricci", "Marino", "Greco", "Bruno", "Gallo", "Conti", "De Luca", "Mancini",
                    "Costa", "Giordano", "Rizzo", "Lombardi", "Moretti", "Barbieri", "Fontana",
                    "Santoro", "Mariani", "Rinaldi", "Caruso", "Ferrara", "Galli", "Martini",
                    "Leone",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                use_patronymic: false,
                use_matronymic: false,
                name_format: NameFormat::WesternStyle,
            },
            dates: DateRules {
                birth_year_start: 1700,
                birth_year_end: 2010,
                min_marriage_age: 18,
                max_marriage_age: 40,
                min_parent_age: 18,
                max_parent_age: 48,
                life_expectancy_mean: 83,
                life_expectancy_stddev: 11,
                include_death_dates: true,
            },
            locations: LocationRules {
                countries: vec![Country {
                    name: "Italy".to_string(),
                    language: "Italian".to_string(),
                    cities: vec![
                        "Roma", "Milano", "Napoli", "Torino", "Palermo", "Genova", "Bologna",
                        "Firenze", "Bari", "Catania", "Venezia", "Verona", "Messina", "Padova",
                        "Trieste", "Brescia",
                    ]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                    probability_weight: 1.0,
                }],
                default_country: "Italy".to_string(),
            },
            demographics: DemographicRules {
                sex_ratio: 0.51,
                twin_rate: 0.032,
                triplet_rate: 0.001,
                languages: vec!["Italian".to_string()],
            },
            relationships: RelationshipRules {
                marriage_probability: 0.82,
                divorce_probability: 0.28,
                remarriage_probability: 0.50,
                children_mean: 1.8,
                children_stddev: 1.0,
                min_children: 0,
                max_children: 9,
                generate_families: true,
                multi_generational: true,
                generations: 4,
            },
            ordinances: OrdinanceRules {
                include_lds_ordinances: false,
                baptism_probability: 0.0,
                confirmation_probability: 0.0,
                endowment_probability: 0.0,
                sealing_to_parents_probability: 0.0,
                sealing_to_spouse_probability: 0.0,
                temples: vec![],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_english_ruleset() {
        let ruleset = Ruleset::default_english();

        assert!(!ruleset.names.male_given_names.is_empty());
        assert!(!ruleset.names.female_given_names.is_empty());
        assert!(!ruleset.names.surnames.is_empty());
        assert!(!ruleset.names.use_patronymic);
        assert_eq!(ruleset.locations.default_country, "United States");
        assert_eq!(ruleset.demographics.languages[0], "English");
        assert!(!ruleset.ordinances.include_lds_ordinances);
    }

    #[test]
    fn test_default_lds_ruleset() {
        let ruleset = Ruleset::default_lds();

        assert!(ruleset.ordinances.include_lds_ordinances);
        assert!(ruleset.ordinances.baptism_probability > 0.0);
        assert!(ruleset.ordinances.endowment_probability > 0.0);
        assert!(!ruleset.ordinances.temples.is_empty());
        assert!(ruleset.ordinances.temples.contains(&"SLAKE".to_string()));
    }

    #[test]
    fn test_default_icelandic_ruleset() {
        let ruleset = Ruleset::default_icelandic();

        assert!(ruleset.names.use_patronymic);
        assert!(ruleset.names.use_matronymic);
        assert!(ruleset.names.surnames.is_empty());
        assert_eq!(ruleset.locations.default_country, "Iceland");
        assert_eq!(ruleset.demographics.languages[0], "Icelandic");
        assert!(matches!(
            ruleset.names.name_format,
            NameFormat::IcelandicStyle
        ));
    }

    #[test]
    fn test_default_spanish_ruleset() {
        let ruleset = Ruleset::default_spanish();

        assert!(ruleset.names.male_given_names.contains(&"José".to_string()));
        assert!(ruleset
            .names
            .female_given_names
            .contains(&"María".to_string()));
        assert!(ruleset.names.surnames.contains(&"García".to_string()));
        assert_eq!(ruleset.locations.default_country, "Spain");
        assert_eq!(ruleset.demographics.languages[0], "Spanish");
    }

    #[test]
    fn test_default_french_ruleset() {
        let ruleset = Ruleset::default_french();

        assert!(ruleset.names.male_given_names.contains(&"Jean".to_string()));
        assert!(ruleset
            .names
            .female_given_names
            .contains(&"Marie".to_string()));
        assert_eq!(ruleset.locations.default_country, "France");
        assert_eq!(ruleset.demographics.languages[0], "French");
    }

    #[test]
    fn test_default_italian_ruleset() {
        let ruleset = Ruleset::default_italian();

        assert!(ruleset
            .names
            .male_given_names
            .contains(&"Giuseppe".to_string()));
        assert!(ruleset
            .names
            .female_given_names
            .contains(&"Maria".to_string()));
        assert!(ruleset.names.surnames.contains(&"Rossi".to_string()));
        assert_eq!(ruleset.locations.default_country, "Italy");
        assert_eq!(ruleset.demographics.languages[0], "Italian");
    }

    #[test]
    fn test_date_rules_validity() {
        let ruleset = Ruleset::default_english();

        assert!(ruleset.dates.birth_year_start < ruleset.dates.birth_year_end);
        assert!(ruleset.dates.min_marriage_age < ruleset.dates.max_marriage_age);
        assert!(ruleset.dates.min_parent_age < ruleset.dates.max_parent_age);
        assert!(ruleset.dates.life_expectancy_mean > 0);
        assert!(ruleset.dates.life_expectancy_stddev > 0);
    }

    #[test]
    fn test_demographic_probabilities() {
        let ruleset = Ruleset::default_english();

        assert!(ruleset.demographics.sex_ratio >= 0.0);
        assert!(ruleset.demographics.sex_ratio <= 1.0);
        assert!(ruleset.demographics.twin_rate >= 0.0);
        assert!(ruleset.demographics.twin_rate <= 1.0);
        assert!(ruleset.demographics.triplet_rate >= 0.0);
        assert!(ruleset.demographics.triplet_rate <= 1.0);
    }

    #[test]
    fn test_relationship_probabilities() {
        let ruleset = Ruleset::default_english();

        assert!(ruleset.relationships.marriage_probability >= 0.0);
        assert!(ruleset.relationships.marriage_probability <= 1.0);
        assert!(ruleset.relationships.divorce_probability >= 0.0);
        assert!(ruleset.relationships.divorce_probability <= 1.0);
        assert!(ruleset.relationships.remarriage_probability >= 0.0);
        assert!(ruleset.relationships.remarriage_probability <= 1.0);
        assert!(ruleset.relationships.min_children <= ruleset.relationships.max_children);
    }

    #[test]
    fn test_ruleset_serialization() {
        let ruleset = Ruleset::default_english();
        let json = serde_json::to_string(&ruleset).unwrap();

        assert!(!json.is_empty());
        assert!(json.contains("\"male_given_names\""));
        assert!(json.contains("\"surnames\""));
    }

    #[test]
    fn test_ruleset_deserialization() {
        let ruleset = Ruleset::default_english();
        let json = serde_json::to_string(&ruleset).unwrap();
        let deserialized: Ruleset = serde_json::from_str(&json).unwrap();

        assert_eq!(
            ruleset.names.male_given_names.len(),
            deserialized.names.male_given_names.len()
        );
        assert_eq!(
            ruleset.locations.default_country,
            deserialized.locations.default_country
        );
    }

    #[test]
    fn test_all_rulesets_have_names() {
        let rulesets = vec![
            Ruleset::default_english(),
            Ruleset::default_lds(),
            Ruleset::default_icelandic(),
            Ruleset::default_spanish(),
            Ruleset::default_french(),
            Ruleset::default_italian(),
        ];

        for ruleset in rulesets {
            assert!(
                !ruleset.names.male_given_names.is_empty(),
                "Male names should not be empty"
            );
            assert!(
                !ruleset.names.female_given_names.is_empty(),
                "Female names should not be empty"
            );
        }
    }

    #[test]
    fn test_all_rulesets_have_locations() {
        let rulesets = vec![
            Ruleset::default_english(),
            Ruleset::default_spanish(),
            Ruleset::default_french(),
            Ruleset::default_italian(),
            Ruleset::default_icelandic(),
        ];

        for ruleset in rulesets {
            assert!(
                !ruleset.locations.countries.is_empty(),
                "Countries should not be empty"
            );
            assert!(
                !ruleset.locations.default_country.is_empty(),
                "Default country should not be empty"
            );
        }
    }
}
