use ping::ping;
use std::net::IpAddr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let base_ip = "192.168.0.";
    let mut threads = vec![];

    for i in 0..=254 {
        let base_ip = base_ip.to_string();
        let thread = std::thread::spawn(move || {
            let ip_str = format!("{}{}", base_ip, i);
            let ip: IpAddr = ip_str.parse().expect("Invalid IP address format");

            match ping(ip, Some(Duration::from_secs(1)), None, None, None, None) {
                Ok(()) => println!("{} is up", ip_str),
                Err(_) => {
                    //format!("{} is down", ip_str)
                }
            };
            // sleep(Duration::from_secs(i));
        });
        threads.push(thread);
    }

    for thread in threads {
        thread.join().expect("Thread panicked");
    }
}
