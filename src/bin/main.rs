extern crate milstian;

use milstian::{Application, Config};

fn main() {
    Application::new(Config::from_env(), true);
}
