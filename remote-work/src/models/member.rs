use chrono::NaiveTime;
use chrono_tz::Tz;

#[derive(Debug)]
pub struct Member {
    name: String,
    // all work intervals must be supplied in UTC
    // timezone: Tz,
    work_intervals: Vec<(NaiveTime, NaiveTime)>,
}

impl Member {
    pub fn new(name: String, work_intervals: Vec<(NaiveTime, NaiveTime)>) -> Self {
        Self {
            name,
            work_intervals,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn timezone(&self) -> &Tz {
        &Tz::UTC
    }

    pub fn work_intervals(&self) -> &Vec<(NaiveTime, NaiveTime)> {
        &self.work_intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_member() {
        let work_intervals = vec![
            (
                NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            ),
        ];

        let member = Member::new(String::from("Alice"), work_intervals.clone());

        assert_eq!(member.name(), "Alice");
        assert_eq!(member.work_intervals(), &work_intervals);
    }

    #[test]
    fn work_intervals() {
        let work_intervals = vec![
            (
                NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            ),
            (
                NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            ),
        ];

        let member = Member::new(String::from("Bob"), work_intervals.clone());
        assert_eq!(member.work_intervals(), &work_intervals);
    }
}
