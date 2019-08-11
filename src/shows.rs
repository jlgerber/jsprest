use std::fs::read_dir;

/// Rewtrieve the list of shows from disk
pub(crate) fn get_showlist() -> Vec<String> {
    let r: Vec<String> = vec![];
    let result = match read_dir("/dd/shows") {
        Ok(readdir) => readdir
            .into_iter()
            // TODO: error handling for unwraps
            .map(|r| r.unwrap().file_name().to_str().unwrap().to_string())
            .collect::<Vec<String>>(),
        Err(_) => r,
    };
    result
}