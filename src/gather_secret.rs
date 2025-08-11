use futures::{StreamExt, TryStreamExt};
use kube::{Client, api::{Api, ResourceExt, ListParams, PostParams}};
use k8s_openapi::api::core::v1::Secret;
use crate::library::KubernetesSecret;
use std::collections::HashMap;

#[tokio::main]
pub async fn get_secret(namespace: &str, secret: &str) -> Result<KubernetesSecret, Box<dyn std::error::Error>> { 
    // Define template hashmap
    let mut secret_map = HashMap::new();
    
    // define vector of kubernetes secrets
    let mut target_secret = KubernetesSecret {
    context: "default".to_string(),
    namespace: namespace.to_string(),       // fill with default or actual value
    secret_name: secret.to_string(),     // fill with default or actual value
    data: secret_map.clone(),
};
    
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await?;

    // Read pods in the configured namespace into the typed interface from k8s-openapi
    let secrets: Api<Secret> = Api::namespaced(client, namespace);
    for s in secrets.list(&ListParams::default()).await? {
        println!("found pod {}", s.name_any());


        if let Some(data) = &s.data {
            for (key, value) in data {
                println!("{}", key);
                let decoded_value = match str::from_utf8(&value.0) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
                
                target_secret.data.insert(key.clone(), decoded_value.to_string());
            }
            
            // println!("{:?}", data)
        }
    }

    println!("{:x?}", target_secret);
    Ok(target_secret)
}