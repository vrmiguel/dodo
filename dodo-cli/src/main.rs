pub use dodo_internals as dodo;

use dodo::{Priority, Task};
use std::fs::File;

fn main() {
    let output = File::open("hehehe").unwrap();

    let task = dodo::Task {
        name: "Hey".into(),
        is_done: true,
        description: "Hahaha".into(),
        creation_date: dodo::utils::today(),
        due_date: None,
        priority: Priority::High,
        checklist: vec![].into_iter().collect(),
    };

    // let _ = dbg!(bincode::serialize_into(&output, &task));

    let deserialized: Task = bincode::deserialize_from(output).unwrap();

    assert_eq!(task, deserialized);
}
