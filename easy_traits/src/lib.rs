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
    let s = if nb_to_append.fract() == 0.0 {
        format!("{}", nb_to_append)
    } else if self.value.contains("-") {
        format!("{}", nb_to_append)
    } else {
        format!("{}", nb_to_append)
    };

    self.value.push_str(&s);
    
    Self {
        value: self.value.clone(),
    }
}

    fn remove_punctuation_marks(&mut self) -> Self {
        self.value = self
            .value
            .chars()
            .filter(|c| *c !='.'&& *c!=','&& *c!='?' && *c != '!')
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
    }
      #[test]
    fn test_append_number() {
        let mut str_aux = StringValue {
            value: String::from(""),
        };

        assert_eq!(String::from("-1"), str_aux.append_number(-1.0).value);

        assert_eq!(String::from("-15"), str_aux.append_number(5.0).value);

        assert_eq!(String::from("-155.5"), str_aux.append_number(5.5).value);

        assert_eq!(
            String::from("-1555"),
            str_aux.remove_punctuation_marks().value
        );
    }
}
