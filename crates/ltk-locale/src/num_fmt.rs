//! Number and currency formatting.

pub struct NumberFormatter { pub locale: String, pub decimals: usize }

impl NumberFormatter {
    pub fn new(locale: impl Into<String>) -> Self { Self { locale: locale.into(), decimals: 2 } }

    fn separators(&self) -> (char, char) {
        match self.locale.split('-').next().unwrap_or("en") {
            "de"|"fr"|"ru"|"es" => ('.', ','),
            _ => (',', '.'),
        }
    }

    pub fn format(&self, n: f64) -> String {
        let (group_sep, decimal_sep) = self.separators();
        let whole = n.trunc().abs() as u64;
        let frac  = ((n.abs() - n.trunc().abs()) * 10f64.powi(self.decimals as i32)).round() as u64;
        let whole_str: String = whole.to_string().chars().rev().enumerate()
            .flat_map(|(i, c)| if i > 0 && i % 3 == 0 { vec![group_sep, c] } else { vec![c] })
            .collect::<String>().chars().rev().collect();
        let sign = if n < 0.0 { "-" } else { "" };
        if self.decimals == 0 { format!("{}{}", sign, whole_str) }
        else { format!("{}{}{}{:0>width$}", sign, whole_str, decimal_sep, frac, width = self.decimals) }
    }

    pub fn format_currency(&self, amount: f64, currency_code: &str) -> String {
        let symbol = match currency_code { "USD" => "$", "EUR" => "€", "GBP" => "£", "JPY" => "¥", c => c };
        format!("{}{}", symbol, self.format(amount))
    }
}
