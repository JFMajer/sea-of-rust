#![allow(dead_code, unused_variables)]

mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        return Status::Connected
    }

    pub fn get_user() {
        //TODO: get user from database
    }
}

mod auth_utils {
    pub fn login(creds: models::Credentials) {
        crate::database::get_user();
    }
    
    fn logout() {
        //TODO: logout user
    }

    pub mod models {
        pub struct Credentials {
            username: String,
            passowrd: String,
        }
    }
}

use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    match database::connect_to_database() {
        Status::Connected => auth_utils::login(creds),
        Status::Interrupted => println!("Interrupted while connecting to database"),
    }
}

