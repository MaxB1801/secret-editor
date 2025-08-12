use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::env;
use serde_json;
use std::fs;

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
        println!("Editing Secret: {}", self.secret_name);

        // get file path of exe directory
        let exe_directory = match env::current_exe() {
            Ok(exe_path) => {
                // println!("Path of this executable is: {}", exe_path.display());
                exe_path.parent().unwrap().to_path_buf()
            }
            Err(e) => {
                println!("failed to get current exe path: {e}");
                std::process::exit(1);
            }
        };
        let path = exe_directory.join("secret.json");


        // write to file
        create_file(self.data.clone(), path.clone());
        let _ = load_editor(path.clone());
        match read_json_to_hashmap(path) {
            Ok(map) => {
                self.data = map;
            }
            Err(e) => eprintln!("Error reading JSON: {}", e),
        }
        "created secret file"

    }
}

fn load_editor(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let display = path.display();

    // Load the editor from the environment variable or use a default
    let editor = env::var("EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| {
            if cfg!(windows) {
                "notepad".to_string()
            } else {
                "vim".to_string()
            }
    });

    // Run the editor command with the file path as argument
    let status = std::process::Command::new(editor)
        .arg(display.to_string())
        .status()?;

    if status.success() {
       // println!("Edited {}", display);
    } else {
        eprintln!("Editor exited with status: {}", status);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Editor failed",
        )));
    }

    Ok(())
    
    // // Execute the editor command
    // match std::process::Command::new(editor).status() {
    //     Ok(status) => {
    //         if !status.success() {
    //             eprintln!("Editor exited with non-zero status: {}", status);
    //         }
    //     }
    //     Err(e) => eprintln!("Failed to execute editor: {}", e),
    // }
}

fn create_file(content: HashMap<String, String>, path: PathBuf) {

  //  println!("Secret file path: {}", path.display());
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

        serde_json::to_writer_pretty(&file, &content)
        .expect("Failed to write JSON to file");

    // // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    // match file.write_all(content.as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
    //     Ok(_) => println!("successfully wrote to {}", display),
    // }
}

fn read_json_to_hashmap(path: PathBuf) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let map: HashMap<String, String> = serde_json::from_str(&data)?;
    Ok(map)
}