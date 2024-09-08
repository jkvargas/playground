use std::fmt::format;

struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let split = date.split("-").map(|s| s.to_string()).collect::<Vec<String>>();

        let year = split[0].parse::<u32>().unwrap();
        let month = split[1].parse::<u32>().unwrap();
        let day = split[2].parse::<u32>().unwrap();

        let binary_year = format!("{:b}", year);
        let binary_month = format!("{:b}", month);
        let binary_day = format!("{:b}", day);

        format!("{}-{}-{}", binary_year, binary_month, binary_day)
    }
}

#[cfg(test)]
mod test {
    use crate::convert_date_to_binary::Solution;

    #[test]
    fn test_one() {
        assert_eq!("100000100000-10-11101".to_string(), Solution::convert_date_to_binary("2080-02-29".to_string()));
    }
}