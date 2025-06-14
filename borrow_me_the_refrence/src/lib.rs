pub fn delete_and_backspace(s: &mut String) {
    let mut new = vec![];
    let mut i = 0;
    for c in s.chars(){
        if c != '+' && c!= '-'{
            if i == 0 {
                new.push(c);
            }else {
                i-=1;
                continue;
            }
        }
        if c == '-'{
            new.pop();
        }
        if c =='+'{
            i += 1;
        }
    }
    *s = new.iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for op in v {
        let mut sum = 0;
        if op.contains("+"){
            let nums = op.split("+");
            for n in nums {
                sum+= n.parse::<i32>().unwrap();
            }
        }else if op.contains("-") {
            let mut nums = op.split("-");
            sum = nums.nth(0).unwrap().parse::<i32>().unwrap() - nums.nth(0).unwrap().parse::<i32>().unwrap();
        }
        *op = sum.to_string();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
            let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
    let mut b = [
        "2+2".to_owned(),
        "3+2".to_owned(),
        "10-3".to_owned(),
        "5+5".to_owned(),
    ];

    // delete_and_backspace(&mut a);
    do_operations(&mut b);

    println!("{:?}", (a, b));
    }
}
