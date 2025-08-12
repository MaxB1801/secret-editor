// Create proper comments for - cargo doc --open 

mod library;
mod gather_secret;
use std::env;

fn main() {


    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <namespace> <secret>", args[0]);
        std::process::exit(1);
    }

    let namespace = &args[1];
    let secret = &args[2];

    let mut found_secret = match gather_secret::get_secret(&namespace, &secret) {
        Ok(secret) => secret,
        Err(e) => {
            eprintln!("Error getting secret - Secret might not exist: {}", e);
            return;
        }
    };

    let _ = found_secret.edit_secret();

   // println!("{:x?}", found_secret);

    println!("Updating Secret!");

    match gather_secret::update_kubernetes_secret(found_secret.data, found_secret.namespace.as_str(), found_secret.secret_name.as_str()) {
        Ok(_) => println!("Secret updated successfully!"),
        Err(e) => eprintln!("Error updating secret: {}", e),
    } 

}
