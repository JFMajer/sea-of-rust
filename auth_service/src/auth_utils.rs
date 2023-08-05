pub mod auth_utils {
    pub fn login(creds: models::Credentials) {
        crate::database::database::get_user();
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