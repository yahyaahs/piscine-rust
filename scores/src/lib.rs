pub fn score(s: &str) -> u64 {
    let mut sum = 0;
    let _ : Vec<_>= s.chars().map(|c| match c {
        'a' | 'e' | 'i' | 'o' |'u'| 'l' | 'n' | 'r' | 's' | 't' => sum += 1,
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => sum += 1,
        'D' | 'G' => sum += 2,
        'd'| 'g'=> sum += 2,
        'b'|'c'|'m'|'p'=>sum+=3,
        'B'|'C'|'M'|'P'=>sum+=3,
        'F'|'H'|'V'|'W'|'Y'=> sum+=4,
        'f'|'h'|'v'|'w'|'y'=> sum+=4,
        'K'|'k'=>sum+=5,
        'J'|'X'|'j'|'x'=> sum+=8,
        'Q'|'Z'|'q'|'z' =>sum+=10,
        _=>sum+=0,
    }).collect();
    return sum;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    println!("{}", score("a"));
    println!("{}", score("ã ê Á?"));
    println!("{}", score("ThiS is A Test"));
    }
}
