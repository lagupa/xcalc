use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate(a: f64, b: f64, op: &str) -> Result<f64, JsValue> {
    match op {
        "add" => Ok(a + b),
        "sub" => Ok(a - b),
        "mul" => Ok(a * b),
        "div" => {
            if b == 0.0 {
                Err(JsValue::from_str("Division by zero"))
            } else {
                Ok(a / b)
            }
        }
        _ => Err(JsValue::from_str(&format!("Unknown operator: {}", op))),
    }
}