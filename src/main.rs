#![allow(unused)]
mod sg;
mod web;
mod ws;

use std::env;

#[tokio::main]
async fn main() {
    let ath = env::args().nth(1).unwrap_or("P2".to_string());
    match ath.as_str() {
        "P1" => crate::sg::init1::run().await,
        "P2" => crate::sg::init2::run().await,
        "P3" => crate::sg::init3::run().await,
        "P4" => crate::sg::tranx::run().await,
        "I1" => crate::sg::mvline::read().await,
        "I2" => crate::sg::mvline::read_trans_lv().await,
        "I3" => crate::sg::mvline::read_lv_line().await,
        _ => print!("NG\n"),
    }
}
