use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

pub fn set_cookie(key: &str, value: &str) {
    let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
    let cookie = format!("{key}={value}; Secure;");
    match document.set_cookie(&cookie) {
        Ok(_) => {}
        Err(e) => gloo::console::error!("Error setting cookie", e),
    }
}
