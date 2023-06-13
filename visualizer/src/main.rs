use chrono::{FixedOffset, NaiveTime};
use remote_work::models::{Member, Team};
use remote_work_visualizer::create_and_save_work_schedule;
use serde::Deserialize;
use serde_json::from_str;
use std::fs;

#[derive(Deserialize)]
pub struct MemberData {
    name: String,
    work_intervals: Vec<(String, String)>,
}

#[derive(Deserialize)]
pub struct TeamData {
    members: Vec<MemberData>,
}

pub fn main() -> () {
    let data = fs::read_to_string("team.example.json").expect("Unable to read file");
    let team_data: TeamData = from_str(&data).expect("Unable to parse JSON");

    let mut team = Team::new();

    for member_data in team_data.members {
        let intervals: Vec<(NaiveTime, NaiveTime)> = member_data
            .work_intervals
            .iter()
            .map(|(start, end)| {
                let start_time = NaiveTime::parse_from_str(start, "%H:%M:%S")
                    .expect("Unable to parse start time");
                let end_time =
                    NaiveTime::parse_from_str(end, "%H:%M:%S").expect("Unable to parse end time");

                (start_time, end_time)
            })
            .collect();

        let member = Member::new(member_data.name, intervals);
        team.add_member(member);
    }

    let my_offset = &FixedOffset::east_opt(0).unwrap();
    let path = create_and_save_work_schedule(&team, my_offset).unwrap();
    println!("saved to {}", path.to_string_lossy());
}
