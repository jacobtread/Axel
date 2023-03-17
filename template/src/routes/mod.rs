use axel::prelude::*;

// Define your other route modules here
mod hello;
mod test;

// Hook your route modules here
define_configure![hello, test];
