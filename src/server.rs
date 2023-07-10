#[rocket::main]
pub async fn start() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(crate::db::stage())
        .mount(
            "/static",
            rocket::fs::FileServer::from(rocket::fs::relative!("/static")),
        )
        .launch()
        .await?;

    Ok(())
}
