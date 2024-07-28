use youtube_dl::YoutubeDl;

#[tokio::main]
async fn main() {
    let ytdl = YoutubeDl::new("https://www.youtube.com/watch?v=on4IoQ2MQ7M")
        .extra_arg("-f bestvideo[ext=mp4]+bestaudio[ext=m4a]")
        .extra_arg("-k")
        .output_template("%(ID)s")
        .download_to_async("./downloads")
        .await
        .unwrap();
}
