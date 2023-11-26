use std::fmt::{Display, Formatter};

pub enum BuildType {
    Uniques,
    Views,
}

impl BuildType {
    pub(crate) fn toggle(&self) -> BuildType {
        match self {
            BuildType::Uniques => BuildType::Views,
            BuildType::Views => BuildType::Uniques,
        }
    }
}

impl Display for BuildType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildType::Uniques => write!(f, "Uniques"),
            BuildType::Views => write!(f, "Views"),
        }
    }
}