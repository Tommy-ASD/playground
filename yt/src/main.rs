use rusty_ytdl::Video;

#[tokio::main]
async fn main() {
    let video_url = "https://www.youtube.com/watch?v=FZ8BxMU3BYc"; // FZ8BxMU3BYc works too!
    let video = Video::new(video_url).unwrap();

    // let stream = video.stream().await.unwrap();

    // while let Some(chunk) = stream.chunk().await.unwrap() {
    //     // Do what you want with chunks
    //     println!("{:?}", chunk);
    //     println!("{}", chunk.len());
    // }

    // Or direct download to path
    let path = std::path::Path::new(r"test.mp3");

    video.download(path).await.unwrap();

    // //
    // // Or with options
    // //

    // let video_options = VideoOptions {
    //     quality: VideoQuality::Lowest,
    //     filter: VideoSearchOptions::Audio,
    //     ..Default::default()
    // };

    // let video = Video::new_with_options(video_url, video_options).unwrap();

    // let stream = video.stream().await.unwrap();

    // while let Some(chunk) = stream.chunk().await.unwrap() {
    //     // Do what you want with chunks
    //     println!("{:#?}", chunk);
    // }

    // // Or direct download to path
    // let path = std::path::Path::new(r"test.mp3");

    // video.download(path).await.unwrap();
}
