#![allow(dead_code, unused_variables)]

mod database;
mod auth_utils;


use auth_utils::auth_utils::models::Credentials;
use auth_utils::auth_utils::login;
use database::database::Status;
use database::database::connect_to_database;

pub fn authenticate(creds: Credentials) {
    match connect_to_database() {
        Status::Connected => login(creds),
        Status::Interrupted => println!("Interrupted while connecting to database"),
    }
}

