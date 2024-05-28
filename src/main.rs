use std::{env, io::Read, fs::File, process};


fn main(){
    // Get the command arguments
    let mut args = env::args();
    
    // Check if the user has provided a filename
    if args.len() < 2 {
        println!("Please provide a filename as a command-line argument.");
        process::exit(1);
    }
    
    // Get the filename from the command line arguments
    let filename: String = args.nth(1).unwrap();
    
    // Open the file
    match File::open(&filename) {
        Ok(mut file) => {
            // Read the contents of the file
            let mut contents: String = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    // Print the contents of the file
                    println!("{}", contents);
                },
                Err(e) => {
                    println!("Error reading file: {}", e);
                    process::exit(1);
                }
            }
        },
        Err(e) => {
            println!("Error opening file: {}", e);
            process::exit(1);
        }
    };
}
