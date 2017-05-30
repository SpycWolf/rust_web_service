extern crate iron;

use std::collections::HashMap;

use iron::prelude::*;
use iron::{Handler};
use iron::status;

struct Router {
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    fn new() -> Self{
        Router { routes: HashMap::new() }
    }
    fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}

fn main() {
    let mut router = Router::new();

    router.add_route("hello".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World")))
    });

    router.add_route("error".to_string(), |_: &mut Request| {
        Ok(Response::with(status::BadRequest))
    });

    let host = "localhost:3000";
    
    println!("binding on {}", host);
    Iron::new(router).http(host).unwrap();
}
