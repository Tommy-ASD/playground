use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Response {
    code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    notes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    example: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<Value>,
}

impl Response {
    pub fn new(code: u16) -> Self {
        Response {
            code,
            ..Default::default()
        }
    }
    pub fn with_code(self, code: u16) -> Self {
        Response { code, ..self }
    }
    pub fn with_error(self, error: &str) -> Self {
        Response {
            error: Some(error.to_string()),
            ..self
        }
    }
    pub fn with_details(self, details: &str) -> Self {
        Response {
            details: Some(details.to_string()),
            ..self
        }
    }
    pub fn with_notes(self, notes: Vec<String>) -> Self {
        Response { notes, ..self }
    }
    pub fn with_example(self, example: &Value) -> Self {
        Response {
            example: Some(example.clone()),
            ..self
        }
    }
    pub fn with_result(self, result: &Value) -> Self {
        Response {
            result: Some(result.clone()),
            ..self
        }
    }
    pub fn with_page(self, page: &Value) -> Self {
        Response {
            page: Some(page.clone()),
            ..self
        }
    }
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).expect("Could not convert response to string")
    }
}
