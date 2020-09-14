use std::process::Command;

pub fn get_tty_cols() -> usize {
    use std::str;

    let cmd = Command::new("tput")
        .arg("cols")
        .output()
        .expect("failed to run tput proses");

    if cmd.status.success() {
        str::from_utf8(&cmd.stdout)
            .expect("cant cast output as str ")
            .trim()
            .parse::<usize>()
            .expect("cant convert output to a number")
    } else {
        panic!("tput failed to run");
    }
}
