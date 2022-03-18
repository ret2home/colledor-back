pub mod challenge {
    use super::super::auth;
    use super::super::misc;
    use actix_web::{get, post, web, HttpResponse, Responder};
    use postgres::{Client, NoTls};
    use rand::{thread_rng, Rng};
    use serde::{Deserialize, Serialize};
    use std::env;
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io::prelude::*;
    use std::iter;
    use std::path::Path;
    use std::process::Command;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ChallengeRequest {
        token: String,
        target: String,
    }

    const SERVER_NUM: i64 = 3;
    const CONTEST_END: i64 = 1647529200;

    #[post("/challenge")]
    pub fn challenge(data: web::Json<ChallengeRequest>) -> HttpResponse {
        let DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
        let user_id = auth::auth::verify_token(data.token.clone());
        if let Err(e) = user_id {
            return HttpResponse::Unauthorized().json({});
        }
        let user_id = user_id.unwrap();

        // 頻度制限
        let mut conn = Client::connect(&DATABASE_URL, NoTls).unwrap();

        let ng_tim: i32 = (misc::misc::current_time_num() - 180) as i32;
        let rows = conn
            .query(
                "SELECT COUNT(*) FROM challenges WHERE tim_num > $1 AND user1_id=$2",
                &[&ng_tim, &user_id.clone()],
            )
            .unwrap();
        let mut cnt: i64 = 0;
        for row in rows {
            cnt = row.get(0);
        }
        if cnt != 0 {
            return HttpResponse::BadRequest().json({});
        }

        // 既に両者共に提出されてるかどうか
        let rows = conn.query("SELECT COUNT(*) FROM (SELECT * FROM (SELECT id FROM submissions WHERE user_id=$1 limit 1) AS table1 UNION ALL SELECT * FROM (SELECT id FROM submissions WHERE user_id=$2 limit 1) AS table2) AS table3;",&[&user_id.clone(),&data.target.clone()]).unwrap();
        cnt = 0;
        for row in rows {
            cnt = row.get(0);
        }
        if cnt != 2 {
            return HttpResponse::BadRequest().json({});
        }

        let mut rated: i32 = 0;

        // 自分以上のレーティングかつ、自分自身でないかつ、コンテスト中なら rated
        let rows=conn.query("SELECT * FROM users WHERE rating >= (SELECT rating FROM users WHERE id=$1) AND id=$2 AND id!=$1",&[&user_id.clone(), &data.target.clone()]).unwrap();
        for row in rows {
            rated = 1;
        }

        if misc::misc::current_time_num() >= CONTEST_END {
            rated = 0;
        }

        conn.execute("BEGIN;", &[]).unwrap();
        let mut cnt: i64 = 0;
        let rows = conn.query("SELECT COUNT(*) FROM challenges", &[]).unwrap();
        for row in rows {
            cnt = row.get(0);
        }
        cnt += 1;

        let server_id: i64 = cnt % SERVER_NUM;

        conn.execute(
            "INSERT INTO challenges VALUES($1,$2,$3,$4,$5,'WJ',$6,$7,'','','')",
            &[
                &(cnt as i32),
                &rated,
                &misc::misc::current_time_string(),
                &(misc::misc::current_time_num() as i32),
                &(server_id as i32),
                &user_id.clone(),
                &data.target.clone(),
            ],
        )
        .unwrap();
        conn.execute("COMMIT;", &[]).unwrap();
        return HttpResponse::Ok().json({});
    }
}
