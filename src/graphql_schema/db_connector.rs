use mongodb::{bson::{self, Document, doc}};
use std::error::Error;
use tokio::{self, stream::StreamExt};

#[tokio::main]
pub async fn get_document() -> Result<Document, Box<dyn Error>> {
    let client = mongodb::Client::with_uri_str("mongodb://localhost:27017").await?;
    //make sure your database name is Tournoi or if any you wont it to connect to
    let db = client.database("Tournoi");
    let collection=db.collection("Competition");
    // remove the _ sign to change the docs to call it later in insert_many function add prefix _ to avoide warnings in runtime
    let _docs = vec![
    doc! {
        "_id":bson::oid::ObjectId::new(),
        "competition_name": "uefa champions league",
        "teams": [
            {
            "team_id":bson::oid::ObjectId::new(),
            "team_name":"FC Barcelona",
            "matchs":[
                {
                    "match_id":bson::oid::ObjectId::new(),
                    "away": "Paris Saint‑Germain F.C.",
                    "home_goals": 2,
                    "away_goals": 0,
                    "score_home" : 3,
                    "score_away" : 0,
                },
                {
                    "match_id":bson::oid::ObjectId::new(),
                    "away": "Manchester City F.C.",
                    "home_goals": 0,
                    "away_goals": 2,
                    "score_home" : 0,
                    "score_away" : 3,
                },
                {
                    "match_id":bson::oid::ObjectId::new(),
                    "away": "Borussia Dortmund",
                    "home_goals": 3,
                    "away_goals": 2,
                    "score_home" : 3,
                    "score_away" : 0,
                }
            ]
        },
         {
            "team_id":bson::oid::ObjectId::new(),
            "team_name":"Paris Saint‑Germain F.C.",
            "matchs":[
                {
                    "match_id": bson::oid::ObjectId::new(),
                    "away": "FC Barcelona",
                    "home_goals": 0,
                    "away_goals": 2,
                    "score_home" : 0,
                    "score_away" : 3,
                },
                {
                    "match_id":bson::oid::ObjectId::new(),
                    "away": "Manchester City F.C.",
                    "home_goals": 0,
                    "away_goals": 4,
                    "score_home" : 0,
                    "score_away" : 3,
                },
                {
                    "match_id":bson::oid::ObjectId::new(),
                    "away": "Borussia Dortmund",
                    "home_goals": 1,
                    "away_goals": 2,
                    "score_home" : 0,
                    "score_away" : 3,
                }
            ]
        }, 
         {
            "team_id":bson::oid::ObjectId::new(),
            "team_name":"Manchester City F.C.",
            "matchs":[
                {
                    "match_id":bson::oid::ObjectId::new(),
                    "away": "FC Barcelona",
                    "home_goals": 2,
                    "away_goals": 0,
                    "score_home" : 3,
                    "score_away" : 0,
                },
                {
                    "match_id": bson::oid::ObjectId::new(),
                    "away": "Paris Saint‑Germain F.C.",
                    "home_goals": 4,
                    "away_goals": 0,
                    "score_home" : 3,
                    "score_away" : 0,
                },
                {
                    "match_id": bson::oid::ObjectId::new(),
                    "away": "Borussia Dortmund",
                    "home_goals": 2,
                    "away_goals": 2,
                    "score_home" : 1,
                    "score_away" : 1,
                }
            ]
        }, 
         {
            "team_id":bson::oid::ObjectId::new(),
            "team_name":"Borussia Dortmund",
            "matchs":[
                {
                    "match_id":bson::oid::ObjectId::new(),
                    "away": "FC Barcelona",
                    "home_goals": 2,
                    "away_goals": 3,
                    "score_home" : 0,
                    "score_away" : 3,
                },
                {
                    "match_id":bson::oid::ObjectId::new(),
                    "away": "Paris Saint‑Germain F.C.",
                    "home_goals": 2,
                    "away_goals": 1,
                    "score_home" : 3,
                    "score_away" : 0,
                },
                {
                    "match_id":bson::oid::ObjectId::new(),
                    "away": "Manchester City F.C.",
                    "home_goals": 2,
                    "away_goals": 2,
                    "score_home" : 1,
                    "score_away" : 1,
                }
            ]
        }]
      }
    ];
    //remove comments if the code is first time executed to insert data in to database  Tournoi and rerun important!
    //then stop the server and start to avoide rehiting the database multiple times wich causes inserting a document each time
    //the endpoint is hitied !!working on finding a solution!!

    //collection.insert_many(docs,None).await?;

    let mut cursor = collection.find(None, None).await?;

   // Iterate over the results of the cursor and retrive the first document 
   let  mut doc=Document::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                 doc=document;
                 break;
            }
            Err(e) => return Err(e.into())
            
        }
    };
    //return document
    Ok(doc)    

   }


