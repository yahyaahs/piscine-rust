pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond = Vec::new();
    let size = c as usize - 'A' as usize + 1;

    for i in 0..size {
        let mut line = vec![' '; size * 2 - 1];
        line[size - 1 - i] = (b'A' + i as u8) as char;
        line[size - 1 + i] = (b'A' + i as u8) as char;
        diamond.push(line.iter().collect());
    }

    for i in (0..size - 1).rev() {
        let mut line = vec![' '; size * 2 - 1];
        line[size - 1 - i] = (b'A' + i as u8) as char;
        line[size - 1 + i] = (b'A' + i as u8) as char;
        diamond.push(line.iter().collect());
    }

    return diamond;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", get_diamond('A'));
        println!("{:?}", get_diamond('C'));
        for line in get_diamond('C') {
            println!("{}", line);
        }
    }
}