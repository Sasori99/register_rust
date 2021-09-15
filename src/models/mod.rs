pub mod test;
pub mod user;

use actix_web::{web::{Data, Path}, HttpResponse, web};
use crate::models::user::{RocksDB, KVStore, User};

pub async fn get(key: Path<String>, db: Data<RocksDB>) -> HttpResponse {
    match &db.find(&key.into_inner()) {
        Some(v) => {
            HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&v).unwrap())
        }
        None    => HttpResponse::NotFound().content_type("application/json").finish()
    }
}

pub async fn post(db:   Data<RocksDB>, mut body: web::Json<User>) -> HttpResponse {

    match &db.find_by_username(&body.username) {
        Some(..) => HttpResponse::InternalServerError().content_type("application/json").body("Username already exists!!!!"),
        None => {
            match &db.find_by_email(&body.email) {
                Some(..) => HttpResponse::InternalServerError().content_type("application/json").body("Email already exists!!!!"),
                None => {
                    let mut check = true;
                    let mut uuid = rand::random::<u128>().to_string();
                    while check {
                        let user = &db.find(&uuid);
                        match user {
                            Some(v) => {
                                uuid = rand::random::<u128>().to_string();
                            }
                            None => {
                                check = false;
                            }
                        }
                    }

                    body.id = uuid;
                    println!("{:?}", body);
                    match &db.save(&body.id, &body) {
                        true => {
                            HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&body.into_inner()).unwrap())
                        }
                        false => HttpResponse::InternalServerError().content_type("application/json").finish()
                    }
                }
            }
        }
    }
}

pub async fn put(db:   Data<RocksDB>, body: web::Json<User>) -> HttpResponse {
    match &body.id == "" {
        true => HttpResponse::InternalServerError().content_type("application/json").body("Id is not exists!!!!"),
        false => {
            match &db.find(&body.id) {
                Some(v) => {
                    let user_username = &db.find_by_username(&body.username);
                    match user_username.is_some() && user_username.as_ref().unwrap().id != body.id {
                        true => HttpResponse::InternalServerError().content_type("application/json").body("Username already exists!!!!"),
                        false => {
                            let user_email = &db.find_by_email(&body.email);
                            match user_email.is_some() && &user_email.as_ref().unwrap().id != &body.id {
                                true => HttpResponse::InternalServerError().content_type("application/json").body("Email already exists!!!!"),
                                false => {
                                    match &db.save(&body.id, &body) {
                                        true => {
                                            HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&body.into_inner()).unwrap())
                                        }
                                        false => HttpResponse::InternalServerError().content_type("application/json").finish()
                                    }
                                }
                            }
                        }
                    }
                }
                None    => HttpResponse::InternalServerError().content_type("application/json").body("Id is not exists!!!!")
            }
        }
    }
}


pub async fn delete(key: Path<String>, db: Data<RocksDB>) -> HttpResponse {
    match &db.delete(&key.into_inner()) {
        true  => HttpResponse::NoContent().content_type("application/json").finish(),
        false => HttpResponse::InternalServerError().content_type("application/json").finish()
    }
}
