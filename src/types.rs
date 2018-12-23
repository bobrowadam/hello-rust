pub mod incident {
    use std::time::SystemTime;
    extern crate chrono;
    extern crate uuid;
    use uuid::Uuid;
    #[derive(Debug)]
    pub struct Incident {
        pub _id: Uuid,
        pub start_time: SystemTime,
        pub active: bool,
        pub status: IncidentStatus,
    }
    #[derive(Debug)]
    pub enum IncidentStatus {
        Ok,
        NotOk { message: String, status: String },
    }
}
