pub fn num_to_ordinal(x: u32) -> String {

    let suff = ["th","st","nd","rd","th","th","th","th","th","th"];
    if ((x%100)>=11) && ((x%100)<=13){
        return format!("{}th", x);
    }
    return format!("{}{}", x, suff[(x%10)as usize].to_string());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
     println!("{}", num_to_ordinal(1));
    println!("{}", num_to_ordinal(22));
    println!("{}", num_to_ordinal(43));
    println!("{}", num_to_ordinal(47));
    }
}