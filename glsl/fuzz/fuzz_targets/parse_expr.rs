#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate glsl;

fuzz_target!(|data: &str| {
  let _ = glsl::expr(data);
});
