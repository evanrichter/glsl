#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate glsl;

fuzz_target!(|data: &str| {
  if let Ok((_, expr)) = glsl::expr(data) {
    let mut output = String::new();
    glsl::transpiler::glsl::show_expr(&mut output, &expr);
    output.push(';');
    let readback_expr = glsl::expr(&output);
    match readback_expr {
      Ok((_, readback_expr)) => assert_eq!(expr, readback_expr),
      Err(_) => panic!("Failed to re-parse '{}'", output),
    }
  }
});
