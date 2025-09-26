use anyhow::anyhow;
use std::{fmt::Display, str::FromStr};

pub enum Platform {
    Twitter,
    E621,
    FurAffinity,
    Unknown,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Platform::Twitter => "Twitter",
            Platform::E621 => "e621",
            Platform::FurAffinity => "FurAffinity",
            Platform::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Twitter" => Ok(Platform::Twitter),
            "E621" => Ok(Platform::E621),
            "FurAffinity" => Ok(Platform::FurAffinity),
            _ => Err(anyhow!("'{}' is not a valid platform", s)),
        }
    }
}
