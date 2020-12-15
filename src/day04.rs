use crate::prelude::*;

day!(4, parse => pt1, pt2);

type Passport<'a> = HashMap<&'a str, &'a str>;

lazy_static! {
    static ref EXPECTED_FIELDS: HashSet<&'static str> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",]
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
}

fn has_expected_fields(passport: &Passport) -> bool {
    let included_count = passport
        .keys()
        .count_if(|key| EXPECTED_FIELDS.contains(key));
    included_count == EXPECTED_FIELDS.len()
}

fn is_number_in_range(str: &str, low: u32, high: u32) -> bool {
    use framework::parser::*;
    take_u32(str)
        .into_result()
        .ok()
        .filter(|&nr| nr >= low && nr <= high)
        .is_some()
}

fn is_valid_passport(passport: &Passport) -> bool {
    if !has_expected_fields(passport) {
        return false;
    }
    for (key, value) in passport {
        let is_valid = match key.as_bytes() {
            b"byr" => is_number_in_range(value, 1920, 2002),
            b"iyr" => is_number_in_range(value, 2010, 2020),
            b"eyr" => is_number_in_range(value, 2020, 2030),
            b"hgt" if value.len() < 3 => false,
            b"hgt" => {
                let unit = &value[value.len() - 2..];
                let value = &value[..value.len() - 2];
                match unit.as_bytes() {
                    b"cm" => is_number_in_range(value, 150, 193),
                    b"in" => is_number_in_range(value, 59, 76),
                    _ => false,
                }
            }
            b"hcl" => {
                value.len() == 7
                    && value[1..].chars().all(|char| {
                        char.is_ascii_digit() || (char.is_lowercase() && char.is_digit(16))
                    })
            }
            b"ecl" => matches!(
                value.as_bytes(),
                b"amb" | b"blu" | b"brn" | b"gry" | b"grn" | b"hzl" | b"oth"
            ),
            b"pid" => value.len() == 9 && value.chars().all(|char| char.is_ascii_digit()),
            _ => continue,
        };
        if !is_valid {
            return false;
        }
    }
    true
}

pub fn pt1(input: &[Passport]) -> usize {
    input.iter().count_if(has_expected_fields)
}

pub fn pt2(input: &[Passport]) -> usize {
    input.iter().count_if(is_valid_passport)
}

pub fn parse(input: &str) -> Result<Vec<Passport>> {
    use framework::parser::*;
    let is_value_char = |c: char| !c.is_ascii_whitespace() && c != ':';
    let key_value_pair = pair(
        terminated(take_while1(is_value_char), char(':')),
        take_while1(is_value_char),
    );
    let passport = fold_many1(
        terminated(key_value_pair, opt(alt((char(' '), char('\n'))))),
        HashMap::new(),
        |mut map, (key, value)| {
            map.insert(key, value);
            map
        },
    );
    separated_list1(whitespace1, passport)(input).into_result()
}

#[cfg(test)]
const PARSE_EXAMPLE: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";

#[cfg(test)]
const EXAMPLE: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

standard_tests!(
    parse [
        PARSE_EXAMPLE => vec![
            vec![
                ("ecl", "gry"),
                ("pid", "860033327"),
                ("eyr", "2020"),
                ("hcl", "#fffffd"),
                ("byr", "1937"),
                ("iyr", "2017"),
                ("cid", "147"),
                ("hgt", "183cm"),
            ].into_iter().collect::<HashMap<&str, &str>>(),
            vec![
                ("iyr", "2013"),
                ("ecl", "amb"),
                ("cid", "350"),
                ("eyr", "2023"),
                ("pid", "028048884"),
                ("hcl", "#cfa07d"),
                ("byr", "1929"),
            ].into_iter().collect::<HashMap<&str, &str>>()
        ]
    ]
    pt1 [ EXAMPLE => 2 ]
);
