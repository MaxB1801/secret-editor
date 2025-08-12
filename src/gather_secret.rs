use kube::{Client, api::{Api, ResourceExt, ListParams, PostParams}};
use k8s_openapi::api::core::v1::Secret;
use crate::library::KubernetesSecret;
use std::collections::HashMap;
use k8s_openapi::ByteString;

#[tokio::main]
pub async fn get_secret(namespace: &str, secret: &str) -> Result<KubernetesSecret, Box<dyn std::error::Error>> { 
    // Define template hashmap
    let secret_map = HashMap::new();
    
    // define vector of kubernetes secrets
    let mut target_secret = KubernetesSecret {
    context: "default".to_string(),
    namespace: namespace.to_string(),      
    secret_name: secret.to_string(),    
    data: secret_map.clone(),
};

    println!("Target {} cluster", target_secret.context);
    
    // Infer the runtime environment and try to create a Kubernetes Client
    let client = Client::try_default().await?;

    // Read pods in the configured namespace into the typed interface from k8s-openapi
    let secrets: Api<Secret> = Api::namespaced(client, namespace);
    for s in secrets.list(&ListParams::default()).await? {
        // println!("found secret {}", s.name_any());
        
        if s.name_any() != secret {
            continue;
        } else {
            target_secret.secret_name = s.name_any();
        }

    // if let Some(data) = &s.data {
    //     for (key, value) in data {
    //         let decoded_value = match std::str::from_utf8(&value.0) {
    //             Ok(v) => v,
    //             Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    //         };

    //         // Interpret escape sequences like \n, \t, etc.
    //         let cleaned_value = match unescape::unescape(decoded_value) {
    //             Some(v) => v,
    //             None => decoded_value.to_string(),
    //         };

    //         target_secret
    //             .data
    //             .insert(key.clone(), cleaned_value);
    //     }
    // }
    if let Some(data) = &s.data {
        for (key, value) in data {
            // println!("{}", key);
            let decoded_value = match str::from_utf8(&value.0) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
            
            target_secret.data.insert(key.clone(), decoded_value.to_string());
        }
        
        // println!("{:?}", data)
    }
    }

    // println!("{:x?}", target_secret);
    Ok(target_secret)
}

#[tokio::main]
pub async fn update_kubernetes_secret(data : HashMap<String, String>, namespace: &str, secret_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a Kubernetes client
    let client = Client::try_default().await?;

    // Define the API for the secrets in the specified namespace
    let secrets: Api<Secret> = Api::namespaced(client, namespace);

    let secret = Secret {
        metadata: kube::api::ObjectMeta {
            name: Some(secret_name.to_string()),
            ..Default::default()
        },
        data: Some(data.into_iter().map(|(k, v)| (k, ByteString(v.into_bytes()))).collect()),
        ..Default::default()
    };


    // Update the secret in Kubernetes
    secrets.replace(&secret.metadata.name.clone().unwrap(), &PostParams::default(), &secret).await?;

    Ok(())
}