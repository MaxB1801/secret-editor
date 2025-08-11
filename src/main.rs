// Create proper comments for - cargo doc --open 

mod library;
mod gather_secret;

fn main() {
    println!("Gathering Secret!");
    let _ = gather_secret::get_secret("vault", "vault-tls");

}
