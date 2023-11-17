use serde::{Deserialize, Serialize};

#[doc = "Describes an alignment between an achievement and a node in an educational framework."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Alignment {
    #[doc = "If applicable, a locally unique string identifier that identifies the alignment target within its framework and/or targetUrl."]
    #[serde(
        rename = "targetCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_code: Option<String>,
    #[doc = "Short description of the alignment target."]
    #[serde(
        rename = "targetDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_description: Option<String>,
    #[doc = "Name of the framework the alignment target."]
    #[serde(
        rename = "targetFramework",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_framework: Option<String>,
    #[doc = "Name of the alignment."]
    #[serde(rename = "targetName")]
    pub target_name: String,
    #[doc = "The type of the alignment target node."]
    #[serde(
        rename = "targetType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_type: Option<AlignmentTargetType>,
    #[doc = "URL linking to the official description of the alignment target, for example an individual standard within an educational framework."]
    #[serde(rename = "targetUrl")]
    pub target_url: String,
    #[serde(rename = "type")]
    pub type_: AlignmentType,
}
impl From<&Alignment> for Alignment {
    fn from(value: &Alignment) -> Self {
        value.clone()
    }
}
impl Alignment {
    pub fn builder() -> builder::Alignment {
        builder::Alignment::default()
    }
}
#[doc = "The type of the alignment target node."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AlignmentTargetType {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<AlignmentTargetTypeSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<AlignmentTargetTypeSubtype1>,
}
impl From<&AlignmentTargetType> for AlignmentTargetType {
    fn from(value: &AlignmentTargetType) -> Self {
        value.clone()
    }
}
impl AlignmentTargetType {
    pub fn builder() -> builder::AlignmentTargetType {
        builder::AlignmentTargetType::default()
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AlignmentTargetTypeSubtype0 {
    #[serde(rename = "ceasn:Competency")]
    CeasnCompetency,
    #[serde(rename = "ceterms:Credential")]
    CetermsCredential,
    #[serde(rename = "CFItem")]
    CfItem,
    #[serde(rename = "CFRubric")]
    CfRubric,
    #[serde(rename = "CFRubricCriterion")]
    CfRubricCriterion,
    #[serde(rename = "CFRubricCriterionLevel")]
    CfRubricCriterionLevel,
    #[serde(rename = "CTDL")]
    Ctdl,
}
impl From<&AlignmentTargetTypeSubtype0> for AlignmentTargetTypeSubtype0 {
    fn from(value: &AlignmentTargetTypeSubtype0) -> Self {
        value.clone()
    }
}
impl ToString for AlignmentTargetTypeSubtype0 {
    fn to_string(&self) -> String {
        match *self {
            Self::CeasnCompetency => "ceasn:Competency".to_string(),
            Self::CetermsCredential => "ceterms:Credential".to_string(),
            Self::CfItem => "CFItem".to_string(),
            Self::CfRubric => "CFRubric".to_string(),
            Self::CfRubricCriterion => "CFRubricCriterion".to_string(),
            Self::CfRubricCriterionLevel => "CFRubricCriterionLevel".to_string(),
            Self::Ctdl => "CTDL".to_string(),
        }
    }
}
impl std::str::FromStr for AlignmentTargetTypeSubtype0 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "ceasn:Competency" => Ok(Self::CeasnCompetency),
            "ceterms:Credential" => Ok(Self::CetermsCredential),
            "CFItem" => Ok(Self::CfItem),
            "CFRubric" => Ok(Self::CfRubric),
            "CFRubricCriterion" => Ok(Self::CfRubricCriterion),
            "CFRubricCriterionLevel" => Ok(Self::CfRubricCriterionLevel),
            "CTDL" => Ok(Self::Ctdl),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AlignmentTargetTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AlignmentTargetTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AlignmentTargetTypeSubtype0 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AlignmentTargetTypeSubtype1(String);
impl std::ops::Deref for AlignmentTargetTypeSubtype1 {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<AlignmentTargetTypeSubtype1> for String {
    fn from(value: AlignmentTargetTypeSubtype1) -> Self {
        value.0
    }
}
impl From<&AlignmentTargetTypeSubtype1> for AlignmentTargetTypeSubtype1 {
    fn from(value: &AlignmentTargetTypeSubtype1) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for AlignmentTargetTypeSubtype1 {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("(ext:)[a-z|A-Z|0-9|.|-|_]+")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"(ext:)[a-z|A-Z|0-9|.|-|_]+\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for AlignmentTargetTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AlignmentTargetTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AlignmentTargetTypeSubtype1 {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for AlignmentTargetTypeSubtype1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AlignmentType {
    Variant0(String),
    Variant1(Vec<String>),
}
impl From<&AlignmentType> for AlignmentType {
    fn from(value: &AlignmentType) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for AlignmentType {
    fn from(value: Vec<String>) -> Self {
        Self::Variant1(value)
    }
}

pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Alignment {
        target_code: Result<Option<String>, String>,
        target_description: Result<Option<String>, String>,
        target_framework: Result<Option<String>, String>,
        target_name: Result<String, String>,
        target_type: Result<Option<super::AlignmentTargetType>, String>,
        target_url: Result<String, String>,
        type_: Result<super::AlignmentType, String>,
    }
    impl Default for Alignment {
        fn default() -> Self {
            Self {
                target_code: Ok(Default::default()),
                target_description: Ok(Default::default()),
                target_framework: Ok(Default::default()),
                target_name: Err("no value supplied for target_name".to_string()),
                target_type: Ok(Default::default()),
                target_url: Err("no value supplied for target_url".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Alignment {
        pub fn target_code<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.target_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target_code: {}", e));
            self
        }
        pub fn target_description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.target_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for target_description: {}",
                    e
                )
            });
            self
        }
        pub fn target_framework<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.target_framework = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for target_framework: {}",
                    e
                )
            });
            self
        }
        pub fn target_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.target_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target_name: {}", e));
            self
        }
        pub fn target_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AlignmentTargetType>>,
            T::Error: std::fmt::Display,
        {
            self.target_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target_type: {}", e));
            self
        }
        pub fn target_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.target_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for target_url: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AlignmentType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Alignment> for super::Alignment {
        type Error = String;
        fn try_from(value: Alignment) -> Result<Self, String> {
            Ok(Self {
                target_code: value.target_code?,
                target_description: value.target_description?,
                target_framework: value.target_framework?,
                target_name: value.target_name?,
                target_type: value.target_type?,
                target_url: value.target_url?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::Alignment> for Alignment {
        fn from(value: super::Alignment) -> Self {
            Self {
                target_code: Ok(value.target_code),
                target_description: Ok(value.target_description),
                target_framework: Ok(value.target_framework),
                target_name: Ok(value.target_name),
                target_type: Ok(value.target_type),
                target_url: Ok(value.target_url),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AlignmentTargetType {
        subtype_0: Result<Option<super::AlignmentTargetTypeSubtype0>, String>,
        subtype_1: Result<Option<super::AlignmentTargetTypeSubtype1>, String>,
    }
    impl Default for AlignmentTargetType {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl AlignmentTargetType {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AlignmentTargetTypeSubtype0>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AlignmentTargetTypeSubtype1>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AlignmentTargetType> for super::AlignmentTargetType {
        type Error = String;
        fn try_from(value: AlignmentTargetType) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl From<super::AlignmentTargetType> for AlignmentTargetType {
        fn from(value: super::AlignmentTargetType) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
}
