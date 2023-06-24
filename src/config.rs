use color_eyre::eyre::{self, eyre};
use std::fs::File;
use std::io;

fn read_conf_from(file_name: &str) -> eyre::Result<Vec<String>> {
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => return Err(eyre!("could not open config file {file_name}: {e}")),
    };
    let conf: String = match io::read_to_string(file) {
        Ok(s) => s,
        Err(e) => return Err(eyre!("problem reading config: {e}")),
    };
    let channels = conf.lines().map(|x| x.to_string()).collect();
    Ok(channels)
}

pub fn read_conf() -> eyre::Result<Vec<String>> {
    read_conf_from("channels.txt")
}

#[cfg(test)]
mod tests {
    use super::read_conf_from;

    #[test]
    fn test_conf() {
        let channels = read_conf_from("test_channels.txt").unwrap();
        assert_eq!(&channels, &["Dawgos", "aboba", "Alison"]);
    }
}
