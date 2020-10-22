# Actix_GraphQl_API

# Juniper

Wrapping a REST API in GraphQL ,Ignite The https://github.com/MedAmineFouzai/Actix_REST_API Project Make Shure That The DataBase Is Correctly Created
It Will Bind The Server Localy And Create A Graphql Instance From the REST Endpoint (http://localhost:8080/competition/{id}/standings) 
[Juniper](https://github.com/graphql-rust/juniper) integration for Actix web.

## Usage
 - git clone https://github.com/MedAmineFouzai/Actix_GraphQl_API
### server

```bash
cd Actix_GraphQl_API
cargo run 
```

### web client

[http://127.0.0.1:8080/graphiql](http://127.0.0.1:8080/graphiql)

_Query example:_
```graphql
query {
  classification(id: "5f8b4839d42642b5f38d78db") {
    competition {
      id
      name
    }
    standings {
      position
      team {
        teamName
        wins
        draws
        losts
        goalScored
        goalAcuired
        points
      }
    }
  }
}
```
_Result:_
```json
{
  "data": {
    "classification": {
      "competition": {
        "id": "5f8b4839d42642b5f38d78db",
        "name": "UEFA Champions League"
      },
      "standings": [
        {
          "position": 1,
          "team": {
            "teamName": "Manchester United",
            "wins": 4,
            "draws": 1,
            "losts": 1,
            "goalScored": 15,
            "goalAcuired": 11,
            "points": 13
          }
        },
        {
          "position": 2,
          "team": {
            "teamName": "Real Madrid",
            "wins": 2,
            "draws": 2,
            "losts": 2,
            "goalScored": 10,
            "goalAcuired": 8,
            "points": 8
          }
        },
        {
          "position": 3,
          "team": {
            "teamName": "Bayern Munich",
            "wins": 2,
            "draws": 1,
            "losts": 3,
            "goalScored": 9,
            "goalAcuired": 8,
            "points": 7
          }
        },
        {
          "position": 4,
          "team": {
            "teamName": "FC Barcelone",
            "wins": 1,
            "draws": 2,
            "losts": 3,
            "goalScored": 5,
            "goalAcuired": 12,
            "points": 5
          }
        }
      ]
    }
  }
}
```


