#[tokio::main]
async fn main() {
    let threads = 1000;
    let mut oks = 0;
    loop {
        let mut handles = vec![];
        for _ in 0..threads {
            handles.push(tokio::task::spawn(reqwest::get("http://localhost:8080")));
        }
        for handle in handles {
            match handle.await {
                Ok(Ok(_)) => {
                    println!("Ok(Ok(_))");
                }
                Ok(Err(e)) => {
                    println!("Ok(Err({e}))");
                }
                Err(_) => {
                    println!("Err(_)");
                }
            }
        }
    }
}
