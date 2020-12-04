use std::str::FromStr;

use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(passports: Vec<Passport>) -> Result<u64> {
    Ok(passports.iter().filter(|&p| p.is_valid()).count() as u64)
  }
}

#[derive(Default, Debug)]
pub struct Passport {
  birth_year: Option<u32>,
  issue_year: Option<u32>,
  expiration_year: Option<u32>,
  height: Option<String>,
  hair_color: Option<String>,
  eye_color: Option<String>,
  passport_id: Option<String>,
  country_id: Option<String>,
}

impl FromStr for Passport {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut passport = Passport::default();
    for part in s.split_whitespace() {
      let mut split = part.split(":");
      let (key, value) = (split.next().ok_or(())?, split.next().ok_or(())?);
      match key {
        "byr" => passport.birth_year = Some(value.parse().unwrap()),
        "iyr" => passport.issue_year = Some(value.parse().unwrap()),
        "eyr" => passport.expiration_year = Some(value.parse().unwrap()),
        "hgt" => passport.height = Some(value.to_string()),
        "hcl" => passport.hair_color = Some(value.to_string()),
        "ecl" => passport.eye_color = Some(value.to_string()),
        "pid" => passport.passport_id = Some(value.to_string()),
        "cid" => passport.country_id = Some(value.to_string()),
        _ => { return Err(()); },
      };
    }
    Ok(passport)
  }
}

impl Passport {
  pub fn is_valid(&self) -> bool {
    self.birth_year.is_some() &&
    self.issue_year.is_some() &&
    self.expiration_year.is_some() &&
    self.height.is_some() &&
    self.hair_color.is_some() &&
    self.eye_color.is_some() &&
    self.passport_id.is_some()
  }
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;
  use super::*;

  #[test]
  fn parse_success() {
    assert_matches!("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm".parse(), Ok(Passport { .. }));
    assert_matches!("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929".parse(), Ok(Passport { .. }));
    assert_matches!("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm".parse(), Ok(Passport { .. }));
    assert_matches!("hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in".parse(), Ok(Passport { .. }));
  }

  #[test]
  fn validity_tests() {
    assert!("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm".parse::<Passport>().unwrap().is_valid());
    assert!("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm".parse::<Passport>().unwrap().is_valid());
    assert_eq!(false, "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929".parse::<Passport>().unwrap().is_valid());
    assert_eq!(false, "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in".parse::<Passport>().unwrap().is_valid());
  }
}