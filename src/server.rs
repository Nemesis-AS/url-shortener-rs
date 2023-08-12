use rocket::{
    fs::{relative, FileServer, NamedFile},
    routes,
};

#[rocket::get("/")]
async fn home() -> Option<NamedFile> {
    NamedFile::open(relative!("views/index.html")).await.ok()
}

#[rocket::get("/auth")]
async fn auth() -> Option<NamedFile> {
    NamedFile::open(relative!("views/auth.html")).await.ok()
}

#[rocket::main]
pub async fn start() -> Result<(), rocket::Error> {
    match dotenvy::dotenv() {
        Ok(_) => println!("Loaded Config!"),
        Err(err) => println!("Couldn't Load Config Variables!\nError: {:?}", err),
    }

    let _rocket = rocket::build()
        .mount("/", routes![home, auth])
        .attach(crate::db::stage())
        .mount("/static", FileServer::from(relative!("/static")))
        .launch()
        .await?;

    Ok(())
}
