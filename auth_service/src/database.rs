pub mod database {
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