use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::fs;
pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let _ = match OpenOptions::new().append(true).create(true).open(path){
        Ok(mut file)=> file.write_all(content.as_bytes()),
        Err(_)=> panic!(),
    };
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
