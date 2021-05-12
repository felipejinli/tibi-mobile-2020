//! All the response types to the `/image/*` endpoints.

use serde::Serialize;

use std::collections::HashMap;

#[derive(Serialize, Default)]
pub struct NewImages {
    pub images: HashMap<String, NewImage>,
}

#[derive(Serialize)]
pub struct NewImage {
    pub original_name: String,
    pub id: String,
}
