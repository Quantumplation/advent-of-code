use anyhow::*;
use std::str::FromStr;

pub mod part1 {
    use super::*;
    pub fn solve(schedule: BusSchedule) -> Result<u64> {
        let (min, wait) = find_departure(schedule);
        return Ok(min * wait);
    }
}
pub mod part2 {
    use super::*;
    pub fn solve(schedule: BusSchedule) -> Result<u64> {
        return Ok(find_cascade(schedule));
    }
}

pub struct BusSchedule {
    pub departure: u64,
    pub busses: Vec<Option<u64>>,
}

impl FromStr for BusSchedule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split("\n");
        let (departure, busses) = (splits.next().unwrap().parse::<u64>().unwrap(), splits.next().unwrap());
        let busses: Vec<_> = busses.split(',')
            .map(|s| s.parse::<u64>())
            .map(|r| r.ok())
            .collect();
        Ok(BusSchedule { departure, busses })
    }
}

pub fn find_departure(schedule: BusSchedule) -> (u64, u64) {
    let mut bus_id = 0;
    let mut departure_time = u64::MAX;
    let mut wait = 0;
    for bus in schedule.busses {
        if let Some(bus) = bus {
            let min_departure = ((schedule.departure / bus) + 1) * bus;
            if min_departure < departure_time {
                bus_id = bus;
                departure_time = min_departure;
                wait = min_departure - schedule.departure;
            }
        }
    }
    return (bus_id, wait);
}

pub fn find_cascade(schedule: BusSchedule) -> u64 {
    let mut timestamp = schedule.busses[0].unwrap();
    let mut lcm = timestamp;
    for (minute, bus) in schedule.busses.iter().enumerate() {
        let minute = minute as u64;
        if let Some(bus) = bus {
            loop {
                if (timestamp + minute) % bus == 0 {
                    break;
                }
                timestamp += lcm;
            }
            lcm = num::integer::lcm(lcm, *bus);
        }
    }
    return timestamp
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success(){
        assert_eq!(1068781, find_cascade(BusSchedule { departure: 0, busses: vec![Some(7), Some(13), None, None, Some(59), None, Some(31), Some(19)]}));
        assert_eq!(3417, find_cascade(BusSchedule { departure: 0, busses: vec![Some(17), None, Some(13), Some(19)]}));
        assert_eq!(1202161486, find_cascade(BusSchedule { departure: 0, busses: vec![Some(1789), Some(37), Some(47), Some(1889)]}));
    }
}