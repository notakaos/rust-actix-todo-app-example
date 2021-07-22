use actix_web::{get, route, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[route("/{id}/{name}/index.html", method = "GET", method = "HEAD")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}\n", name, id)
}

#[derive(Serialize)]
struct Todo {
    title: String,
    checked: bool,
}

#[derive(Serialize)]
struct Todos {
    todos: Vec<Todo>,
}

#[get("/")]
async fn todo_index() -> HttpResponse {
    // HttpResponse::Ok().json(Todo {
    //     title: "Hello".to_string(),
    //     checked: true,
    // })
    // let mut todos:Vec<Todo> = Vec::new();

    let mut response = Todos { todos: Vec::new() };

    response.todos.push(Todo {
        title: "Todo 1".to_string(),
        checked: true,
    });
    response.todos.push(Todo {
        title: "Todo 2".to_string(),
        checked: false,
    });
    response.todos.push(Todo {
        title: "Todo 3".to_string(),
        checked: true,
    });

    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!!");

    // let x = 3.1415;
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);

    HttpServer::new(|| App::new().service(index).service(todo_index))
        .keep_alive(75)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
