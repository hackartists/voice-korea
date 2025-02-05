#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::ApiModel;
use dioxus_translate::Translate;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default, ApiModel, Translate)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Field {
    #[default]
    #[translate(ko = "경제")]
    Economy = 1,
    #[translate(ko = "사회")]
    Society = 2,
    #[translate(ko = "환경")]
    Environment = 3,
    #[translate(ko = "교육")]
    Education = 4,
    #[translate(ko = "문화")]
    Culture = 5,
    #[translate(ko = "노동")]
    Labor = 6,
    #[translate(ko = "도시")]
    City = 7,
    #[translate(ko = "기술")]
    Technology = 8,
    #[translate(ko = "보건")]
    Health = 9,
    #[translate(ko = "정치")]
    Politics = 10,
}

// impl Field {
//     pub fn translate(&self, lang: &Language) -> &'static str {
//         match lang {
//             Language::En => match self {
//                 Field::Economy => "Economy",
//                 Field::Society => "Society",
//                 Field::Environment => "Environment",
//                 Field::Education => "Education",
//                 Field::Culture => "Culture",
//                 Field::Labor => "Labor",
//                 Field::City => "City",
//                 Field::Technology => "Technology",
//                 Field::Health => "Health",
//                 Field::Politics => "Politics",
//             },
//             Language::Ko => match self {
//                 Field::Economy => "경제",
//                 Field::Society => "사회",
//                 Field::Environment => "환경",
//                 Field::Education => "교육",
//                 Field::Culture => "문화",
//                 Field::Labor => "노동",
//                 Field::City => "도시",
//                 Field::Technology => "기술",
//                 Field::Health => "보건",
//                 Field::Politics => "정치",
//             },
//         }
//     }
// }

// impl std::fmt::Display for Field {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Field::Economy => write!(f, "economy"),
//             Field::Society => write!(f, "society"),
//             Field::Environment => write!(f, "environment"),
//             Field::Education => write!(f, "education"),
//             Field::Culture => write!(f, "culture"),
//             Field::Labor => write!(f, "labor"),
//             Field::City => write!(f, "city"),
//             Field::Technology => write!(f, "technology"),
//             Field::Health => write!(f, "health"),
//             Field::Politics => write!(f, "politics"),
//         }
//     }
// }

// impl FromStr for Field {
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "경제" | "Economy" => Ok(Field::Economy),
//             "사회" | "Society" => Ok(Field::Society),
//             "환경" | "Environment" => Ok(Field::Environment),
//             "교육" | "Education" => Ok(Field::Education),
//             "문화" | "Culture" => Ok(Field::Culture),
//             "노동" | "Labor" => Ok(Field::Labor),
//             "도시" | "City" => Ok(Field::City),
//             "기술" | "Technology" => Ok(Field::Technology),
//             "보건" | "Health" => Ok(Field::Health),
//             "정치" | "Politics" => Ok(Field::Politics),
//             _ => Err(format!("invalid field")),
//         }
//     }

//     type Err = String;
// }
