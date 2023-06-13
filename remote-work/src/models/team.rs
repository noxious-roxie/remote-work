use crate::models::Member;
use chrono::NaiveTime;

pub struct Team {
    members: Vec<Member>,
}

impl Team {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: Member) {
        self.members.push(member);
    }

    pub fn members(&self) -> &Vec<Member> {
        &self.members
    }

    pub fn members_online(&self, current_time: NaiveTime) -> Vec<&Member> {
        self.members
            .iter()
            .filter(|member| {
                member.work_intervals().iter().any(|&(start, end)| {
                    // working hours that do not cross over midnight
                    if start < end {
                        current_time >= start && current_time < end
                    // working hours that do cross over midnight
                    } else {
                        current_time >= start || current_time < end
                    }
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn test_add_member() {
        let mut team = Team::new();
        assert_eq!(team.members.len(), 0);

        let member = Member::new(
            String::from("Alice"),
            vec![
                (
                    NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                    NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
                ),
                (
                    NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
                    NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
                ),
            ],
        );
        team.add_member(member);
        assert_eq!(team.members.len(), 1);
    }

    #[test]
    fn test_members_online() {
        let mut team = Team::new();

        let alice = Member::new(
            String::from("Alice"),
            vec![
                (
                    NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                    NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
                ),
                (
                    NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
                    NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
                ),
            ],
        );
        let bob = Member::new(
            String::from("Bob"),
            vec![(
                NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(23, 30, 0).unwrap(),
            )],
        );
        team.add_member(alice);
        team.add_member(bob);

        let online_members = team.members_online(NaiveTime::from_hms_opt(15, 0, 0).unwrap());
        assert_eq!(online_members.len(), 1);
        assert_eq!(online_members[0].name(), "Alice");

        let online_members = team.members_online(NaiveTime::from_hms_opt(17, 00, 0).unwrap());
        assert_eq!(online_members.len(), 2);
        assert_eq!(online_members[0].name(), "Alice");
        assert_eq!(online_members[1].name(), "Bob");
    }
}
