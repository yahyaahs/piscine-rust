#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }

    }
}

impl FromStr for RhFactor {
        type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s{
            "Positive" => Ok(RhFactor::Positive),
            "Negative" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}


impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "A+" => Ok(BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            }),
            "A-" => Ok(BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            }),
            "B+" => Ok(BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            }),
            "B-" => Ok(BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            }),
            "AB+" => Ok(BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            }),
            "AB-" => Ok(BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            }),
            "O+" => Ok(BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            }),
            "O-" => Ok(BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            }),
            _ => Err(()),
        }
    }

}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let rh_factor_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen_str, rh_factor_str)
    }
}

impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
        let ant =match (&self.antigen, &other.antigen) {
            (Antigen::A, Antigen::A) => true,
            (Antigen::B, Antigen::B) => true,
            (Antigen::AB, _) => true,
            (Antigen::O, Antigen::O) => true,
            (_, Antigen::O) => true,
            _ => false,
        }; 
        let fac = match (&self.rh_factor, &other.rh_factor) {
            (RhFactor::Positive, RhFactor::Positive) => true,
            (RhFactor::Positive, RhFactor::Negative) => true,
            (RhFactor::Negative, RhFactor::Negative) => true,
            _ => false,
        };
        fac&& ant
	}

	pub fn donors(&self) -> Vec<Self> {
        let mut donors = Vec::new();
        for antigen in [Antigen::A, Antigen::B, Antigen::AB, Antigen::O].iter() {
            for rh in [RhFactor::Positive, RhFactor::Negative].iter() {
                let donor = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                };
                if self.can_receive_from(&donor) {
                    donors.push(donor);
                }
            }
        }
        donors
	}

	pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();
        for antigen in [Antigen::A, Antigen::B, Antigen::AB, Antigen::O].iter() {
            for rh in [RhFactor::Positive, RhFactor::Negative].iter() {
                let recipient = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                };
                if recipient.can_receive_from(self) {
                    recipients.push(recipient);
                }
            }
        }
        recipients

    }
}
impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            ord => ord,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn compatible_ab_neg_with_a_pos() {
    let blood_type: BloodType = "AB-".parse().unwrap();
    let other_bt: BloodType = "A+".parse().unwrap();
    assert!(!blood_type.can_receive_from(&other_bt));
}

#[test]
fn compatible_a_neg_with_a_pos() {
    let blood_type: BloodType = "A-".parse().unwrap();
    let other_bt: BloodType = "A+".parse().unwrap();
    assert!(!blood_type.can_receive_from(&other_bt));
}

#[test]
fn compatible_a_neg_with_ab_neg() {
    let blood_type: BloodType = "AB-".parse().unwrap();
    let other_bt: BloodType = "A-".parse().unwrap();
    assert!(blood_type.can_receive_from(&other_bt));
}

#[test]
fn compatible_ab_neg_with_o_pos() {
    let blood_type: BloodType = "AB-".parse().unwrap();
    let other_bt: BloodType = "O+".parse().unwrap();
    assert!(!blood_type.can_receive_from(&other_bt));
}

#[test]
fn compatible_ab_pos_with_o_pos() {
    let blood_type: BloodType = "AB+".parse().unwrap();
    let other_bt: BloodType = "O+".parse().unwrap();
    assert!(blood_type.can_receive_from(&other_bt));
}

#[test]
fn test_compatible_ab_neg_with_o_neg() {
    let blood_type: BloodType = "AB-".parse().unwrap();
    let other_bt: BloodType = "O-".parse().unwrap();
    assert!(blood_type.can_receive_from(&other_bt));
}

#[test]
fn test_antigen_ab_from_str() {
    let blood = "AB+";
    let blood_type: BloodType = blood.parse().unwrap();
    assert_eq!(blood_type.antigen, Antigen::AB);
    assert_eq!(blood_type.rh_factor, RhFactor::Positive);
}

#[test]
fn test_antigen_a_from_str() {
    let blood = "A-";
    let blood_type = blood.parse::<BloodType>().unwrap();
    assert_eq!(blood_type.antigen, Antigen::A);
    assert_eq!(blood_type.rh_factor, RhFactor::Negative);
}

#[test]
#[should_panic]
fn test_unexistent_blood_type() {
    let _blood_type: BloodType = "AO-".parse().unwrap();
}

#[test]
fn test_donors() {
    let mut givers = "AB+".parse::<BloodType>().unwrap().donors();
    println!("Before sorting {:?}", &givers);
    givers.sort();
    println!("{:?}", &givers);
    let mut expected = vec![
        "AB-".parse::<BloodType>().unwrap(),
        "A-".parse().unwrap(),
        "B-".parse().unwrap(),
        "O-".parse().unwrap(),
        "AB+".parse().unwrap(),
        "A+".parse().unwrap(),
        "B+".parse().unwrap(),
        "O+".parse().unwrap(),
    ];
    expected.sort();
    assert_eq!(givers, expected);
}

#[test]
fn test_a_neg_donors() {
    let mut givers = "A-".parse::<BloodType>().unwrap().donors();
    givers.sort();
    let mut expected: Vec<BloodType> = vec!["A-".parse().unwrap(), "O-".parse().unwrap()];
    expected.sort();
    assert_eq!(givers, expected);
}

#[test]
fn test_o_neg_donors() {
    let mut givers = "O-".parse::<BloodType>().unwrap().donors();
    givers.sort();
    let mut expected: Vec<BloodType> = vec!["O-".parse().unwrap()];
    expected.sort();
    assert_eq!(givers, expected);
}

#[test]
fn test_ab_pos_recipients() {
    let mut recipients: Vec<BloodType> = "AB+".parse::<BloodType>().unwrap().recipients();
    recipients.sort();
    let mut expected: Vec<BloodType> = vec!["AB+".parse().unwrap()];
    expected.sort();
    assert_eq!(recipients, expected);
}

#[test]
fn test_a_neg_recipients() {
    let mut recipients = "A-".parse::<BloodType>().unwrap().recipients();
    recipients.sort();
    let mut expected: Vec<BloodType> = vec![
        "A-".parse().unwrap(),
        "AB+".parse().unwrap(),
        "A+".parse().unwrap(),
        "AB-".parse().unwrap(),
    ];
    expected.sort();
    assert_eq!(recipients, expected);
}

#[test]
fn test_output() {
    let blood_type: BloodType = "O+".parse().unwrap();
    println!("recipients of O+ {:?}", blood_type.recipients());
    println!("donors of O+ {:?}", blood_type.donors());
    let another_blood_type: BloodType = "A-".parse().unwrap();
    println!(
        "donors of O+ can receive from {:?} {:?}",
        &another_blood_type,
        blood_type.can_receive_from(&another_blood_type)
    );
}
}
