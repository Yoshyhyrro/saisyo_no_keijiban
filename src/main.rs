use rocket::{State, Shutdown};
use rocket::fs::FileServer;
use rocket::form::Form;
use serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::Request;
use rocket::fs::relative;


#[macro_use] extern crate rocket;

/*
#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}


#[post("/sinen", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // A send 'fails' if there are no active subscribers. That's okay.
    let _res = queue.send(form.into_inner());
}
*/
#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}
/*#[post("/", data = "<input>")]
fn sinen(input: Form<Input>) {
     /* .. */
 }
 */
#[launch]
fn rocket() -> _ {
    rocket::build()
    //.manage(channel::<Message>(1024).0)
    //.mount("/sinen", routes![sinen])
    .mount("/", FileServer::from("static"))
    //.mount("/500","/404",catchers![internal_error, not_found])
    //.launch();
}