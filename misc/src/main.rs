fn main() {
    let now = chrono::Utc::now().naive_local();
    println!("{now}");
    let timestamp = now.format("%Y-%m-%d.%H-%M-%S").to_string();
    let nanos = now.timestamp_nanos_opt().unwrap_or(now.timestamp());
    println!("{timestamp}.{nanos}");
}
