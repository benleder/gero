use std::collections::HashMap;
use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LanguageFile(HashMap<String, String>);

#[derive(Debug)]
pub struct Localizer {
    translations: HashMap<String, String>,
}

impl Localizer {
    pub fn new(language: &str) -> std::io::Result<Self> {
        let mut loc = Localizer { translations: HashMap::new() };
        loc.load(language)?;
        Ok(loc)
    }

    pub fn load(&mut self, language: &str) -> std::io::Result<()> {
        let path = format!("assets/locales/{}.json", language);
        let data = fs::read_to_string(path)?;
        let map: HashMap<String, String> = serde_json::from_str(&data).unwrap_or_default();
        self.translations = map;
        Ok(())
    }

    pub fn get(&self, key: &str) -> String {
        self.translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_language_file() {
        let loc = Localizer::new("en").unwrap();
        assert_eq!(loc.get("ui.tab.abilities"), "Abilities");
    }
}
