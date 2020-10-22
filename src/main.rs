extern crate juniper;
use std::io;
use std::sync::Arc;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::{http::GraphQLRequest};
use juniper::{http::playground::playground_source};//or use graphiql::graphiql_source for graphiql 
mod graphql_schema;
use crate::graphql_schema::{create_schema,Schema};

    fn graphql(
        st: web::Data<Arc<Schema>>,
        data: web::Json<GraphQLRequest>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        web::block(move || {
            let res = data.execute(&st, &());
            Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
        })
        .map_err(Error::from)
        .and_then(|user| {
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user))
        })
    }

    fn graphiql() -> HttpResponse {
        let html = playground_source("http://localhost:8080/graphql");
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)
    }
   
    fn main() -> io::Result<()> {
         
        let schema = std::sync::Arc::new(create_schema());
        HttpServer::new(move || {
            App::new()
                .data(schema.clone())
                .service(web::resource("/graphql").route(web::post().to_async(graphql)))
                .service(web::resource("/graphiql").route(web::get().to(graphiql)))
        })
        .bind("localhost:8000")?
        .run()
    }

    

