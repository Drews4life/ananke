extern crate rand;

use std::path::{PathBuf};
use rand::distributions::{Alphanumeric, DistString};

pub fn get_child_path_from_string(component: &String) -> PathBuf {
    let raw_path = format!("./{}", component.clone());

    PathBuf::from(raw_path)
}

pub fn generate_arbitrary_string() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 20)
}

pub fn create_branch_name() -> String {
    format!("ananke/dev/{}", generate_arbitrary_string())
}