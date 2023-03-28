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
        let ng_tim: i64 = (misc::misc::current_time_num() - 30) as i64;
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

        Command::new("mkdir")
            .args(vec![format!("./ac-library/atcoder/{}",file_name.clone())]).output().unwrap();

        Command::new("cp")
            .args(vec!["./game.hpp".to_string(),format!("./ac-library/atcoder/{}/game.hpp", file_name.clone())])
            .output().unwrap();
        Command::new("cp")
            .args(vec!["./judge.cpp".to_string(),format!("./ac-library/atcoder/{}/judge.cpp", file_name.clone())])
            .output().unwrap();
        Command::new("cp")
            .args(vec!["./player2.cpp".to_string(),format!("./ac-library/atcoder/{}/player2.cpp", file_name.clone())])
            .output().unwrap();

        let src_path: String = format!("./ac-library/atcoder/{}/player1.cpp", file_name.clone());
        let mut f = File::create(&Path::new(&src_path)).unwrap();
        f.write_all(data.source.clone().as_bytes());
    
        Command::new("sed")
            .args(vec!["-i".to_string(),"s/PlayerXXX/Player1/".to_string(),format!("./ac-library/atcoder/{}/player1.cpp", file_name.clone())])
            .output().unwrap();

        let bin_path: String = format!("./tmp/{}", file_name.clone());
        let compile_result = Command::new("g++")
            .args(vec!["-o".to_string(), bin_path.clone(),format!("./ac-library/atcoder/{}/judge.cpp", file_name.clone()) ,"-I".to_string(),"./ac-library/".to_string()])
            .status()
            .expect("Compile status Error");

        
        Command::new("cp")
            .args(vec!["./player1.cpp".to_string(),format!("./ac-library/atcoder/{}/player1.cpp", file_name.clone())])
            .output().unwrap();
        let src_path: String = format!("./ac-library/atcoder/{}/player2.cpp", file_name.clone());
        f = File::create(&Path::new(&src_path)).unwrap();
        f.write_all(data.source.clone().as_bytes());
        
        Command::new("sed")
                .args(vec!["-i".to_string(),"s/PlayerXXX/Player2/".to_string(),format!("./ac-library/atcoder/{}/player2.cpp", file_name.clone())])
                .output().unwrap();

        let compile_result2 = Command::new("g++")
                .args(vec!["-o".to_string(), bin_path.clone(),format!("./ac-library/atcoder/{}/judge.cpp", file_name.clone()) ,"-I".to_string(),"./ac-library/".to_string()])
                .status()
                .expect("Compile status Error");
        

        Command::new("rm")
            .args(vec!["-r".to_string(),format!("./ac-library/atcoder/{}",file_name.clone())])
            .output().unwrap();

        if compile_result.success() && compile_result2.success() {
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
                    &(cnt as i64),
                    &misc::misc::current_time_string(),
                    &(misc::misc::current_time_num() as i64),
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
