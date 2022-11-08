mod boids;
mod app;

use crate::app::{Model};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}