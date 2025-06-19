use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    short_hand : String,
    long_hand: String,
    desc : String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Flag {
            short_hand : format!("-{}", name.chars().next().unwrap()),
            long_hand: format!("--{}", name),
            desc : d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
        self.flags.insert(flag.desc, func);

    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        match self.flags.get(input){
            Some(func)=> match func(argv[0], argv[1]){
                Ok(res)=> Ok(res),
                Err(_)=> Err("invalid float literal".to_string()),
            },
            None=> Err("invalid float literal".to_string()),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    match (a.parse::<f64>(), b.parse::<f64>()){
        (Ok(one), Ok(two))=> Ok((one/two).to_string()),
         (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    match (a.parse::<f64>(), b.parse::<f64>()){
        (Ok(one), Ok(two))=> Ok((one%two).to_string()),
        (Err(e), _) | (_, Err(e)) => Err(e),
    }}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
                let mut handler = FlagsHandler {
            flags: HashMap::new(),
        };

        let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
        let r = Flag::opt_flag(
            "remainder",
            "remainder of the division between two values, formula (a % b)",
        );

        handler.add_flag(d, div);
        handler.add_flag(r, rem);

        println!("{:?}", handler.exec_func("-d", &["1.0", "2.0"]));

        println!("{:?}", handler.exec_func("-r", &["2.0", "2.0"]));

        println!("{:?}", handler.exec_func("--division", &["a", "2.0"]));

        println!("{:?}", handler.exec_func("--remainder", &["2.0", "fd"]));
    }
}
