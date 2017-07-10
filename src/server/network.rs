use server::GameEngine;
use server::requests;

use std::sync::{Arc, Mutex};
use std::io::Read;
use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use iron::headers::{Authorization, Basic};

struct Router {
    routes: HashMap<String, Box<Handler>>,
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: String, handler: H)
        where H: Handler
    {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound)),
        }
    }
}

pub fn start(mutex: Arc<Mutex<GameEngine>>) {
    let mut router = Router::new();

    let cloned_engine = mutex.clone();
    router.add_route("world_size".to_owned(),
                     move |_: &mut Request| match requests::world_size(&cloned_engine) {
                         Some(expr) => Ok(Response::with((status::Ok, expr))),
                         None => Ok(Response::with((status::Ok))),
                     });

    let cloned_engine = mutex.clone();
    router.add_route("objects".to_string(), move |req: &mut Request| {
        if !check_username(&req, "admin".to_owned()) {
            return Ok(Response::with((status::Ok))); // TODO: А вот тут должна быть ошибка
        }
        match requests::objects(&cloned_engine) {
            Some(response) => Ok(Response::with((status::Ok, response))),
            None => Ok(Response::with((status::Ok))), // TODO: Заменить на ошибку, хотя вряд ли она может тут возникнуть
        }
    });
    let cloned_engine = mutex.clone();
    router.add_route("object_info".to_string(), move |req: &mut Request| {
        let mut buf = String::new();
        req.body.read_to_string(&mut buf).unwrap();
        println!("{:?} \n {:?}", req, buf);

        match requests::object_info(&cloned_engine, buf, get_username(&req)) {
            Some(response) => Ok(Response::with((status::Ok, response))),
            None => Ok(Response::with((status::Ok))),
        }
    });

    let cloned_engine = mutex.clone();
    router.add_route("move".to_string(), move |req: &mut Request| {
        let mut buf = String::new();
        req.body.read_to_string(&mut buf).unwrap();

        if requests::move_object(&cloned_engine, buf, get_username(&req)) {
            Ok(Response::with((status::Ok)))
        } else {
            Ok(Response::with((status::Ok))) // TODO: Заменить на ошибку
        }
    });

    let cloned_engine = mutex.clone();
    router.add_route("info".to_string(), move |_: &mut Request| {
        match requests::info(&cloned_engine) {
            Some(response) => Ok(Response::with((status::Ok, response))),
            None => Ok(Response::with((status::Ok))), // TODO: Заменить на ошибку, хотя вряд ли она тут будет
        }
    });

    let cloned_engine = mutex.clone();
    router.add_route("radar".to_string(),
                     move |req: &mut Request| match requests::radar(&cloned_engine,
                                                                    get_username(&req)) {
                         Some(response) => Ok(Response::with((status::Ok, response))),
                         None => Ok(Response::with((status::Ok))),
                     });

    let cloned_engine = mutex.clone();
    router.add_route("weapon_fire".to_owned(), move |req: &mut Request| {
        let mut buf = String::new();
        req.body.read_to_string(&mut buf).unwrap();

        if requests::weapon_fire(&cloned_engine, buf, get_username(&req)) {
            Ok(Response::with((status::Ok)))
        } else {
            Ok(Response::with((status::Ok)))
        }
    });

    let cloned_engine = mutex.clone();
    router.add_route("build".to_owned(), move |req: &mut Request| {
        let mut buf = String::new();
        req.body.read_to_string(&mut buf).unwrap();

        if requests::build(&cloned_engine, buf, get_username(&req)) {
            Ok(Response::with((status::Ok)))
        } else {
            Ok(Response::with((status::Ok)))
        }
    });

    Iron::new(router).http("localhost:3000").unwrap();
}

// Возвращает логин пользователя или "unknown" при его отсутствии
fn get_username(req: &Request) -> String {
    match req.headers.get::<Authorization<Basic>>() {
        Some(expr) => expr.username.clone(),
        None => "unknown".to_owned(),
    }
}

fn check_username(req: &Request, username: String) -> bool {
    match req.headers.get::<Authorization<Basic>>() {
        Some(expr) => {
            if expr.username != username {
                return false;
            } else {
                return true;
            }
        }
        None => false,  
    }
}
