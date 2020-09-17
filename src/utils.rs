use std::process::Command;
use std::{env, error::Error, path::PathBuf};

/// get the terminal columns from the tput command
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

// try and use either XDG_DATA_HOME or HOME else return an error
pub fn get_data_path() -> Result<PathBuf, Box<dyn Error>> {
    if let Ok(data_home) = env::var("XDG_DATA_HOME") {
        if data_home.is_empty() {
            return Err(Box::from(
                "XDG_DATA_HOME not set correctly and is empty",
            ));
        }

        Ok(PathBuf::from(data_home).join("words_cli"))
    } else if let Ok(val) = env::var("HOME") {
        if val.is_empty() {
            return Err(Box::from("HOME not set correctly and is empty"));
        }

        Ok(PathBuf::from(val)
            .join(".local")
            .join("share")
            .join("words_cli"))
    } else {
        Err(Box::from(
            "environment variables HOME and XDG_DATA_HOME are not set",
        ))
    }
}

pub fn get_user_config_path() -> Result<PathBuf, Box<dyn Error>> {
    if let Ok(config_home) = env::var("XDG_CONFIG_HOME") {
        if config_home.is_empty() {
            return Err(Box::from(
                "XDG_CONFIG_HOME not set correctly and is empty",
            ));
        }

        Ok(PathBuf::from(config_home).join("words_cli"))
    } else if let Ok(val) = env::var("HOME") {
        if val.is_empty() {
            return Err(Box::from("HOME not set correctly and is empty"));
        }

        Ok(PathBuf::from(val).join(".config").join("words_cli"))
    } else {
        Err(Box::from(
            "environment variables HOME and XDG_CONFIG_HOME are not set",
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_cache_path() {
        env::set_var("XDG_DATA_HOME", "test");

        let path = get_data_path().unwrap();

        assert_eq!(
            path,
            PathBuf::from("test/words_cli"),
            "did not make path correctly"
        )
    }

    #[test]
    fn test_get_cache_path_no_var_data() {
        env::set_var("XDG_DATA_HOME", "");

        if let Err(err) = get_data_path() {
            assert_eq!(
                err.to_string(),
                "XDG_DATA_HOME not set correctly and is empty",
                "got a different error then expected "
            )
        } else {
            assert!(false, "some how got data in env var")
        }
    }
}
