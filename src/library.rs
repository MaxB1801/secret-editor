use std::collections::HashMap;

#[derive(Debug)]pub struct KubernetesSecret {
    pub context: String,
    pub namespace: String,
    pub secret_name: String,
    pub data: HashMap<String, String>
}

// pub struct SecretData {
//     pub data: HashMap<String, String>,
// }

// pub struct SecretData {
//     pub keys: Vec<String>,
//     pub value: Vec<String>,
// }


impl KubernetesSecret {
    pub fn edit_secret(&mut self) -> &str {
        "64"
    }
}
