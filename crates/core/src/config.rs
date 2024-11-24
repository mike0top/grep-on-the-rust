///# This is struct `Config`.
pub struct Config {
    pub flags: Vec<String>,
    pub query: String,
    pub file_path: String,
}

///## This is implementation `Config`.
impl Config {
    //!### This is attribute `build`.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let mut flags = Vec::new();

        while let Some(arg) = args.next() {
            if arg.starts_with('-') {
                flags.push(arg.clone());
                if flags.contains(&"-help".to_string()) || flags.contains(&"-version".to_string()) {
                    return Ok(Config {
                        flags,
                        query: "".to_string(),
                        file_path: "".to_string(),
                    });
                }
            } else {
                let query = arg;

                let file_path = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get a file path"),
                };

                return Ok(Config {
                    flags,
                    query,
                    file_path,
                });
            }
        }

        Err("No query or file path provided")
    }
}
