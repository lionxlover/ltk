//! ICU-compatible plural rule engine.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluralCategory { Zero, One, Two, Few, Many, Other }

/// A compiled plural rule for a given locale.
pub struct PluralRule { pub locale: String }

impl PluralRule {
    pub fn new(locale: &str) -> Self { Self { locale: locale.into() } }

    /// Classify a count into a plural category (English-centric default).
    pub fn classify(&self, n: f64) -> PluralCategory {
        match self.locale.split('-').next().unwrap_or("en") {
            "ar" => self.arabic(n),
            "ru" | "uk" | "be" => self.slavic(n),
            _ => if n == 1.0 { PluralCategory::One } else { PluralCategory::Other }
        }
    }

    fn arabic(&self, n: f64) -> PluralCategory {
        match n as u64 % 100 {
            0 => PluralCategory::Zero,
            1 => PluralCategory::One,
            2 => PluralCategory::Two,
            3..=10 => PluralCategory::Few,
            11..=99 => PluralCategory::Many,
            _ => PluralCategory::Other,
        }
    }

    fn slavic(&self, n: f64) -> PluralCategory {
        let n = n as u64;
        let last_two = n % 100;
        let last_one = n % 10;
        if last_two >= 11 && last_two <= 14 { return PluralCategory::Many; }
        match last_one {
            1 => PluralCategory::One,
            2..=4 => PluralCategory::Few,
            _ => PluralCategory::Many,
        }
    }
}
