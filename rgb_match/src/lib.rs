#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut m: (&str, &str) = ("", "");
        if self.r == first {
            m.0 = "r";
        }
        if self.g == first {
            m.0 = "g";
        }
        if self.b == first {
            m.0 = "b";
        }
        if self.a == first {
            m.0 = "a";
        }
        if self.r == second {
            m.1 = "r";
        }
        if self.g == second {
            m.1 = "g";
        }
        if self.b == second {
            m.1 = "b";
        }
        if self.a == second {
            m.1 = "a";
        }
        let cl = match m {
            ("r", "g") | ("g", "r") => {
                let (r, g) = (self.g, self.r);
                Color {
                    r,
                    g,
                    b: self.b,
                    a: self.a,
                }
            }
            ("r", "b") | ("b", "r") => {
                let (r, b) = (self.b, self.r);
                Color {
                    r,
                    g: self.g,
                    b,
                    a: self.a,
                }
            }
            ("r", "a") | ("a", "r") => {
                let (r, a) = (self.a, self.r);
                Color {
                    r,
                    g: self.g,
                    b: self.b,
                    a,
                }
            }
            ("g", "b") | ("b", "g") => {
                let (g, b) = (self.b, self.g);
                Color {
                    r: self.r,
                    g,
                    b,
                    a: self.a,
                }
            }
            ("g", "a") | ("a", "g") => {
                let (g, a) = (self.a, self.g);
                Color {
                    r: self.r,
                    g,
                    b: self.b,
                    a,
                }
            }
            ("b", "a") | ("a", "b") => {
                let (b, a) = (self.a, self.b);
                Color {
                    r: self.r,
                    g: self.g,
                    b,
                    a,
                }
            }
            _ => self,
        };

        return cl;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let c = Color {
        r: 255,
        g: 200,
        b: 10,
        a: 30,
    };

    println!("{:?}", c.swap(c.r, c.b));
    println!("{:?}", c.swap(c.r, c.g));
    println!("{:?}", c.swap(c.r, c.a));
    println!();
    println!("{:?}", c.swap(c.g, c.r));
    println!("{:?}", c.swap(c.g, c.b));
    println!("{:?}", c.swap(c.g, c.a));
    println!();
    println!("{:?}", c.swap(c.b, c.r));
    println!("{:?}", c.swap(c.b, c.g));
    println!("{:?}", c.swap(c.b, c.a));
    println!();
    println!("{:?}", c.swap(c.a, c.r));
    println!("{:?}", c.swap(c.a, c.b));
    println!("{:?}", c.swap(c.a, c.g));
    }
}