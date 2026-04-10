pub mod app;
pub mod auth;
pub mod components;
pub mod context;
pub mod pages;

use crate::app::App;
use leptos::prelude::mount_to_body;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
