use std::process::Command;
use std::thread;

fn main() {
    let n = 16;

    let command = "iperf";
    let args = ["-c", "tommyasd.com"];

    let mut handles = vec![];

    for _ in 0..n {
        let handle = thread::spawn(move || {
            let output = Command::new(command)
                .args(&args)
                .output()
                .expect("Failed to execute command");

            println!("{}", String::from_utf8_lossy(&output.stdout));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
