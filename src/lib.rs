pub mod runtime;
pub mod store;
pub mod utils;

use js_sys::{Object, Reflect};
use store::Store;
use wasm_bindgen::prelude::*;

#[cfg(feature = "debug")]
use wasm_bindgen_test::console_log;

#[cfg(feature = "debug")]
#[wasm_bindgen]
pub fn enable_debug() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn create_store(id: &str, states: &Object) -> Store {
    #[cfg(feature = "debug")]
    console_log!("Creating store {}...", id);

    let mut store = Store::new(id.to_string());

    Object::keys(states).for_each(&mut |key, _, _| {
        store.set(
            key.as_string()
                .unwrap_or_else(|| panic!("Invalid key: {:?}", key)),
            Reflect::get(&states, &key).unwrap(),
        );
    });
    store
}
