use chrono::{DateTime, FixedOffset, Local, Timelike};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut};
use rusttype::{Font, Scale};
use std::env;
use std::io::Result;
use std::path::PathBuf;

use remote_work::models::Team;
use remote_work::utils::time;

pub fn create_team_schedule_image(
    team: &Team,
    utc_offset: &FixedOffset,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    const WHITE_PIXEL: [u8; 4] = [255, 255, 255, 255];
    const BLACK_PIXEL: [u8; 4] = [0, 0, 0, 255];

    const EARLIEST_HOUR: i32 = 0;
    const LATEST_HOUR: i32 = 23;

    let width = (LATEST_HOUR - EARLIEST_HOUR + 1) * 40 + 200;
    let height = team.members().len() * 50 + 100;

    let mut img = ImageBuffer::from_fn(width as u32, height as u32, |_, _| {
        Rgba(WHITE_PIXEL) // White pixel
    });

    let font_data = include_bytes!("../fonts/IndieFlower-Regular.ttf") as &[u8];
    let font = Font::try_from_bytes(font_data).unwrap();
    let scale = Scale::uniform(20.0);

    let current_time = Local::now().naive_local();
    let current_time_in_target_tz = DateTime::<FixedOffset>::from_utc(current_time, *utc_offset);
    let current_date_time_string =
        format!("{}", current_time_in_target_tz.format("%Y-%m-%d %H:%M:%S"));
    // Set font and scale for header
    let header_font = Font::try_from_bytes(font_data).unwrap();
    let header_scale = Scale::uniform(30.0);
    // Draw the current date and time at the top of the image
    draw_text_mut(
        &mut img,
        Rgba([0u8, 0u8, 0u8, 255u8]),
        50, // X Position: Adjust this to fit your image size
        0,  // Y Position: Adjust this to fit your image size
        header_scale,
        &header_font,
        &current_date_time_string,
    );

    // Draw Y axis labels (team member names)
    for (i, member) in team.members().iter().enumerate() {
        let y = i * 50 + 50;
        draw_text_mut(
            &mut img,
            Rgba(BLACK_PIXEL),
            10,
            y.try_into().unwrap(),
            scale,
            &font,
            &member.name(),
        );
    }

    // Draw X axis labels (hours in a day)
    for hour in (EARLIEST_HOUR)..=(LATEST_HOUR + 1) {
        let x = (hour - EARLIEST_HOUR) * 40 + 100;
        let hour = if hour != 24 { hour } else { 0 };
        draw_text_mut(
            &mut img,
            Rgba(BLACK_PIXEL),
            x.try_into().unwrap(),
            30,
            scale,
            &font,
            &format!("{}", hour),
        );
    }

    // Draw blocks for each member's work intervals
    for (i, member) in team.members().iter().enumerate() {
        let y = i * 50 + 50;

        for &(start, end) in member.work_intervals() {
            let start_hour = time::convert_to_tz(start, utc_offset).hour() as i32;
            let end_hour = time::convert_to_tz(end, utc_offset).hour() as i32;

            if end_hour < start_hour {
                // The work interval crosses over midnight

                // Draw from start hour to midnight
                let x1 = (start_hour - EARLIEST_HOUR) * 40 + 100;
                let x2 = (24 - EARLIEST_HOUR) * 40 + 100;
                draw_filled_rect_mut(
                    &mut img,
                    imageproc::rect::Rect::at(x1, y.try_into().unwrap())
                        .of_size((x2 - x1 + 1) as u32, 40),
                    Rgba(BLACK_PIXEL),
                );

                // Draw from midnight to end hour
                let x1 = (0 - EARLIEST_HOUR) * 40 + 100;
                let x2 = (end_hour - EARLIEST_HOUR) * 40 + 100;
                draw_filled_rect_mut(
                    &mut img,
                    imageproc::rect::Rect::at(x1, y.try_into().unwrap())
                        .of_size((x2 - x1 + 1) as u32, 40),
                    Rgba(BLACK_PIXEL),
                );
            } else {
                // The work interval does not cross over midnight
                let x1 = (start_hour - EARLIEST_HOUR) * 40 + 100;
                let x2 = (end_hour - EARLIEST_HOUR) * 40 + 100;
                draw_filled_rect_mut(
                    &mut img,
                    imageproc::rect::Rect::at(x1, y.try_into().unwrap())
                        .of_size((x2 - x1 + 1) as u32, 40),
                    Rgba(BLACK_PIXEL),
                );
            }
        }
    }

    let current_time = Local::now().naive_local();
    let current_time_in_target_tz = DateTime::<FixedOffset>::from_utc(current_time, *utc_offset);
    let current_time = current_time_in_target_tz.time();
    let current_hour = current_time.hour() as i32;
    let current_minutes = current_time.minute() as i32;
    let current_x = (current_hour * 60 + current_minutes) * 40 / 60 + 100;

    // Draw a red line from top to bottom at the current time
    draw_line_segment_mut(
        &mut img,
        (current_x as f32, 0.0),           // Start point
        (current_x as f32, height as f32), // End point
        Rgba([255u8, 0u8, 0u8, 255u8]),    // Red
    );

    img
}

pub fn save_temp_image(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<PathBuf> {
    let temp_dir = env::temp_dir();
    let file_path = temp_dir.join("team_schedule.png");
    image
        .save_with_format(&file_path, image::ImageFormat::Png)
        .unwrap();
    Ok(file_path)
}

pub fn create_and_save_work_schedule(team: &Team, utc_offset: &FixedOffset) -> Result<PathBuf> {
    let img = create_team_schedule_image(team, utc_offset);
    save_temp_image(&img)
}
