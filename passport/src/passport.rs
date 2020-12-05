use regex::Regex;

const VALID_EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Debug)]
pub struct Passport {
    birth_year: Option<i32>,
    issue_year: Option<i32>,
    expiration_year: Option<i32>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    pub fn build(input: &str) -> Passport {
        let mut passport = Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        };
        let tokens = input.split_ascii_whitespace();
        tokens.for_each(|token| {
            let mut split = token.split(':');
            let key = split.next().unwrap();
            let value = split.next().unwrap();
            match key {
                "byr" => {
                    match Passport::string_to_date(value) {
                        Some(date) => {
                            if date >= 1920 && date <= 2020 {
                                passport.birth_year = Some(date)
                            }
                        }
                        None => {}
                    }
                }
                "iyr" => {
                    match Passport::string_to_date(value) {
                        Some(date) => {
                            if date >= 2010 && date <= 2020 {
                                passport.issue_year = Some(date)
                            }
                        }
                        None => {}
                    }
                }
                "eyr" => {
                    match Passport::string_to_date(value) {
                        Some(date) => {
                            if date >= 2020 && date <= 2030 {
                                passport.expiration_year = Some(date)
                            }
                        }
                        None => {}
                    }
                }
                "hgt" => {
                    let re = Regex::new(r"^(\d+)(cm|in)").unwrap();
                    match re.captures(value) {
                        Some(captures) => {
                            match &captures[2] {
                                "cm" => {
                                    match captures[1].parse::<i32>() {
                                        Ok(height) => {
                                            if height >= 150 && height <= 193 {
                                                passport.height = Some(value.to_string())
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                                "in" => {
                                    match captures[1].parse::<i32>() {
                                        Ok(height) => {
                                            if height >= 59 && height <= 76 {
                                                passport.height = Some(value.to_string())
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        None => {}
                    }
                }
                "hcl" => {
                    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                    match re.captures(value) {
                        Some(capture) => { passport.hair_color = Some(value.to_string()) }
                        None => {}
                    }
                }
                "ecl" => {
                    if VALID_EYE_COLORS.contains(&value) {
                        passport.eye_color = Some(value.to_string())
                    }
                }
                "pid" => passport.passport_id = Some(value.to_string()),
                "cid" => passport.country_id = Some(value.to_string()),
                _ => {}
            }
        });
        passport
    }

    fn string_to_date(input: &str) -> Option<i32> {
        if input.len() == 4 {
            input.parse::<i32>().ok()
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        return self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.eye_color.is_some()
            && self.hair_color.is_some()
            && self.passport_id.is_some();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ENTRY: &'static str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";

    struct PassportStringBuilder<'a> {
        birth_year: &'a str,
        issue_year: &'a str,
        expiration_year: &'a str,
        height: &'a str,
        hair_color: &'a str,
        eye_color: &'a str,
        passport_id: &'a str,
        country_id: &'a str,
    }

    impl PassportStringBuilder<'_> {
        fn new<'a>() -> PassportStringBuilder<'a> {
            PassportStringBuilder {
                birth_year: "1920",
                issue_year: "2010",
                expiration_year: "2020",
                height: "150cm",
                hair_color: "#123456",
                eye_color: "amb",
                passport_id: "123456789",
                country_id: "any",
            }
        }
    }

    impl ToString for PassportStringBuilder<'_> {
        fn to_string(&self) -> String {
            format!("byr:{} iyr:{} eyr:{} hgt:{} hcl:{} ecl:{} pid:{} cid:{}",
                    self.birth_year,
                    self.issue_year,
                    self.expiration_year,
                    self.height,
                    self.hair_color,
                    self.eye_color,
                    self.passport_id,
                    self.country_id)
        }
    }

    #[test]
    fn validate_birth_year() {
        let passport = Passport::build(VALID_ENTRY);
        assert_eq!(1937, passport.birth_year.unwrap());
    }

    #[test]
    fn validate_issue_year() {
        let passport = Passport::build(VALID_ENTRY);
        assert_eq!(2017, passport.issue_year.unwrap());
    }

    #[test]
    fn validate_expiration_year() {
        let passport = Passport::build(VALID_ENTRY);
        assert_eq!(2020, passport.expiration_year.unwrap());
    }

    #[test]
    fn validate_height() {
        let passport = Passport::build(VALID_ENTRY);
        assert_eq!("183cm", passport.height.unwrap());
    }

    #[test]
    fn validate_hair_color() {
        let passport = Passport::build(VALID_ENTRY);
        assert_eq!("#fffffd", passport.hair_color.unwrap());
    }

    #[test]
    fn validate_eye_color() {
        let passport = Passport::build(VALID_ENTRY);
        assert_eq!("gry", passport.eye_color.unwrap());
    }

    #[test]
    fn validate_passport_id() {
        let passport = Passport::build(VALID_ENTRY);
        assert_eq!("860033327", passport.passport_id.unwrap());
    }

    #[test]
    fn validate_country_id() {
        let passport = Passport::build(VALID_ENTRY);
        assert_eq!("147", passport.country_id.unwrap());
    }

    #[test]
    fn requires_birth_year() {
        let passport = Passport::build("iyr:2020 eyr:2020 hgt:150cm hcl:#123456 ecl:amb pid:a cid:a");
        assert!(!passport.is_valid())
    }

    #[test]
    fn requires_issue_year() {
        let passport = Passport::build("byr:1920 eyr:2020 hgt:150cm hcl:#123456 ecl:amb pid:a cid:a");
        assert!(!passport.is_valid());
    }

    #[test]
    fn requires_expiration_year() {
        let passport = Passport::build("byr:1920 iyr:2020 hgt:150cm hcl:#123456 ecl:amb pid:a cid:a");
        assert!(!passport.is_valid());
    }

    #[test]
    fn requires_height() {
        let passport = Passport::build("byr:1920 iyr:2020 eyr:2020 hcl:#123456 ecl:amb pid:a cid:a");
        assert!(!passport.is_valid());
    }

    #[test]
    fn requires_hair_color() {
        let passport = Passport::build("byr:1920 iyr:2020 eyr:2020 hgt:150cm ecl:amb pid:a cid:a");
        assert!(!passport.is_valid());
    }

    #[test]
    fn requires_eye_color() {
        let passport = Passport::build("byr:1920 iyr:2020 eyr:2020 hgt:150cm hcl:#123456 pid:a cid:a");
        assert!(!passport.is_valid());
    }

    #[test]
    fn requires_passport_id() {
        let passport = Passport::build("byr:1920 iyr:2020 eyr:2020 hgt:150cm hcl:#123456 ecl:amb cid:a");
        assert!(!passport.is_valid());
    }

    #[test]
    fn doesnt_need_country_id() {
        let passport = Passport::build("byr:1920 iyr:2020 eyr:2020 hgt:150cm hcl:#123456 ecl:amb pid:a");
        assert!(passport.is_valid());
    }

    #[test]
    fn valid_whole_passport() {
        let passport = Passport::build("byr:1920 iyr:2020 eyr:2020 hgt:150cm hcl:#123456 ecl:amb pid:a cid:a");
        assert!(passport.is_valid());
    }

    #[test]
    fn require_four_digits_in_birth_year() {
        let passport1 = Passport::build("byr:19920 hcl:#dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        let passport2 = Passport::build("byr:199 hcl:#dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        assert!(!passport1.is_valid());
        assert!(!passport2.is_valid());
    }

    #[test]
    fn require_digits_in_birth_year() {
        let passport = Passport::build("byr:199a hcl:#dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        assert!(!passport.is_valid());
    }

    #[test]
    fn require_at_least_1920_in_birth_year() {
        let passport1 = Passport::build("byr:1919 hcl:#dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        let passport2 = Passport::build("byr:1920 hcl:#dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        assert!(!passport1.is_valid());
        assert!(passport2.is_valid());
    }

    #[test]
    fn require_at_most_2020_in_birth_year() {
        let passport1 = Passport::build("byr:2021 hcl:#dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        let passport2 = Passport::build("byr:2020 hcl:#dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        assert!(!passport1.is_valid());
        assert!(passport2.is_valid());
    }

    #[test]
    fn require_four_digits_in_issue_year() {
        let passport1 = Passport::build("iyr:201 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        let passport2 = Passport::build("iyr:20120 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        assert!(!passport1.is_valid());
        assert!(!passport2.is_valid());
    }

    #[test]
    fn require_digits_in_issue_year() {
        let passport = Passport::build("iyr:abcd byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        assert!(!passport.is_valid());
    }

    #[test]
    fn require_issue_year_at_least_2010() {
        let passport1 = Passport::build("iyr:2009 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        let passport2 = Passport::build("iyr:2010 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        assert!(!passport1.is_valid());
        assert!(passport2.is_valid());
    }

    #[test]
    fn require_issue_year_at_most_2020() {
        let passport1 = Passport::build("iyr:2020 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        let passport2 = Passport::build("iyr:2021 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 eyr:2020 cid:277");
        assert!(passport1.is_valid());
        assert!(!passport2.is_valid());
    }

    #[test]
    fn require_expiration_four_digits() {
        let passport1 = Passport::build("eyr:202 iyr:2020 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 cid:277");
        let passport2 = Passport::build("eyr:20202 iyr:2020 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 cid:277");
        assert!(!passport1.is_valid());
        assert!(!passport2.is_valid());
    }

    #[test]
    fn require_expiration_is_digits() {
        let passport = Passport::build("eyr:abcd iyr:2020 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 cid:277");
        assert!(!passport.is_valid());
    }

    #[test]
    fn require_expiration_at_least_2020() {
        let passport1 = Passport::build("eyr:2019 iyr:2020 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 cid:277");
        let passport2 = Passport::build("eyr:2020 iyr:2020 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 cid:277");
        assert!(!passport1.is_valid());
        assert!(passport2.is_valid());
    }

    #[test]
    fn require_expiration_at_most_2030() {
        let passport1 = Passport::build("eyr:2030 iyr:2020 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 cid:277");
        let passport2 = Passport::build("eyr:2031 iyr:2020 byr:1920 hcl:#dab227 ecl:brn hgt:182cm pid:021572410 cid:277");
        assert!(passport1.is_valid());
        assert!(!passport2.is_valid());
    }

    #[test]
    fn require_height_is_number_plus_units() {
        let mut passport_builder = PassportStringBuilder::new();

        passport_builder.height = "192";
        assert!(!Passport::build(&passport_builder.to_string()).is_valid());

        passport_builder.height = "acm";
        assert!(!Passport::build(&passport_builder.to_string()).is_valid());

        passport_builder.height = "ain";
        assert!(!Passport::build(&passport_builder.to_string()).is_valid());

        passport_builder.height = "192cm";
        assert!(Passport::build(&passport_builder.to_string()).is_valid());

        passport_builder.height = "59in";
        assert!(Passport::build(&passport_builder.to_string()).is_valid());
    }

    #[test]
    fn require_height_is_at_least_150cm() {
        let mut passport_builder = PassportStringBuilder::new();
        passport_builder.height = "149cm";
        assert!(!Passport::build(&passport_builder.to_string()).is_valid());
        passport_builder.height = "150cm";
        assert!(Passport::build(&passport_builder.to_string()).is_valid());
    }

    #[test]
    fn require_height_is_at_most_193cm() {
        let mut passport_builder = PassportStringBuilder::new();
        passport_builder.height = "193cm";
        assert!(Passport::build(&passport_builder.to_string()).is_valid());
        passport_builder.height = "194cm";
        assert!(!Passport::build(&passport_builder.to_string()).is_valid());
    }

    #[test]
    fn require_height_is_at_least_59in() {
        let mut passport_builder = PassportStringBuilder::new();
        passport_builder.height = "58in";
        assert!(!Passport::build(&passport_builder.to_string()).is_valid());
        passport_builder.height = "59in";
        assert!(Passport::build(&passport_builder.to_string()).is_valid());
    }

    #[test]
    fn require_height_is_at_most_76in() {
        let mut passport_builder = PassportStringBuilder::new();
        passport_builder.height = "76in";
        assert!(Passport::build(&passport_builder.to_string()).is_valid());
        passport_builder.height = "77in";
        assert!(!Passport::build(&passport_builder.to_string()).is_valid());
    }

    #[test]
    fn require_hair_color_is_pound_and_six_digits() {
        let mut passport_builder = PassportStringBuilder::new();
        passport_builder.hair_color = "123456";
        assert!(!Passport::build(&passport_builder.to_string()).is_valid());
        passport_builder.hair_color = "#12345";
        assert!(!Passport::build(&passport_builder.to_string()).is_valid());
        passport_builder.hair_color = "#1234567";
        assert!(!Passport::build(&passport_builder.to_string()).is_valid());
        passport_builder.hair_color = "#123456";
        assert!(Passport::build(&passport_builder.to_string()).is_valid());
    }

    #[test]
    fn require_hair_color_is_valid_digits() {
        let mut builder = PassportStringBuilder::new();
        builder.hair_color = "#123456";
        assert!(Passport::build(&builder.to_string()).is_valid());
        builder.hair_color = "#7890ab";
        assert!(Passport::build(&builder.to_string()).is_valid());
        builder.hair_color = "#cdef01";
        assert!(Passport::build(&builder.to_string()).is_valid());
        builder.hair_color = "#GHIJKL";
        assert!(!Passport::build(&builder.to_string()).is_valid());
    }

    #[test]
    fn require_eye_color_is_valid() {
        let mut builder = PassportStringBuilder::new();
        builder.eye_color = "lol";
        assert!(!Passport::build(&builder.to_string()).is_valid());
        VALID_EYE_COLORS.iter().for_each(|eye_color| {
            builder.eye_color = eye_color;
            assert!(Passport::build(&builder.to_string()).is_valid());
        })
    }
}