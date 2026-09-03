mod model;
use model::{Priority,Task};


fn main(){
    let mut task = Task::new("Buy Milk",Priority::Medium);
    println!("{:?}",task);

    task.mark_done();
    println!("{}",task.is_done());
}