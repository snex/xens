#[macro_use] extern crate rocket;
use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::response::Redirect;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::{Template, context};

extern crate redis;

extern crate rand;
use rand::{distributions::Alphanumeric, Rng};

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("index.html").await.ok()
}

fn generate_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

fn add_key_to_redis(new_key: String, url: String) -> Option<()> {
    let new_key = format!("xens:{}", new_key);
    let client = redis::Client::open("redis://127.0.0.1/").ok()?;
    let mut con = client.get_connection().ok()?;
    redis::pipe()
        .atomic()
        .cmd("SET").arg(new_key.clone()).arg(url).ignore()
        .cmd("EXPIRE").arg(new_key.clone()).arg(60 * 60 * 24 * 7)
        .query(&mut con).ok()?
}

#[derive(FromForm)]
struct UrlForm {
    url: String
}

#[post("/new", data = "<data>")]
fn new(data: Form<UrlForm>) -> Template {
    let new_key = generate_key();
    add_key_to_redis(new_key.clone(), data.url.clone());

    Template::render("new", context! { new_url: new_key.clone() })
}

fn get_key_from_redis(key: String) -> Option<String> {
    let key = format!("xens:{}", key);
    let client = redis::Client::open("redis://127.0.0.1/").ok()?;
    let mut con = client.get_connection().ok()?;
    redis::cmd("GET").arg(key.clone()).query(&mut con).ok()?
}

#[get("/<key>")]
fn redir(key: String) -> Redirect {
    match get_key_from_redis(key) {
      Some(url) => Redirect::to(format!("{}", url)),
      None      => Redirect::to("/")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![new])
        .mount("/", routes![redir])
        .attach(Template::fairing())
}
