use juniper::{EmptyMutation, RootNode};
use juniper::{GraphQLObject};
use serde::{Serialize,Deserialize};
use serde_json;
use mongodb::bson;
mod db_connector;

#[derive(Serialize, Deserialize,GraphQLObject)]
struct  Match {
    away: String,
    home_goals: i32,
    away_goals: i32,
    score_home: i32,
    score_away: i32
    }
      
#[derive(Serialize, Deserialize,GraphQLObject)]
struct Team {
    team_name: String,
    matchs : Vec<Match>,
    } 
 
#[derive(Serialize, Deserialize,GraphQLObject)]
#[graphql(description = "A Competition Of Certen FootBall Games ")]
struct Competition {
    competition_name: String,
    teams: Vec<Team>,
    }

#[derive(Serialize, Deserialize,GraphQLObject)]
#[graphql(description = "A Classification Of Certen FootBall Teams ")]
struct Classification{
    team_name: String,
    team_goals: i32,
    competions_score: i32,
    }

pub struct QueryRoot;
    
    #[juniper::object]
    impl QueryRoot {
        fn competition() -> Vec<Competition> {
            let doc=db_connector::get_document();
            let mut document_place_holder=bson::Document::new();
            match doc{
                Ok(doc)=>{document_place_holder=doc}
                Err(e)=>println!("error:{}",e),
            };
            let competition:Competition=bson::from_document(
                document_place_holder
              ).unwrap();
            vec![competition]//returns vector of Competition
        }

        fn classification()-> Vec<Classification>{
            let doc=db_connector::get_document();
            let mut document_place_holder=bson::Document::new();
            match doc{
                Ok(doc)=>{document_place_holder=doc}
                Err(e)=>println!("error:{}",e),
            };
            let serlized=serde_json::to_string(&document_place_holder).unwrap();
            let com:Competition=serde_json::from_str(&serlized).unwrap();
            let mut classified:Vec<Classification>=Vec::new();
            for i in 0..4{
                let mut score:i32=0;
                let mut goals:i32=0;
                for j in 0..3{
                    goals=goals+com.teams[i].matchs[j].home_goals;
                    score=score+com.teams[i].matchs[j].score_home;
                }
                classified.push(
                    Classification{
                        team_name:com.teams[i].team_name.to_owned(),
                        team_goals:goals,
                        competions_score:score,
              });
            }
            classified.sort_by(|low,max| max.competions_score.cmp(&low.competions_score));
            classified//returns vector of Classification
        }
    }

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
      Schema::new(QueryRoot {}, EmptyMutation::new())
    }