use std::collections::HashMap;

use winit::keyboard::PhysicalKey;

pub struct MenuSettingsKeyCache {
    names: HashMap<PhysicalKey, String>,
}

impl MenuSettingsKeyCache {
    pub fn new() -> Self {
        return Self {
            names: HashMap::new(),
        };
    }

    pub fn name<'a>(&'a mut self, key: &PhysicalKey) -> &'a str {
        if !self.names.contains_key(key) {
            let name = match key {
                PhysicalKey::Code(code) => format!("{:?}", code).to_ascii_uppercase(),
                PhysicalKey::Unidentified(native) => format!("{:?}", native).to_ascii_uppercase(),
            };

            self.names.insert(*key, name);
        }

        return self.names.get(key).unwrap();
    }
}
