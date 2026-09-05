mod model;
mod error;
use model::{Priority,Task};
use error::{TaskError};


fn main(){
    let mut task = Task::new("Buy Milk",Priority::Medium);
    println!("{:?}",task);

    task.mark_done();
    println!("{}",task.is_done());
}