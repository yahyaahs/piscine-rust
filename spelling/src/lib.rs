
fn under_thousand(n: u64) -> String {
    let hundreds = n / 100;
    let rest = n % 100;

    if hundreds == 0 {
        return under_hundred(rest);
    }

    if rest == 0 {
        format!("{} hundred", get_unit(hundreds))
    } else {
        format!("{} hundred {}", get_unit(hundreds), under_hundred(rest))
    }
}

fn under_hundred(n: u64) -> String {
    if n < 20 {
        get_unit(n)
    } else {
        let t = n / 10;
        let u = n % 10;

        if u == 0 {
            get_ten(t)
        } else {
            format!("{}-{}", get_ten(t), get_unit(u))
        }
    }
}

fn get_unit(n: u64) -> String {
    let units : Vec<String>= vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven",
        "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen",
        "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ].into_iter().map(|str| str.to_string()).collect();
    units.get(n as usize).unwrap_or(&"".to_string()).to_string()
}

fn get_ten(n: u64) -> String {
    let tens: Vec<String> =   vec![
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ].into_iter().map(|str| str.to_string()).collect();
    tens.get(n as usize).unwrap_or(&"".to_string()).to_string()
}


pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let million = (n % 1_000_000_000) / 1_000_000;
    let thousand = (n % 1_000_000) / 1_000;
    let remainder = n % 1_000;

    let mut parts = Vec::new();

 
    if million > 0 {
        parts.push(format!("{} million", under_thousand(million)));
    }

    if thousand > 0 {
        parts.push(format!("{} thousand", under_thousand(thousand)));
    }

    if remainder > 0 {
        parts.push(under_thousand(remainder));
    }

    parts.join(" ")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    println!("{}", spell(1000000000));
    println!("{}", spell(9996));
    }
}
