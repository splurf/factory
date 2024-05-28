use chrono::{DateTime, Local};
use clap::ValueEnum;
use serde::Deserialize;
use serde_with::{serde_as, NoneAsEmptyString};
use strum::EnumString;

#[derive(Clone, Copy, Deserialize, Debug, EnumString, PartialEq, ValueEnum)]
pub enum Country {
    AUD,
    CAD,
    CHF,
    CNY,
    EUR,
    GBP,
    JPY,
    NZD,
    USD,
    All, // hmmm
}

#[derive(Clone, Copy, Deserialize, Debug, PartialEq)]
pub enum Impact {
    Holiday = 0, // hmm
    Low = 1,
    Medium = 2,
    High = 3,
}

#[serde_as]
#[derive(Clone, Deserialize, Debug)]
pub struct Event {
    title: String,
    country: Country,
    date: DateTime<Local>,
    impact: Impact,
    #[serde_as(as = "NoneAsEmptyString")]
    forecast: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    previous: Option<String>,
    url: String,
}

impl Event {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub const fn country(&self) -> Country {
        self.country
    }

    pub const fn date(&self) -> DateTime<Local> {
        self.date
    }

    pub const fn is_normal(&self) -> bool {
        self.impact as u8 > 0
    }

    pub fn forecast(&self) -> Option<&str> {
        self.forecast.as_deref()
    }

    pub fn previous(&self) -> Option<&str> {
        self.previous.as_deref()
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!(
            "`{:?}`  |  **{:?}**  |  [{}]({})",
            self.country(),
            self.impact,
            self.title(),
            self.url(),
        );

        if let Some(forecast) = self.forecast() {
            s.push_str(&format!("  |  `{}`", forecast));
        }

        if let Some(previous) = self.previous() {
            s.push_str(&format!("  |  `{}`", previous));
        }
        f.write_str(&s)
    }
}
