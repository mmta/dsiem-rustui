mod app;
mod alarm;
mod config;
extern crate chrono;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}