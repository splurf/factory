use chrono::{DateTime, Local};
use clap::ValueEnum;
use serde::Deserialize;
use serde_with::{serde_as, NoneAsEmptyString};
use strum::EnumString;

#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, ValueEnum)]
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

#[derive(
    Clone, Copy, Debug, Default, Deserialize, EnumString, PartialEq, PartialOrd, ValueEnum,
)]
pub enum Impact {
    #[default]
    Holiday,
    Low,
    Medium,
    High,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
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

    pub const fn impact(&self) -> Impact {
        self.impact
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
            "`{}`  |  **{:?}**  |  [**{:?}** {}](<{}>)",
            self.date().format("%H:%M"),
            self.impact,
            self.country(),
            self.title(),
            self.url(),
        );

        if let Some(forecast) = self.forecast() {
            s.push_str(&format!("  |  `Forecast = {}`", forecast));
        }

        if let Some(previous) = self.previous() {
            s.push_str(&format!("  |  `Previous = {}`", previous));
        }
        f.write_str(&s)
    }
}
