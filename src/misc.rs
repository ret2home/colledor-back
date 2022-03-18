pub mod misc{
    use chrono::{Local,FixedOffset};
    extern crate rand;
    use rand::seq::SliceRandom;


    pub fn current_time_num() -> i64{
        return Local::now().with_timezone(&FixedOffset::east(9*3600)).timestamp();
    }
    pub fn current_time_string() -> String {
        return Local::now().with_timezone(&FixedOffset::east(9*3600)).format("%Y/%m/%d %H:%M:%S").to_string();
    }
    pub fn gen_rnd_str(size: usize)->String{
        const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = &mut rand::thread_rng();
        String::from_utf8(
            BASE_STR.as_bytes()
                .choose_multiple(&mut rng, size)
                .cloned()
                .collect()
        ).unwrap()
    }
}