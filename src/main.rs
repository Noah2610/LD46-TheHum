extern crate deathframe;
extern crate ron;
#[macro_use]
extern crate serde;

mod components;
mod init;
mod input;
mod resources;
mod settings;
mod states;
mod systems;

pub use deathframe::core::resource_helper::resource;

fn main() {
    if let Err(e) = init::run() {
        eprintln!("An error occured: {}", e);
        std::process::exit(1);
    }
}
