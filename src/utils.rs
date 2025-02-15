use wasm_bindgen::prelude::wasm_bindgen;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_wasm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn is_debug() -> bool {
    cfg!(feature = "debug")
}

#[wasm_bindgen]
pub fn enable_logger() -> Result<(), String> {
    #[cfg(feature = "debug")]
    console_log::init().map_err(|e| e.to_string())?;
    Ok(())
}
