use std::fs;
use std::path::Path;

use crate::error::TaskError;
use crate::models::Task;

const SAVE_FILE :&str = "task.json";

pub fn load_task() -> Result<Vec<Task>,TaskError>{
    // first i need to check wheather fuile existed or not

   let path = Path::new(SAVE_FILE);

   if(!path.exits(){
        return OK(vec![])
   })

   let contents = fs::read_to_string(SAVE_FILE)?;
   let tasks = serde_json::from_str(&contents)?;

   Ok(tasks)
    
}