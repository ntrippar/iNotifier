extern crate mac_notification_sys;
extern crate iron;
extern crate bodyparser;
extern crate router;
#[macro_use]
extern crate serde_derive;

use mac_notification_sys::*;
use iron::prelude::*;
use iron::status;
use router::Router;

#[derive(Debug, Clone, Deserialize)]
struct Notification {
    application: String,
    title: String,
    subtitle: String,
    body: String,
}

fn notify(req: &mut Request) -> IronResult<Response> {

	let struct_body = req.get::<bodyparser::Struct<Notification>>();
    match struct_body {
        Ok(Some(notification)) => {
        	send_notification(format!("[{}] {}", notification.application, notification.title).as_str(),
                      &Some(notification.subtitle.as_str()),
                      notification.body.as_str(),
                      &None).unwrap();
        },
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

	Ok(Response::with(status::Ok))
}

fn main() {
    let mut router = Router::new();
    router.post("/notify", notify, "notify");

    Iron::new(router).http("127.0.0.1:31337").unwrap();
}


