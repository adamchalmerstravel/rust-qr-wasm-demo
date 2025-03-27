use base64::Engine;
use image::Luma;
use qrcode::QrCode;
use wasm_bindgen::prelude::*;

// Imports JS fns to Rust
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn msg() -> String {
    alert("Hello from Rust");
    "Hello".to_owned()
}

pub fn qr_png_for(url: String) -> Vec<u8> {
    let code = QrCode::new(url).unwrap();
    let img = code.render::<Luma<u8>>().build();
    let mut bytes = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageFormat::Png,
    )
    .unwrap();
    bytes
}

#[wasm_bindgen]
pub fn qr_png_b64(url: String) -> String {
    let png_bytes = qr_png_for(url);
    base64::engine::general_purpose::STANDARD.encode(png_bytes)
}

#[wasm_bindgen]
pub async fn validate_link(data: String) -> Result<(), String> {
  let url = match url::Url::parse(&data) {
    Ok(u) => u,
    Err(e) => return Err(e.to_string()),
  };
  match reqwest::get(url).await {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}