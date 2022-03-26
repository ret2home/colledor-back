pub mod info {
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
    pub struct Submission {
        id: i32,
        tim: String,
        user: String,
        source: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubmissionInfoRequest {
        token: String,
        id: i32,
    }
    #[post("/submission-info")]
    pub fn submission_info(data: web::Json<SubmissionInfoRequest>) -> HttpResponse {
        let DATABASE_URL: String = env::var("DATABASE_URL").unwrap();

        let user_id = auth::auth::verify_token(data.token.clone());
        if let Err(e) = user_id {
            return HttpResponse::Unauthorized().json({});
        }
        let user_id = user_id.unwrap();
        let mut conn = Client::connect(&DATABASE_URL, NoTls).unwrap();

        let contest_end:i64=env::var("CONTEST_END").unwrap().parse().unwrap();

        if misc::misc::current_time_num() < contest_end && user_id.clone() != "admin" {
            let rows = conn
                .query(
                    "SELECT * FROM submissions WHERE id=$1 AND user_id=$2",
                    &[&data.id, &user_id.clone()],
                )
                .unwrap();

            for row in rows {
                let res: Submission = Submission {
                    id: row.get(0),
                    tim: row.get(1),
                    user: row.get(3),
                    source: row.get(4),
                };
                return HttpResponse::Ok().json(res);
            }
            return HttpResponse::BadRequest().json({});
        } else {
            let rows = conn
                .query("SELECT * FROM submissions WHERE id=$1", &[&data.id])
                .unwrap();

            for row in rows {
                let res: Submission = Submission {
                    id: row.get(0),
                    tim: row.get(1),
                    user: row.get(3),
                    source: row.get(4),
                };
                return HttpResponse::Ok().json(res);
            }
            return HttpResponse::BadRequest().json({});
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct SubmissionsListRequest {
        token: String,
        user: String,
    }
    #[post("/submissions-list")]
    pub fn submissions_list(data: web::Json<SubmissionsListRequest>) -> HttpResponse {
        let DATABASE_URL: String = env::var("DATABASE_URL").unwrap();

        let user_id = auth::auth::verify_token(data.token.clone());
        if let Err(e) = user_id {
            return HttpResponse::Unauthorized().json({});
        }
        let user_id = user_id.unwrap();
        let mut conn = Client::connect(&DATABASE_URL, NoTls).unwrap();

        #[derive(Debug, Serialize, Deserialize)]
        struct SubmissionsListResponse {
            submissions: Vec<Submission>,
        }

        let contest_end:i64=env::var("CONTEST_END").unwrap().parse().unwrap();

        // コンテスト中は自分の以外閲覧禁止
        if misc::misc::current_time_num() < contest_end && user_id.clone() != "admin" {
            let rows = conn
                .query(
                    "SELECT * FROM submissions WHERE user_id=$1 ORDER BY id DESC",
                    &[&user_id.clone()],
                )
                .unwrap();

            let mut res: SubmissionsListResponse = SubmissionsListResponse {
                submissions: vec![],
            };
            for row in rows {
                res.submissions.push(Submission {
                    id: row.get(0),
                    tim: row.get(1),
                    user: row.get(3),
                    source: "".to_string(),
                });
            }
            return HttpResponse::Ok().json(res);
        } else {
            if data.user.clone() != "all" {
                let rows = conn
                    .query(
                        "SELECT * FROM submissions WHERE user_id=$1 ORDER BY id DESC",
                        &[&data.user.clone()],
                    )
                    .unwrap();

                let mut res: SubmissionsListResponse = SubmissionsListResponse {
                    submissions: vec![],
                };
                for row in rows {
                    res.submissions.push(Submission {
                        id: row.get(0),
                        tim: row.get(1),
                        user: row.get(3),
                        source: "".to_string(),
                    });
                }
                return HttpResponse::Ok().json(res);
            } else {
                let rows = conn
                    .query("SELECT * FROM submissions ORDER BY id DESC", &[])
                    .unwrap();

                let mut res: SubmissionsListResponse = SubmissionsListResponse {
                    submissions: vec![],
                };
                for row in rows {
                    res.submissions.push(Submission {
                        id: row.get(0),
                        tim: row.get(1),
                        user: row.get(3),
                        source: "".to_string(),
                    });
                }
                return HttpResponse::Ok().json(res);
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Challenge {
        id: i32,
        rated: i32,
        tim: String,
        stat: String,
        user1: String,
        user2: String,
        user1_score: String,
        user2_score: String,
        output: String,
    }
    #[get("/challenge-info/{id}")]
    pub fn challenge_info(web::Path(id): web::Path<String>) -> HttpResponse {
        let DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
        let mut conn = Client::connect(&DATABASE_URL, NoTls).unwrap();
        let rows=conn.query("SELECT id,rated,tim_st,stat,user1_id,user2_id,user1_score,user2_score,opt FROM challenges WHERE id=$1",&[&(id.parse::<i32>().unwrap())]).unwrap();

        for row in rows {
            let mut res: Challenge = Challenge {
                id: row.get(0),
                rated: row.get(1),
                tim: row.get(2),
                stat: row.get(3),
                user1: row.get(4),
                user2: row.get(5),
                user1_score: row.get(6),
                user2_score: row.get(7),
                output: row.get(8),
            };
            return HttpResponse::Ok().json(res);
        }
        return HttpResponse::BadRequest().json({});
    }

    #[get("/challenges-list/{user}")]
    pub fn challenges_list(web::Path(user): web::Path<String>) -> HttpResponse {
        let DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
        let mut conn = Client::connect(&DATABASE_URL, NoTls).unwrap();

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ChallengesListResponse {
            challenges: Vec<Challenge>,
        };

        if user.clone() != "all" {
            let rows = conn
                .query("SELECT id,rated,tim_st,stat,user1_id,user2_id,user1_score,user2_score FROM challenges WHERE user1_id=$1 OR user2_id=$1 ORDER BY id DESC",&[&user.clone()]).unwrap();

            let mut res: ChallengesListResponse = ChallengesListResponse { challenges: vec![] };

            for row in rows {
                res.challenges.push(Challenge {
                    id: row.get(0),
                    rated: row.get(1),
                    tim: row.get(2),
                    stat: row.get(3),
                    user1: row.get(4),
                    user2: row.get(5),
                    user1_score: row.get(6),
                    user2_score: row.get(7),
                    output: "".to_string(),
                })
            }
            return HttpResponse::Ok().json(res);
        } else {
            let rows = conn
                .query("SELECT id,rated,tim_st,stat,user1_id,user2_id,user1_score,user2_score FROM challenges ORDER BY id DESC",&[])
                .unwrap();

            let mut res: ChallengesListResponse = ChallengesListResponse { challenges: vec![] };

            for row in rows {
                res.challenges.push(Challenge {
                    id: row.get(0),
                    rated: row.get(1),
                    tim: row.get(2),
                    stat: row.get(3),
                    user1: row.get(4),
                    user2: row.get(5),
                    user1_score: row.get(6),
                    user2_score: row.get(7),
                    output: "".to_string(),
                })
            }
            return HttpResponse::Ok().json(res);
        }
    }

    #[get("/users")]
    pub fn users() -> HttpResponse {
        let DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
        let mut conn = Client::connect(&DATABASE_URL, NoTls).unwrap();
        let rows = conn
            .query("SELECT id, rating FROM users ORDER BY rating DESC", &[])
            .unwrap();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct User {
            id: String,
            rating: i32,
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Users {
            users: Vec<User>,
        }
        let mut res = Users { users: vec![] };
        for row in rows {
            res.users.push(User {
                id: row.get(0),
                rating: row.get(1),
            });
        }
        return HttpResponse::Ok().json(res);
    }

    #[get("/top-rating-history")]
    pub fn top_rating_history()->HttpResponse{
        let DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
        let mut conn = Client::connect(&DATABASE_URL, NoTls).unwrap();
        let rows=conn.query("SELECT * FROM ratinghistory WHERE user_id in (SELECT id FROM users WHERE rating >= (SELECT rating FROM users ORDER BY rating DESC LIMIT 1 OFFSET 9)) ORDER BY tim_num;", &[]).unwrap();

        #[derive(Debug, Serialize, Deserialize)]
        pub struct History{
            tim_num: i32,
            user_id: String,
            rating: i32
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Res{
            history: Vec<History>
        }
        let mut res:Res=Res{history: vec![]};
        for row in rows{
            res.history.push(History{
                tim_num: row.get(0),
                user_id: row.get(1),
                rating: row.get(2)
            });
        }
        return HttpResponse::Ok().json(res);
    }
    #[get("/submitted-users")]
    pub fn submitted_users()->HttpResponse{
        let DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
        let mut conn = Client::connect(&DATABASE_URL, NoTls).unwrap();
        let rows=conn.query("SELECT id, rating FROM users WHERE (SELECT COUNT(*) FROM submissions WHERE user_id=users.id)>=1 ORDER BY rating DESC", &[]).unwrap();
        #[derive(Debug, Serialize, Deserialize)]
        pub struct User {
            id: String,
            rating: i32,
        }
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Users {
            users: Vec<User>,
        }
        let mut res = Users { users: vec![] };
        for row in rows {
            res.users.push(User {
                id: row.get(0),
                rating: row.get(1),
            });
        }
        return HttpResponse::Ok().json(res);
    }
}
