extern crate core;

use std::panic;

use crate::components::app::App;

mod components;
mod config;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    yew::Renderer::<App>::new().render();
}
