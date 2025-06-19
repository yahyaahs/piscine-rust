pub use chrono::Utc;
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values :(&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        FormError{
            form_values : ((field_name, field_value)), 
            date : (Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            err : err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name : String,
    pub password : String,
}
pub fn pass_checker(pass : &str) -> bool{
    let mut lett = false;
    let mut number  = false;
    let mut punc = false;
    for c in pass.chars(){
        if c.is_ascii_alphabetic(){
            lett = true;
        }else if c.is_ascii_digit(){
            number = true;
        } else if c.is_ascii_graphic() || c.is_ascii_punctuation()&& !c.is_ascii_alphanumeric(){
            punc = true;
        }
    }
    lett && number && punc
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        let pass = self.password.clone();
        if self.name.is_empty(){
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));       
        } else if self.password.len() < 8 {
                        return Err(FormError::new("password", pass, "Password should be at least 8 characters long"));

        } else if !pass_checker(&self.password){
                        return Err(FormError::new("password", pass, "Password should be a combination of ASCII numbers, letters and symbols"));
        }
        return Ok(());
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
            let mut form_output = Form {
        name: "Lee".to_owned(),
        password: "qwqwsa1dty_".to_owned(),
    };

    println!("{:?}", form_output);
    println!("{:?}", form_output.validate());

    form_output.name = "".to_owned();
    println!("{:?}", form_output.validate());

    form_output.name = "as".to_owned();
    form_output.password = "dty_1".to_owned();
    println!("{:?}", form_output.validate());

    form_output.password = "asdasASd(_".to_owned();
    println!("{:?}", form_output.validate());

    form_output.password = "asdasASd123SA".to_owned();
    println!("{:?}", form_output.validate());
    }
}
