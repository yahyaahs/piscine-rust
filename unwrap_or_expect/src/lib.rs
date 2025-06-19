
pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::Unknown=> server.unwrap().to_string(),
        Security::NotFound => match server {
            Ok(url)=> url.to_string(),
            Err(e)=> format!("Not found: {}", e),
            
        }
        Security::UnexpectedUrl => server.unwrap_err().to_string(),
        Security::Message=> server.expect("ERROR : program stops").to_string(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
            println!("{}", fetch_data(Ok("server1.com"), Security::Warning));
    println!("{}", fetch_data(Err("server.com"), Security::Warning));
    println!("{}", fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl));

    fetch_data(Err("ERROR CRITICAL"), Security::Unknown);

    fetch_data(Err("server.com"), Security::Message);

    fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
    }
}
