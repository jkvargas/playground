// https://leetcode.com/problems/design-underground-system/

use std::collections::HashMap;

type StationName = String;
type UserId = i32;

struct UndergroundSystem {
    times: HashMap<StationTravel, TimeToTravel>,
    got_in: HashMap<UserId, CheckIn>,
}

struct CheckIn {
    when: i32,
    station_name: StationName
}

impl CheckIn {
    fn new(time: i32, station_name: StationName) -> CheckIn {
        Self {
            when: time,
            station_name
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct StationTravel {
    pub start: StationName,
    pub end: StationName,
}

struct TimeToTravel {
    amount: f64,
    total: f64,
    medium: f64,
}

impl TimeToTravel {
    fn new(time: i32) -> Self {
        Self {
            amount: 1.0,
            total: time as f64,
            medium: time as f64,
        }
    }

    fn new_time(&mut self, time: i32) {
        self.amount += 1.0;
        self.total += time as f64;
        self.medium = self.total / self.amount;
    }
}

impl StationTravel {
    fn new(start: StationName, end: StationName) -> Self {
        Self {
            start,
            end
        }
    }
}

/**
  * `&self` means the method takes an immutable reference
  * If you need a mutable reference, change it to `&mut self` instead
  */

impl UndergroundSystem {

    fn new() -> Self {
        Self {
            times: HashMap::new(),
            got_in: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        let checkin = CheckIn::new(t, station_name);
        self.got_in.insert(id, checkin);
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let departed_from = self.got_in.get(&id).unwrap();
        let station_travel = StationTravel::new(departed_from.station_name.clone(), station_name);

        if let Some(st) = self.times.get_mut(&station_travel) {
            st.new_time(t - departed_from.when);
        } else {
            self.times.insert(station_travel, TimeToTravel::new(t - departed_from.when));
        }

        self.got_in.remove(&id);
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let station_travel = StationTravel::new(start_station, end_station);

        if let Some(st) = self.times.get(&station_travel) {
            return st.medium;
        }

        0.0
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
