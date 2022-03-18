pub mod auth {
    use super::super::misc;
    use actix_web::{get, post, web, HttpResponse, Responder};
    use jsonwebtoken::{
        dangerous_insecure_decode_with_validation, decode, DecodingKey, TokenData, Validation,
    };
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use md5;
    use postgres::{Client, NoTls};

    use serde::{Deserialize, Serialize};
    use std::env;

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        id: String,
        exp: usize,
    }
    pub fn verify_token(token: String) -> Result<String, String> {
        let result: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode::<Claims>(
            token.as_str(),
            &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_ref()),
            &Validation::default(),
        );
        if let Err(error) = result {
            return Err("ERROR".to_string());
        } else {
            let id: String = result.unwrap().claims.id;
            return Ok(id);
        }
    }
    fn issue_token(id: String) -> String {
        let data: Claims = Claims {
            id: id.clone(),
            exp: misc::misc::current_time_num() as usize + 8640000,
        };
        let token: String = encode(
            &Header::default(),
            &data,
            &EncodingKey::from_secret(env::var("JWT_KEY").unwrap().as_ref()),
        )
        .unwrap();
        return token;
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoginCredential {
        id: String,
        pw: String,
    }
    #[post("/login")]
    pub fn login(cred: web::Json<LoginCredential>) -> HttpResponse {
        let DATABASE_URL: String = env::var("DATABASE_URL").unwrap();

        let hashed_pw: String = format!("{:x}", md5::compute(cred.pw.clone()));

        let mut conn = Client::connect(&DATABASE_URL, NoTls).unwrap();
        let rows = conn
            .query(
                "SELECT * FROM users WHERE id=$1 AND pw=$2",
                &[&cred.id.clone(), &hashed_pw.clone()],
            )
            .unwrap();

        let mut cnt = 0;
        for row in rows {
            cnt += 1;
        }

        if cnt == 1 {
            // Success
            #[derive(Debug, Serialize, Deserialize)]
            struct Response {
                token: String,
            }
            return HttpResponse::Ok().json(Response {
                token: issue_token(cred.id.clone()),
            });
        } else {
            // Unauthorized
            return HttpResponse::Unauthorized().json({});
        }
    }
}
