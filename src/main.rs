extern crate chrono;

use chrono::Local;
use std::time::Instant;

#[derive(Debug)] // This is for formating purposes
struct Incident {
    _id: String,
    start_time: u64,
    active: bool,
}

struct TopicPartition(String, u32);

fn main() {
    // Structs:
    let incident_1 = build_incident(String::from("someId"), 156277777, true);
    let incident_2 = Incident {
        _id: String::from("other_id"),
        ..incident_1
    };
    let tp = TopicPartition(String::from("correlation.workday.old-notifications"), 0);
    println!("Incident 1 id: {:?}", incident_1);
    print!("incident 2 id: {}\n", incident_2._id);
    print!("incident 2 start_time: {}\n", incident_2.start_time);
    print!("incident 2 is_active: {}\n", incident_2.active);
    print!("topic: {}; partition: {}\n", tp.0, tp.1);

    // Dates and Time:
    print_date();

    // Area calc
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

    // Enums:
    enum_stuff();
}

fn build_incident(_id: String, start_time: u64, active: bool) -> Incident {
    Incident {
        _id,
        start_time,
        active,
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
    #[derive(Debug)]
    enum IncidentStatus {
        Ok,
    }
    #[derive(Debug)]
    struct Incident {
        _id: String,
        start_time: Instant,
        active: bool,
        status: IncidentStatus,
    }

    let critical_incident = Incident {
        _id: String::from("some-id"),
        start_time: Instant::now(),
        active: true,
        status: IncidentStatus::Ok,
    };

    println!(" Incident: {:?}", critical_incident);
}
