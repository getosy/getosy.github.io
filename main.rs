use rocket::fs::{FileServer, relative};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/assets", FileServer::from(relative!("assets"))) // Serve assets
}
#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Osy Client - High Performance Exploit Tool",
        header_title: "Welcome to Osy Client",
        header_subtitle: "Your high-performance exploit executor for seamless functionality",
        download_url: "https://link-target.net/854704/sentinel-remake-bootstrap",
        community_url: "https://dsc.gg/getosy",
        features: [
            { "title": "Reliable UNC", "description": "Achieve 80% UNC with exceptional stability for all operations." },
            { "title": "Sleek User Interface", "description": "Navigate seamlessly with a professional, intuitive UI." },
            { "title": "Powered by Avalon Editor", "description": "Experience smooth, highlighted coding with Avalon text editor." }
        ],
        screenshot_url: "/assets/GUI.png"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/assets", FileServer::from(relative!("assets")))
}