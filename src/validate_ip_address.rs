struct Solution;

impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        if is_ipv4(&query_ip) { return "IPv4".to_string();}
        if is_ipv6(&query_ip) { return "IPv6".to_string(); }

        "Neither".to_string()
    }
}

fn is_ipv6(word: &String) -> bool {
    let cut = word.split(":").map(|x| x.to_string()).collect::<Vec<String>>();

    for x in cut {
        if x.is_empty() { return false; }

        if !x.chars().all(|c| c.is_digit(16) ) {
            return false;
        }
    }

    true
}

fn is_ipv4(word: &String) -> bool {
    let cut = word.split(".").map(|x| x.to_string()).collect::<Vec<String>>();
    if cut.len() != 4 {
        return false;
    }

    for x in cut {
        let number = x.parse::<i32>();
        if number.is_err() { return false; }
        let n = number.unwrap();
        if !(n >= 0 && n <= 255) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::validate_ip_address::Solution;

    #[test]
    fn test() {
        assert_eq!("IPv4".to_string(), Solution::valid_ip_address("01.01.01.01".to_string()));
    }
}