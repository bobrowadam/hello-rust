extern crate chrono;
extern crate uuid;

use chrono::Local;
use std::time::{Instant, SystemTime};
use uuid::Uuid;

mod types;

struct TopicPartition(String, u32);
use types::incident::{Incident, IncidentStatus};

fn main() {
    structures();
    print_date();
    area_calc();
    enum_stuff();
    match_enums();
    event::alert::print_alert();
    notification::call_match_enums();
}
// Structs:
fn structures() {
    let incident_1 = build_incident(
        Uuid::new_v5(&Uuid::NAMESPACE_DNS, "foo".as_bytes()),
        SystemTime::now(),
        true,
        IncidentStatus::Ok,
    );
    let incident_2 = Incident {
        _id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, "foo".as_bytes()),
        ..incident_1
    };

    let tp = TopicPartition(String::from("correlation.workday.old-notifications"), 0);
    print!("incident 2 id: {}\n", incident_2._id);
    // print!("incident 2 start_time: {}\n", incident_2.start_time);
    print!("incident 2 is_active: {}\n", incident_2.active);
    print!("topic: {}; partition: {}\n", tp.0, tp.1);
}

fn area_calc() {
    let rectanglge = (5, 5);
    println!("Area of 5 * 5 is {}", area(rectanglge));
    let rect1 = Rectangle {
        width: 10,
        hight: 5,
    };
    let rect2 = Rectangle {
        width: 15,
        hight: 4,
    };

    println!(
        "rect1 area ({}) is greater than rect2 ({})? {}",
        rect1.area(),
        rect2.area(),
        rect1.area_greater_than(&rect2)
    );
}

fn build_incident(
    _id: Uuid,
    start_time: SystemTime,
    active: bool,
    status: IncidentStatus,
) -> types::incident::Incident {
    Incident {
        _id,
        start_time,
        active,
        status,
    }
}

fn print_date() {
    let now = Instant::now();
    let local_time = Local::now();
    println!("Time now is: {:?}\nLocal time is: {:?}", now, local_time);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    hight: u32,
}

impl Rectangle {
    fn area_greater_than(&self, other: &Rectangle) -> bool {
        (self.width * self.hight) > (other.width * other.hight)
    }
    fn area(&self) -> u32 {
        self.width * self.hight
    }
}

fn enum_stuff() {
    let ok_incident = Incident {
        _id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, "foo".as_bytes()),
        start_time: SystemTime::now(),
        active: true,
        status: IncidentStatus::Ok,
    };

    let not_ok_incident = Incident {
        _id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, "foo".as_bytes()),
        start_time: SystemTime::now(),
        active: true,
        status: IncidentStatus::NotOk {
            message: String::from("Bad gateway"),
            status: String::from("Critical"),
        },
    };

    println!(" Incident: {:?}", ok_incident);
    println!(" Incident: {:?}", not_ok_incident);
}

fn match_enums() {
    let incident_3 = Incident {
        _id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, "foo".as_bytes()),
        start_time: SystemTime::now(),
        active: true,
        status: IncidentStatus::NotOk {
            message: String::from("high disk space"),
            status: String::from("Warning"),
        },
    };
    match incident_3.status {
        IncidentStatus::NotOk {
            ref message,
            ref status,
        }
            if "high disk space" == message =>
        {
            println!("Not Ok!\nmessage: {},\nstatus: {}", message, status)
        }
        IncidentStatus::NotOk {
            ref message,
            ref status,
        } => println!(
            "Its kind of ok... \nmessage: {},\nstatus: {}",
            message, status
        ),
        IncidentStatus::Ok => println!("Result 2"),
    };
}

mod event {
    pub mod alert {
        pub fn print_alert() {
            println!("{}", String::from("Alert is bla"))
        }
    }
}

mod notification {
    pub fn call_match_enums() {
        println!(
            "{}",
            String::from("Calling call_match_enums from notificaqtion mod")
        );
        super::match_enums();
    }
}
