use std::time::Duration;
use tracing_subscriber::prelude::*;
use rand::prelude::*;
use tokio::time::sleep;
use tokio::task::JoinSet;
use tracing::instrument;

#[instrument]
async fn get_students_courses(student_id: u16) {
    tracing::info!("We are fetching {student_id}");
    let timeout_duration = thread_rng().gen_range(10..1000);
    sleep(Duration::from_millis(timeout_duration)).await;

    if random() {
        tracing::info!("we succeded {student_id}");
        // println!("we succeded {student_id}");
        // ()
    } else {
        tracing::info!("we failed {student_id}");
        // println!("we failed {student_id}");
        // panic!("Failed to get student")
    }
}


#[tokio::main]
async fn main() {
    let fmt_layer = tracing_subscriber::fmt::Layer::default();

    let subscriber = tracing_subscriber::registry()
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();


    let student_ids = [134, 524, 244, 356, 621, 413, 777];
    let mut set = JoinSet::new();

    for student_id in student_ids {
        // println!("We are fetching {student_id}");
        set.spawn(get_students_courses(student_id));
    }

    while set.join_next().await.is_some() {}
    
}
