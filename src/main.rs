extern crate deathframe;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate serde_json;

pub mod components;
pub mod entities;
pub mod init;
pub mod input;
pub mod level_loader;
pub mod resources;
pub mod settings;
pub mod states;
pub mod systems;

pub use deathframe::core::resource_helper::resource;

fn main() {
    if let Err(e) = init::run() {
        eprintln!("An error occured: {}", e);
        std::process::exit(1);
    }
}
