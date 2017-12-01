extern crate rand;
extern crate chrono;

use rand::Rng;
use chrono::prelude::{DateTime, Local, Timelike};

fn main() {
    fn get_signature_id() -> String {
        let mut rng = rand::thread_rng();
        format!("{:x}" ,(rng.gen::<u32>()))
    }
    println!("get_signature_id: {}", get_signature_id());


    fn generate_timestamp_relative() -> String {
        let dt: DateTime<Local> = Local::now();
        let mut timestamp = format!("{:02}:", dt.hour());
        timestamp.push_str(&format!("{:02}", dt.minute()));
        timestamp

    }
    println!("generate_timestamp_relative: {}", generate_timestamp_relative());


    fn generate_threading_ID(client_id: String) -> String{
        let dt: DateTime<Local> = Local::now();
        // TODO: Use .timestamp_millis when chrono releases 0.5
        let mut timestamp = (dt.timestamp() * 1000) as u64;
        timestamp += u64::from(dt.timestamp_subsec_millis());
        let mut rng = rand::thread_rng();
        let random_num = rng.gen::<u32>();

        let mut threading_id =  "<".to_string();
        threading_id.push_str(&timestamp.to_string());
        threading_id.push_str(":");
        threading_id.push_str(&random_num.to_string());
        threading_id.push_str("-");
        threading_id.push_str(&client_id);
        threading_id.push_str("@mail.projektitan.com>");
        threading_id

    }
    println!("{:?}", generate_threading_ID("1234".to_string()));


    fn generate_offline_threading_id() -> u64 {
        let mut rng = rand::thread_rng();
        let random_num = rng.gen::<u32>();

        let mut random_num = format!("{:b}", random_num);

        if random_num.len() < 22 {
            // pad if less than 22 len
            let mut random_num_str = "".to_string();
            for i in 0..(22 - random_num.len()) {
                random_num_str.push_str("0");
            }
            random_num_str.push_str(&random_num);
            random_num = random_num_str.to_string();
        } else {
            random_num = random_num[random_num.len() - 22..].to_string();
        }

        let dt: DateTime<Local> = Local::now();
        // TODO: Use .timestamp_millis when chrono releases 0.5
        let mut timestamp = (dt.timestamp() * 1000) as u64;
        timestamp += u64::from(dt.timestamp_subsec_millis());

        let mut threading_id = format!("{:b}", timestamp);
        threading_id.push_str(&random_num);

        let threading_id = u64::from_str_radix(&threading_id, 2);
        threading_id.unwrap()
    }
    println!("{}", generate_offline_threading_id());

}
