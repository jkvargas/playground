// https://leetcode.com/explore/interview/card/google/65/design-4/3093/

use std::collections::HashMap;

const TIME_BETWEEN: i32 = 10;

struct Logger {
    last_occurrence: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */
impl Logger {
    fn new() -> Self {
        Logger {
            last_occurrence: HashMap::new(),
        }
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(val) = self.last_occurrence.get_mut(&message) {
            if timestamp < *val {
                return false;
            } else {
                *val = timestamp + TIME_BETWEEN;
                return true;
            }
        }

        self.last_occurrence
            .insert(message, timestamp + TIME_BETWEEN);

        return true;
    }
}

/**
 * Your Logger object will be instantiated and called as such:
 * let obj = Logger::new();
 * let ret_1: bool = obj.should_print_message(timestamp, message);
 */

#[cfg(test)]
mod test {
    use crate::logger_rate_limiter::Logger;

    #[test]
    fn test_one() {
        let mut logger = Logger::new();

        assert!(logger.should_print_message(1, "foo".to_string()));
        assert!(logger.should_print_message(2, "bar".to_string()));
        assert!(!logger.should_print_message(3, "foo".to_string()));
        assert!(!logger.should_print_message(8, "bar".to_string()));
        assert!(!logger.should_print_message(10, "foo".to_string()));
        assert!(logger.should_print_message(11, "foo".to_string()));
    }

    //["Logger","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage","shouldPrintMessage"]
    // [[],[0,"A1"],[3,"A4"],[6,"A0"],[9,"A3"],[12,"A3"],[15,"A4"],[18,"A3"],[21,"A2"],[24,"A1"],[27,"A2"],[30,"A0"],[33,"A0"],[36,"A4"],[39,"A1"],[42,"A2"],[45,"A2"],[48,"A2"],[51,"A0"],[54,"A0"],[57,"A1"],[60,"A2"],[63,"A4"],[66,"A2"],[69,"A3"],[72,"A2"],[75,"A4"],[78,"A0"],[81,"A1"],[84,"A1"],[87,"A4"],[90,"A0"],[93,"A3"],[96,"A1"],[99,"A3"],[102,"A0"],[105,"A0"],[108,"A2"],[111,"A4"],[114,"A4"],[117,"A3"],[120,"A4"],[123,"A1"],[126,"A3"],[129,"A3"],[132,"A1"],[135,"A0"],[138,"A3"],[141,"A1"],[144,"A1"],[147,"A3"],[150,"A1"],[153,"A1"],[156,"A4"],[159,"A4"],[162,"A0"],[165,"A3"],[168,"A0"],[171,"A2"],[174,"A0"],[177,"A4"],[180,"A2"],[183,"A0"],[186,"A1"],[189,"A4"],[192,"A4"],[195,"A3"],[198,"A4"],[201,"A3"],[204,"A3"],[207,"A2"],[210,"A3"],[213,"A1"],[216,"A0"],[219,"A3"],[222,"A1"],[225,"A4"],[228,"A0"],[231,"A2"],[234,"A2"],[237,"A3"],[240,"A4"],[243,"A2"],[246,"A1"],[249,"A0"],[252,"A0"],[255,"A0"],[258,"A2"],[261,"A0"],[264,"A0"],[267,"A1"],[270,"A2"],[273,"A4"],[276,"A4"],[279,"A3"],[282,"A2"],[285,"A3"],[288,"A4"],[291,"A0"],[294,"A4"],[297,"A0"],[300,"A2"],[303,"A2"],[306,"A2"],[309,"A3"],[312,"A1"],[315,"A3"],[318,"A2"],[321,"A2"],[324,"A2"],[327,"A0"],[330,"A4"],[333,"A3"],[336,"A2"],[339,"A4"],[342,"A4"],[345,"A1"],[348,"A0"],[351,"A4"],[354,"A0"],[357,"A1"],[360,"A4"],[363,"A0"],[366,"A0"],[369,"A3"],[372,"A0"],[375,"A2"],[378,"A2"],[381,"A0"],[384,"A2"],[387,"A0"],[390,"A1"],[393,"A2"],[396,"A1"],[399,"A1"],[402,"A0"],[405,"A3"],[408,"A1"],[411,"A2"],[414,"A2"],[417,"A1"],[420,"A2"],[423,"A1"],[426,"A3"],[429,"A0"],[432,"A1"],[435,"A0"],[438,"A3"],[441,"A4"],[444,"A1"],[447,"A4"],[450,"A3"],[453,"A1"],[456,"A1"],[459,"A3"],[462,"A3"],[465,"A3"],[468,"A4"],[471,"A4"],[474,"A4"],[477,"A2"],[480,"A0"],[483,"A2"],[486,"A4"],[489,"A1"],[492,"A3"],[495,"A3"],[498,"A4"],[501,"A4"],[504,"A3"],[507,"A4"],[510,"A1"],[513,"A4"],[516,"A2"],[519,"A2"],[522,"A2"],[525,"A2"],[528,"A3"],[531,"A1"],[534,"A0"],[537,"A2"],[540,"A0"],[543,"A4"],[546,"A1"],[549,"A2"],[552,"A1"],[555,"A0"],[558,"A0"],[561,"A2"],[564,"A1"],[567,"A3"],[570,"A0"],[573,"A2"],[576,"A1"],[579,"A3"],[582,"A3"],[585,"A4"],[588,"A3"],[591,"A0"],[594,"A0"],[597,"A0"],[600,"A2"],[603,"A4"],[606,"A2"],[609,"A1"],[612,"A3"],[615,"A4"],[618,"A0"],[621,"A1"],[624,"A2"],[627,"A4"],[630,"A4"],[633,"A3"],[636,"A1"],[639,"A2"],[642,"A3"],[645,"A0"],[648,"A1"],[651,"A1"],[654,"A2"],[657,"A1"],[660,"A3"],[663,"A4"],[666,"A1"],[669,"A3"],[672,"A4"],[675,"A3"],[678,"A2"],[681,"A2"],[684,"A1"],[687,"A3"],[690,"A2"],[693,"A0"],[696,"A4"],[699,"A0"],[702,"A1"],[705,"A0"],[708,"A2"],[711,"A3"],[714,"A3"],[717,"A3"],[720,"A4"],[723,"A2"],[726,"A1"],[729,"A0"],[732,"A0"],[735,"A3"],[738,"A4"],[741,"A0"],[744,"A3"],[747,"A1"],[750,"A0"],[753,"A2"],[756,"A3"],[759,"A0"],[762,"A4"],[765,"A3"],[768,"A3"],[771,"A4"],[774,"A1"],[777,"A1"],[780,"A4"],[783,"A2"],[786,"A1"],[789,"A3"],[792,"A2"],[795,"A3"],[798,"A3"],[801,"A2"],[804,"A1"],[807,"A4"],[810,"A2"],[813,"A1"],[816,"A0"],[819,"A4"],[822,"A0"],[825,"A3"],[828,"A2"],[831,"A2"],[834,"A1"],[837,"A0"],[840,"A3"],[843,"A1"],[846,"A1"],[849,"A1"],[852,"A0"],[855,"A3"],[858,"A2"],[861,"A1"],[864,"A2"],[867,"A2"],[870,"A0"],[873,"A2"],[876,"A2"],[879,"A0"],[882,"A1"],[885,"A2"],[888,"A3"],[891,"A0"],[894,"A1"],[897,"A1"]]
}
