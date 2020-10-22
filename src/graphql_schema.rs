use juniper::{EmptyMutation, RootNode};
use juniper::{GraphQLObject};
use isahc::prelude::*;
use serde::{Serialize,Deserialize};



#[derive(Debug,Serialize, Deserialize,GraphQLObject)]
struct  Competition {
    id: String,
    name: String
}
#[derive(Debug,Serialize, Deserialize,GraphQLObject)]
struct Team{
    team_name:String,
    wins: i32,
    draws : i32,
    losts: i32,
    goal_scored: i32,
    goal_acuired: i32,
    points: i32
}
#[derive(Debug,Serialize, Deserialize,GraphQLObject)]
struct Position {
    position: i32,
    team:Team
}

#[derive(Debug,Serialize, Deserialize,GraphQLObject)]
pub struct Classification{
    competition: Competition,
    standings: Vec<Position>
}


pub struct QueryRoot;
    
    #[juniper::object]
    impl QueryRoot {
       fn classification (id:String)->Classification{
        let mut  data:String="".to_string();
        let mut response =isahc::get( format!("http://localhost:8080/competition/{}/standings",id));
        match  response {
            Ok(mut res)=>{
                data=res.text().ok().unwrap();
            }
            Err(e)=>{
                println!("error :{}",e);
            }
        }
        
        let classfication:Classification=serde_json::from_str(&data).unwrap();
        classfication
    }

    }

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
      Schema::new(QueryRoot {}, EmptyMutation::new())
    }