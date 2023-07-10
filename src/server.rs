use rocket::{
    fs::{relative, FileServer, NamedFile},
    routes,
};

#[rocket::get("/")]
async fn home() -> Option<NamedFile> {
    NamedFile::open(relative!("views/index.html")).await.ok()
}

#[rocket::main]
pub async fn start() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![home])
        .attach(crate::db::stage())
        .mount("/static", FileServer::from(relative!("/static")))
        .launch()
        .await?;

    Ok(())
}
