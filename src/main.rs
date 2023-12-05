use std::process::Command;

fn main() {
    // Define the command to run (`ls` in this case)
    let mut cmd = Command::new("ls");

    // Execute the command and get the output
    match cmd.output() {
        Ok(output) => {
            // Check if the command was successful
            if output.status.success() {
                // Convert the output to a String and print it
                let output_str = String::from_utf8_lossy(&output.stdout);
                println!("{}", output_str);
            } else {
                // If the command failed, handle the error (e.g., print an error message)
                eprintln!("Command executed with failing error code");
            }
        }
        Err(error) => {
            // Handle the case where the command couldn't be executed (e.g., `ls` not found)
            eprintln!("An error occurred when running the command: {}", error);
        }
    }
}
