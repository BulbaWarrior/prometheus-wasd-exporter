use color_eyre::eyre;
use std::fs::File;
use std::io;

fn read_conf_from(file_name: &str) -> eyre::Result<Vec<String>> {
    let file = File::open(file_name)?;
    let conf: String = io::read_to_string(file)?;
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
