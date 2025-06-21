pub mod edit_distance;
use edit_distance::edit_distance;
pub fn expected_variable(first: &str, sec: &str) -> Option<String> {
    let steps = edit_distance(&first.to_lowercase(), &sec.to_lowercase());
  if first.contains(" ")||!first.contains("_") && !sec.chars().nth(0).unwrap().is_uppercase()&& steps==0 {
        return None;
    }
    println!("these are the steps {}", steps);
    let calc = (sec.len() as f64 - steps as f64) as f64 / sec.len() as f64 * 100.;
    if calc < 50. {
        return None;
    }
    return Some(format!("{}%", calc.round().to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
    println!(
        "{} close to it",
        expected_variable("On_Point", "on_point").unwrap()
    );
    println!(
        "{} close to it",
        expected_variable("soClose", "so_close").unwrap()
    );
    println!(
        "{:?}",
        expected_variable("something", "something_completely_different")
    );
    println!(
        "{} close to it",
        expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
    );
    }
}

