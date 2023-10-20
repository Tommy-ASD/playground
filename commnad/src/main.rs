use std::{
    ffi::{OsStr, OsString},
    process::Command,
};

use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() {
    let mut cmd = Command::new("cmd");
    cmd.arg("/c")
        .arg("start")
        .raw_arg("\"\"")
        .raw_arg(wrap_in_quotes("https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id=288065419406-gies4ktpnqomngu6p73e9r5rptm1mqlq.apps.googleusercontent.com&state=Y14n_rJxRCN30VKOoRXCAw&code_challenge=FvZ6VG1UkDl7AH4mlJ3BVqqHEdQzyUA8rPWOWsKD4JM&code_challenge_method=S256&redirect_uri=http%3A%2F%2Flocalhost%3A13425&scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.email+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fuserinfo.profile+https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fyoutube.upload+openid"))
        .creation_flags(CREATE_NO_WINDOW);
    cmd.spawn();
}

fn wrap_in_quotes<T: AsRef<OsStr>>(path: T) -> OsString {
    let mut result = OsString::from("\"");
    result.push(path);
    result.push("\"");

    result
}
