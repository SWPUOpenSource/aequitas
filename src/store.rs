use std::collections::HashMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Store {
    name: String,
    states: HashMap<String, JsValue>,
}

#[wasm_bindgen]
impl Store {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            states: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: JsValue) {
        self.states.insert(key, value);
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
