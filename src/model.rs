use uuid::Uuid;
use chrono::{DateTime,Utc};

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub enum Priority {
Low,
Medium,High
}


#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub enum Status {
    Todo,
    Done
}

pub struct Task {
   pub id:Uuid,
   pub title:String,
   pub priority : Priority,
   pub status:Status,
   pub created_at :DateTime<Utc>

}

imple Task{
    pub fn new (title:&str,priority:Priority)->Self{
        Task {
            id:Uuid::new_v4(),
            title:title.to_string(),
            priority,
            status:Status::Todo,
            created_at:Utc::npw()

        }


    }

    pub fn mark_done(&mut self){
        self.status = Status::Done;

    }

    pub fn is_done($self)->bool{
        self.status==Status::Done

    }
}