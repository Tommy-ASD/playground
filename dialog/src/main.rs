use native_dialog::{FileDialog, MessageDialog, MessageType};

fn main() {
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Folder", &["png"])
        .show_open_single_dir()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return,
    };

    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Do you want to open the file?")
        .set_text(&format!("{:#?}", path))
        .show_confirm()
        .unwrap();

    if yes {
        println!("{:?}", path);
    }
}
