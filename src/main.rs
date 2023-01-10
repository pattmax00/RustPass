use std::env;

fn main() {
    // Take in two command line arguments, one i32 and one str
    let args: Vec<String> = env::args().collect();
    let new_password_length: i32 = args[1].parse().unwrap();
    let allowed_chars: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-=`~!@#$%^&*()_+[]\\{}|;':\",./<>?";

    // Generate a random string of length n
    let mut new_password = String::new();
    for _ in 0..new_password_length {
        new_password.push(
            allowed_chars.chars()
                .nth(rand::random::<usize>() % allowed_chars.len())
                .unwrap()
        );
    }

    // Print the new password
    println!("{}", new_password);
}
