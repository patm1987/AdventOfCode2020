struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn build(input: &str) -> Passport {
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
                "byr" => passport.birth_year = Some(value.to_string()),
                "iyr" => passport.issue_year = Some(value.to_string()),
                "eyr" => passport.expiration_year = Some(value.to_string()),
                "hgt" => passport.height = Some(value.to_string()),
                "hcl" => passport.hair_color = Some(value.to_string()),
                "ecl" => passport.eye_color = Some(value.to_string()),
                "pid" => passport.passport_id = Some(value.to_string()),
                "cid" => passport.country_id = Some(value.to_string()),
                _ => {}
            }
        });
        passport
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_ENTRY: &'static str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm";

    #[test]
    fn validate_birth_year() {
        let passport = Passport::build(VALID_ENTRY);
        assert_eq!("1937", passport.birth_year.unwrap());
    }

    #[test]
    fn validate_issue_year() {
        let passport = Passport::build(VALID_ENTRY);
        assert_eq!("2017", passport.issue_year.unwrap());
    }

    #[test]
    fn validate_expiration_year() {
        let passport = Passport::build(VALID_ENTRY);
        assert_eq!("2020", passport.expiration_year.unwrap());
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
}