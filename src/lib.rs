//! # Overview
//! This little library will get you pin-point accuracy with your mortars in PlayerUnknown's Battlegrounds (PUBG).

use std::error::Error;

use crossterm::event::{self, Event, KeyCode};
use mouse_rs::Mouse;

const NUM_RECORD: u8 = 4;

/// Used to record the positions of the mouse cursor. The **ENTER** key is used to record an entry. 4 entries are required to get the distance. The first two are two callibrate the distance for
/// `c_dis`, e.g. if `c_dis` is 100 then that translates to 100 meters. The last two entries are to measure the distance between your position and the target position.
pub fn record<T: std::convert::Into<f64>>(
    c_dis: T,
    std_output: bool,
) -> Result<f64, Box<dyn Error>> {
    let c_dis: f64 = c_dis.into();
    let mouse = Mouse::new();
    let mut ctr = 0;

    let mut callibrate_pos = Vec::with_capacity((NUM_RECORD / 2) as usize);
    let mut mortar_pos = Vec::with_capacity((NUM_RECORD / 2) as usize);
    loop {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Enter {
                let pos = mouse.get_position()?;

                if std_output {
                    println!("Recorded position: {}/{}", ctr + 1, NUM_RECORD);
                }

                if ctr < (NUM_RECORD / 2) {
                    callibrate_pos.push(pos);
                } else {
                    mortar_pos.push(pos);
                }
                ctr += 1;

                if ctr == NUM_RECORD {
                    break;
                }
            }
        }
    }
    let call_x_2 = ((callibrate_pos[0].x - callibrate_pos[1].x) as f64).powf(2.);
    let call_y_2 = ((callibrate_pos[0].y - callibrate_pos[1].y) as f64).powf(2.);
    let call_dis = (call_x_2 + call_y_2).powf(0.5);

    let pos_x_2 = ((mortar_pos[0].x - mortar_pos[1].x) as f64).powf(2.);
    let pos_y_2 = ((mortar_pos[0].y - mortar_pos[1].y) as f64).powf(2.);
    let pos_dis = (pos_x_2 + pos_y_2).powf(0.5);
    Ok((c_dis / call_dis) * pos_dis)
}
