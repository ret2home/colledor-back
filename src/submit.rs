pub mod submit {
    use super::super::auth;
    use super::super::misc;
    use actix_web::{get, post, web, HttpResponse, Responder};
    use postgres::{Client, NoTls};
    use serde::{Deserialize, Serialize};
    use std::env;
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io::prelude::*;
    use std::path::Path;
    use std::process::Command;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubmissionRequest {
        token: String,
        source: String,
    }

    #[post("/submit")]
    pub fn submit(data: web::Json<SubmissionRequest>) -> HttpResponse {
        let DATABASE_URL: String = env::var("DATABASE_URL").unwrap();

        let user_id = auth::auth::verify_token(data.token.clone());
        if let Err(e) = user_id {
            return HttpResponse::Unauthorized().json({});
        }
        let user_id = user_id.unwrap();

        let mut conn = Client::connect(&DATABASE_URL, NoTls).unwrap();
        
        // 提出頻度制限
        let ng_tim: i32 = (misc::misc::current_time_num() - 30) as i32;
        for row in conn
            .query(
                "SELECT * FROM submissions WHERE user_id=$1 AND tim_num>$2",
                &[&user_id.clone(), &ng_tim],
            )
            .unwrap()
        {
            return HttpResponse::BadRequest().json({});
        }

        let file_name: String = misc::misc::gen_rnd_str(8);
        let src_path: String = format!("./ac-library/atcoder/{}.cpp", file_name.clone());
        let mut f = File::create(&Path::new(&src_path)).unwrap();
        f.write_all(data.source.clone().as_bytes());

        let bin_path: String = format!("./tmp/{}", file_name.clone());
        let compile_result = Command::new("g++")
            .args(vec!["-o".to_string(), bin_path.clone(), src_path.clone(),"-I".to_string(),"./ac-library/".to_string()])
            .status()
            .expect("Compile status Error");
        fs::remove_file(src_path).unwrap();

        if compile_result.success() {
            fs::remove_file(bin_path).unwrap();
            conn.execute("BEGIN", &[]).unwrap();

            let mut cnt: i64 = 0;
            for row in &conn.query("SELECT COUNT(*) FROM submissions", &[]).unwrap() {
                cnt = row.get(0);
            }
            cnt += 1;
            conn.execute(
                "INSERT INTO submissions VALUES($1,$2,$3,$4,$5)",
                &[
                    &(cnt as i32),
                    &misc::misc::current_time_string(),
                    &(misc::misc::current_time_num() as i32),
                    &user_id.clone(),
                    &data.source.clone(),
                ],
            )
            .unwrap();
            conn.execute("COMMIT", &[]).unwrap();
            return HttpResponse::Ok().json({});
        }
        return HttpResponse::BadRequest().json({});
    }
}
