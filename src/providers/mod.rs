pub mod fireworks;

use std::fmt::Display;

pub trait Provider: Display {
    fn name(&self) -> &'static str;
    fn models(&self) -> Vec<&'static str>;
}

pub fn get_providers() -> Vec<Box<dyn Provider>> {
    vec![
        Box::new(fireworks::Fireworks),
        // Add other providers here
    ]
}
