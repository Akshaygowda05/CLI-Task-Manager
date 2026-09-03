use uuid::Uuid;
use chrono::{DateTime,Utc};
use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub enum Priority {
Low,
Medium,
High
}


#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub enum Status {
    Todo,
    Done
}

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]

pub struct Task {
   pub id:Uuid,
   pub title:String,
   pub priority : Priority,
   pub status:Status,
   pub created_at :DateTime<Utc>

}

 impl Task{
    pub fn new (title:&str,priority:Priority)->Self{
        Task {
            id:Uuid::new_v4(),
            title:title.to_string(),
            priority,
            status:Status::Todo,
            created_at:Utc::now(),

        }


    }
    pub fn mark_done(&mut self){
        self.status = Status::Done;

    }

    pub fn is_done(&self)->bool{
        self.status==Status::Done

    }
}