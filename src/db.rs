use rocket::fairing::AdHoc;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};
use rocket::{routes, Build, Rocket};

use rocket_sync_db_pools::{database, rusqlite};

use self::rusqlite::params;

#[database("rusqlite")]
pub struct DB(rusqlite::Connection);

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct LinkData<'r> {
    link: &'r str,
}
#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
struct LinkRes {
    id: String,
    link: String,
}

#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
struct SimpleRes {
    message: String,
}

#[rocket::post("/shorten", data = "<form>")]
async fn shorten(db: DB, form: Json<LinkData<'_>>) -> Option<Json<LinkRes>> {
    let id: String = nanoid::nanoid!(6);
    let link: String = String::from(form.link);
    let res: LinkRes = LinkRes {
        id: id.clone(),
        link: link.clone(),
    };
    db.run(move |conn| {
        conn.execute(
            "INSERT INTO links(id, link) VALUES(?, ?)",
            params![id, link],
        )
    })
    .await
    .ok()?;

    Some(Json(res))
}

#[rocket::get("/<id>")]
async fn get_link(db: DB, id: String) -> Option<Redirect> {
    let res: Vec<String> = db
        .run(move |conn| {
            conn.prepare("SELECT * FROM links WHERE id = ?")?
                .query_map(params![id], |row| row.get(1))?
                .collect::<Result<Vec<String>, _>>()
        })
        .await
        .ok()?;

    if res.is_empty() {
        return Some(Redirect::to("/"));
    }
    Some(Redirect::to(res.get(0).unwrap().clone()))
}

#[rocket::get("/rem-exp")]
async fn remove_expired(db: DB) -> Option<Json<SimpleRes>> {
    db.run(|conn| {
        conn.execute(
            "DELETE FROM links WHERE expiry < CURRENT_TIMESTAMP",
            params![],
        )
    })
    .await
    .ok()?;

    let res: SimpleRes = SimpleRes {
        message: String::from("Removed Expired Links!"),
    };

    Some(Json(res))
}

async fn init_db(rocket: Rocket<Build>) -> Rocket<Build> {
    DB::get_one(&rocket)
        .await
        .expect("Database Mounted")
        .run(|conn| {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS links(id TEXT PRIMARY KEY, link TEXT NOT NULL, expiry DATETIME DEFAULT (DATETIME(CURRENT_TIMESTAMP, '+30 days')))",
                params![],
            )
        })
        .await
        .expect("Can Init DB");

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Rusqlite Stage", |rocket| async {
        rocket
            .attach(DB::fairing())
            .attach(AdHoc::on_ignite("Rusqlite Init", init_db))
            .mount("/", routes![shorten, remove_expired, get_link])
    })
}
