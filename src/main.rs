use std::env;

use rand::RngCore;

fn main() {
    // Take in two command line arguments, one i32 and one optional String
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { // Make sure at least one argument is provided
        println!("Error: First argument must be a valid integer (Ex. 32)");
        println!("Argument not supplied");
        return;
    }

    let new_password_length: Option<i32> = args[1].parse().ok();
    if new_password_length.is_none() { // Make sure i32 parsed correctly
        println!("Error: First argument must be a valid integer (Ex. 32)");
        return;
    }

    // Disallowed_chars is an optional argument
    let disallowed_chars: Option<&String> = args.get(2);

    let mut allowed_chars: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-=`~!@#$%^&*()_+[]\\{}|;':,./<>?");

    // Remove disallowed characters from allowed_characters if disallowed_chars is specified
    if let Some(disallowed_chars) = disallowed_chars {
        for disallowed_char in disallowed_chars.chars() {
            allowed_chars = allowed_chars.replace(disallowed_char, "");
        }

        if allowed_chars.len() == 0 { // Make sure there are still characters left to use
            println!("Error: You have disallowed all characters");
            println!("The allowed character set includes: abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-=`~!@#$%^&*()_+[]\\{{}}|;':,./<>?");
            return;
        }
    }

    if let Some(new_password_length) = new_password_length { // Unwrap Option in i32
        let mut rng = rand::thread_rng();
        // Generate a random string of length n
        let mut new_password = String::new();
        for _ in 0..new_password_length {
            new_password.push(
                allowed_chars.chars()
                    .nth(rng.next_u32() as usize % allowed_chars.len())
                    .unwrap()
            );
        }

        // Print the new password
        println!("{}", new_password);
    }
}
