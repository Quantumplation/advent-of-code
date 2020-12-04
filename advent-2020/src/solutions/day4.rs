use std::str::FromStr;

use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(_passports: Vec<Passport>) -> Result<u64> {
    bail!("Part 1 solution incompatible with Part 2");
  }
}

pub mod part2 {
  use super::*;
  pub fn solve(passports: Vec<Passport>) -> Result<u64> {
    Ok(passports.iter().filter(|&p| p.is_valid()).count() as u64)
  }
}

#[derive(Debug)]
pub enum Length {
  In(u32),
  Cm(u32)
}

impl FromStr for Length {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let unit = s.len() - 2;
    let (value, unit) = (s[..unit].parse::<u32>(), &s[unit..]);
    match unit {
      "in" => {
        value.map(|l| Length::In(l)).map_err(|_| ())
      },
      "cm" => {
        value.map(|l| Length::Cm(l)).map_err(|_| ())
      }
      _ => Err(())
    }
  }
}


#[derive(Debug)]
pub struct HairColor(String);
impl FromStr for HairColor {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.starts_with("#") && s[1..].len() == 6 && s[1..].chars().all(char::is_alphanumeric) {
      Ok(HairColor(s.to_string()))
    } else {
      Err(())
    }
  }
}

#[derive(Debug)]
pub enum EyeColor {
  Amber, Blue, Brown, Grey, Green, Hazel, Other
}

impl FromStr for EyeColor {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "amb" => Ok(EyeColor::Amber),
      "blu" => Ok(EyeColor::Blue),
      "brn" => Ok(EyeColor::Brown),
      "gry" => Ok(EyeColor::Grey),
      "grn" => Ok(EyeColor::Green),
      "hzl" => Ok(EyeColor::Hazel),
      "oth" => Ok(EyeColor::Other),
      _ => Err(())
    }
  }
}

#[derive(Default, Debug)]
pub struct Passport {
  birth_year: Option<u32>,
  issue_year: Option<u32>,
  expiration_year: Option<u32>,
  height: Option<Length>,
  hair_color: Option<HairColor>,
  eye_color: Option<EyeColor>,
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
        "byr" => passport.birth_year = value.parse().map_err(|_| ()).ok(),
        "iyr" => passport.issue_year = value.parse().map_err(|_| ()).ok(),
        "eyr" => passport.expiration_year = value.parse().map_err(|_| ()).ok(),
        "hgt" => passport.height = value.parse().map_err(|_| ()).ok(),
        "hcl" => passport.hair_color = value.parse().map_err(|_| ()).ok(),
        "ecl" => passport.eye_color = value.parse().map_err(|_| ()).ok(),
        "pid" => passport.passport_id = value.parse().map_err(|_| ()).ok(),
        "cid" => passport.country_id = value.parse().map_err(|_| ()).ok(),
        _     => { return Err(()); },
      };
    }
    Ok(passport)
  }
}

impl Passport {
  pub fn is_valid(&self) -> bool {
    matches!(self.birth_year, Some(x) if x >= 1920 && x <= 2002) &&
    matches!(self.issue_year, Some(x) if x >= 2010 && x <= 2020) &&
    matches!(self.expiration_year, Some(x) if x >= 2020 && x <= 2030) &&
    (
      matches!(self.height, Some(Length::In(x)) if x >= 59 && x <= 76) ||
      matches!(self.height, Some(Length::Cm(x)) if x >= 150 && x <= 193)
    ) &&
    matches!(self.hair_color, Some(_)) &&
    matches!(self.eye_color, Some(_)) &&
    matches!(self.passport_id.clone(), Some(x) if x.len() == 9)
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
    assert!("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f".parse::<Passport>().unwrap().is_valid());
    assert!("eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".parse::<Passport>().unwrap().is_valid());
    assert!("hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022".parse::<Passport>().unwrap().is_valid());
    assert!("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".parse::<Passport>().unwrap().is_valid());

    assert_eq!(false, "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".parse::<Passport>().unwrap().is_valid());
    assert_eq!(false, "iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946".parse::<Passport>().unwrap().is_valid());
    assert_eq!(false, "hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".parse::<Passport>().unwrap().is_valid());
    assert_eq!(false, "hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007".parse::<Passport>().unwrap().is_valid());
  }
}