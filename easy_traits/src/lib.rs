#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;

    fn append_number(&mut self, nb_to_append: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self {
        self.value.push_str(&str_to_append);
        Self {
            value: self.value.clone(),
        }
    }
    fn append_number(&mut self, nb_to_append: f64) -> Self {
        let nb = nb_to_append as i64;
        let nb1 = nb.abs();
        self.value.push_str(&nb1.to_string());
        Self {
            value: self.value.clone(),
        }
    }
    fn remove_punctuation_marks(&mut self) -> Self {
        self.value = self
            .value
            .chars()
            .filter(|c| !c.is_ascii_punctuation())
            .collect();
        Self {
            value: self.value.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut str_aux = StringValue {
            value: String::from("hello"),
        };

        println!("Before append: {}", str_aux.value);

        str_aux.append_str(String::from(" there!"));
        println!("After append: {}", str_aux.value);

        str_aux.remove_punctuation_marks();
        println!("After removing punctuation: {}", str_aux.value);
        str_aux.append_number(-123.456);
        println!("After appending number: {}", str_aux.value);
    }
}
