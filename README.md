# Actix_GraphQl_API

# Juniper

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
  competition {
    competitionName
    teams {
      teamName
      matchs {
        away
        homeGoals
        awayGoals
        scoreHome
        scoreAway
      }
    }
  }
  classification {
    teamName
    teamGoals
    competionsScore
  }
}
```
_Result:_
```json
{
  "data": {
    "competition": [
      {
        "competitionName": "uefa champions league",
        "teams": [
          {
            "teamName": "FC Barcelona",
            "matchs": [
              {
                "away": "Paris Saint‑Germain F.C.",
                "homeGoals": 2,
                "awayGoals": 0,
                "scoreHome": 3,
                "scoreAway": 0
              },
              {
                "away": "Manchester City F.C.",
                "homeGoals": 0,
                "awayGoals": 2,
                "scoreHome": 0,
                "scoreAway": 3
              },
              {
                "away": "Borussia Dortmund",
                "homeGoals": 3,
                "awayGoals": 2,
                "scoreHome": 3,
                "scoreAway": 0
              }
            ]
          },
          {
            "teamName": "Paris Saint‑Germain F.C.",
            "matchs": [
              {
                "away": "FC Barcelona",
                "homeGoals": 0,
                "awayGoals": 2,
                "scoreHome": 0,
                "scoreAway": 3
              },
              {
                "away": "Manchester City F.C.",
                "homeGoals": 0,
                "awayGoals": 4,
                "scoreHome": 0,
                "scoreAway": 3
              },
              {
                "away": "Borussia Dortmund",
                "homeGoals": 1,
                "awayGoals": 2,
                "scoreHome": 0,
                "scoreAway": 3
              }
            ]
          },
          {
            "teamName": "Manchester City F.C.",
            "matchs": [
              {
                "away": "FC Barcelona",
                "homeGoals": 2,
                "awayGoals": 0,
                "scoreHome": 3,
                "scoreAway": 0
              },
              {
                "away": "Paris Saint‑Germain F.C.",
                "homeGoals": 4,
                "awayGoals": 0,
                "scoreHome": 3,
                "scoreAway": 0
              },
              {
                "away": "Borussia Dortmund",
                "homeGoals": 2,
                "awayGoals": 2,
                "scoreHome": 1,
                "scoreAway": 1
              }
            ]
          },
          {
            "teamName": "Borussia Dortmund",
            "matchs": [
              {
                "away": "FC Barcelona",
                "homeGoals": 2,
                "awayGoals": 3,
                "scoreHome": 0,
                "scoreAway": 3
              },
              {
                "away": "Paris Saint‑Germain F.C.",
                "homeGoals": 2,
                "awayGoals": 1,
                "scoreHome": 3,
                "scoreAway": 0
              },
              {
                "away": "Manchester City F.C.",
                "homeGoals": 2,
                "awayGoals": 2,
                "scoreHome": 1,
                "scoreAway": 1
              }
            ]
          }
        ]
      }
    ],
    "classification": [
      {
        "teamName": "Manchester City F.C.",
        "teamGoals": 8,
        "competionsScore": 7
      },
      {
        "teamName": "FC Barcelona",
        "teamGoals": 5,
        "competionsScore": 6
      },
      {
        "teamName": "Borussia Dortmund",
        "teamGoals": 6,
        "competionsScore": 4
      },
      {
        "teamName": "Paris Saint‑Germain F.C.",
        "teamGoals": 1,
        "competionsScore": 0
      }
    ]
  }
}
```


