use std::{collections::HashMap, sync::{Arc, Mutex}};

use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder, error::ErrorNotFound};
use serde::{Deserialize, Serialize};
// -------------------------------------------------------------------------
// NOTE: Some part of the example are commented as a log for didactical use.
//       this would log the sequence of steps I used here, sometimes we write a code than change it later for 
//       including other features. The logs might record basic steps. 
// -------------------------------------------------------------------------
// 1) Creating our basic http server 
//
// #[actix_web::main]   // or #[tokio::main]
// async fn main()  -> std::io::Result<()> {
// 
//     let port = 8000;
//     println!("Starting server on port {}", port);
// 
//     HttpServer::new(|| App::new())
//         .bind(("127.0.0.1", port)).expect("error while binding server!")
//         .workers(2)   // an estimate of the default amount of parallelism a program should use, often corresponds to the amount of CPUs (but it may diverge in some cases)
//         .run()
//         .await 
// }
// after creating the basic http server, it is time to create our first endpoint. check step 2)
// 
// 1.1) this is a snapshot of our http up to the topic 3)
// #[actix_web::main]   // or #[tokio::main]
// async fn main()  -> std::io::Result<()> {
//     let port = 8080;
//     println!("Starting server on port {}", port);
//    // HttpServer::new(|| App::new())                     // 1) http server without any service registered
//    // HttpServer::new(|| App::new().service(greet))      // 2) registering our endpoint
//    HttpServer::new(|| App::new().service(greetuser))     // 3) registering an endpoint with path paramenter
//          // App:  The top-level builder for an Actix Web application.
//         .bind(("127.0.0.1", port))?  //.expect("error while binding server!")
//         .workers(2)   // an estimate of the default amount of parallelism a program should use, often corresponds to the amount of CPUs (but it may diverge in some cases)
//         .run()
//         .await 
// }
// 1.2) this is the http changed for topic 4 
#[actix_web::main]   // or #[tokio::main]
async fn main()  -> std::io::Result<()> {

    let port = 8080;
    println!("Starting server on port {}", port);

    // 4) declaring our DB simulator, 
    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));

   // HttpServer::new(|| App::new())                     // 1) http server without any service registered
   // HttpServer::new(|| App::new().service(greet))      // 2) registering our endpoint
   // HttpServer::new(|| App::new().service(greetuser))     // 3) registering an endpoint with path paramenter
    HttpServer::new(move || { 
            let my_app_data = web::Data::new(user_db.clone());
            // Data: Application data wrapper and extractor.
            App::new()  // App: The top-level builder for an Actix Web application.
                .app_data(my_app_data)
                .service(greet)      //register services (endpoints/ressources)
                .service(greetuser)      
                .service(create_user)
                .service(get_user)
            
        }) 
        .bind(("127.0.0.1", port))?  //.expect("error while binding server!")
        .workers(2)   // an estimate of the default amount of parallelism a program should use, often corresponds to the amount of CPUs (but it may diverge in some cases)
        .run()
        .await 
}
// -------------------------------------------------------------------------
// 2) Creating our first endpoint , 
// the function below is the function that defines our endpoint and it should implement the trait 'Responder'
#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    format!("Hello World!")
}
// Whenever an user visits the '/greet' end point this function will be executed
// after defining our endpoints we ned to register them into our http server. 
// In order to register our endpoint we use the service method provided by the actix_web::App
// - service method: Registers a HTTP service. Http service is any type that implements HttpServiceFactory trait.
//                   Actix Web provides several services implementations:
//                   'Resource' is an entry in resource table which corresponds to requested URL.
//                   'Scope' is a set of resources with common root path.
// We register our endpoint (resource) in our http server 
// -------------------------------------------------------------------------
// 3) now suppose we want to pass a path parameter for our endpoint
//    using the same hello world example lets pass an id of a user to the endpoint
#[actix_web::get("/greet/{id}")]
async fn greetuser(user_id: web::Path<u32>) -> impl Responder {
    format!("Hello {}", user_id)
}
// -------------------------------------------------------------------------
// 4) now lets suppose we want to declare some data (i.e., some rust struct) and we want to return this
//    data in Json format to the user, and we also want to insert some data from the user as a POST request.
//     
// first, lets declare de data: 
// we derive the serde serialize and deserialize macros that generate implementations of the Serialize 
// and Deserialize traits for data structures defined in your crate, ref: https://serde.rs/derive.html
#[derive(Serialize, Deserialize)]    
struct User {
    name: String,
}
// 4.1) for this example, we will simulate a database to store the data, to make it easy we are just using 
// a variable of HashMap type, that will map user ids (u32) to our User struct, like this: 
type UserDb = Arc<Mutex<HashMap<u32, User>>>;
// we declared a type which is Mutex , that means it provides mutually exclusice access to this type, 
// that is wrapped into an Arc, that provides atomic reference counting to make it easy to manage the references to it. 
// note that, arc and mutex are not really the context here, this is just a simulation of a database, in practice 
// we are going to use a real database access, for now we just assume this type db as a simulation of our database.
// now, we go to the http server and declare our database.
// 4.2) then, after declaring the database we will create an endpoint that will handle the POST requests
#[actix_web::post("/newuser")]
async fn create_user(
    user_data: web::Json<User>, 
    db: web::Data<UserDb>
) -> impl Responder {

    let mut db = db.lock().unwrap();   
    let new_id = db.keys().max().unwrap_or(&0) + 1;    // new_id: gets the last id and adds 1 
    let name = user_data.name.clone();              // gets the name in the User struct , from the json
    db.insert(new_id, user_data.into_inner());      // inserts a new user into the DB

    HttpResponse::Created().json(User{ name, })    // responde the requester with the added name 
}
// -------------------------------------------------------------------------
// 5) now we aer going to add another endpoint to read a specific user from our simulated database
#[actix_web::get("/users/{id}")]
async fn get_user(
    user_id: web::Path<u32> , 
    db: web::Data<UserDb>) 
    -> Result<impl Responder, Error> {
        let user_id = user_id.into_inner();
        let db = db.lock().unwrap();

        match db.get(&user_id) {
            Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
            None => Err(ErrorNotFound("User not found")), 
        }
}





// Testing with curl: 
// 2) curl http://127.0.0.1:8080/greet
// 3) curl http://127.0.0.1:8080/greet/13
// 4) curl -H "Content-Type: application/json" --request POST -d '{"name":"bob"}' http://127.0.0.1:8080/newuser
// 5) curl http://127.0.0.1:8080/users/1 