use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use rocket::fairing::AdHoc;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize};
use rocket::{routes, Build, Rocket};
use sha2::Sha384;
use std::collections::BTreeMap;

use rocket_sync_db_pools::{database, rusqlite};

use self::rusqlite::params;

const TEST_SECRET: &[u8; 10] = b"HelloWorld";

// $Env:SQLITE3_LIB_DIR = "D:\Source\sqlite\v3420000\lib"

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

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct UserInfo<'r> {
    username: &'r str,
    password: &'r str,
}

#[derive(Debug, rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
struct UserRes {
    username: String,
    token: String,
    status: i32,
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

// Auth

#[rocket::post("/register", data = "<form>")]
async fn register(db: DB, form: Json<UserInfo<'_>>) -> Option<Json<UserRes>> {
    let mut uname: String = String::from(form.username);
    let hashed_password: String = bcrypt::hash(form.password, bcrypt::DEFAULT_COST).unwrap();

    let taken = db
        .run(move |conn| {
            conn.prepare("SELECT * FROM users WHERE username = ?")?
                .exists(params![&uname])
        })
        .await
        .ok()?;

    if taken {
        let res: UserRes = UserRes {
            username: String::new(),
            token: String::new(),
            status: 400,
            message: String::from("User already exists!"),
        };

        return Some(Json(res));
    }

    uname = String::from(form.username);
    db.run(move |conn| {
        conn.execute(
            "INSERT INTO users(username, password) VALUES(?, ?)",
            params![uname, hashed_password],
        )
    })
    .await
    .ok()?;

    let token: Token<Header, BTreeMap<&str, &str>, jwt::token::Signed> =
        generate_token(form.username)?;

    let res: UserRes = UserRes {
        username: String::from(form.username),
        token: token.as_str().to_owned(),
        status: 200,
        message: String::from("Login Successfull"),
    };

    Some(Json(res))
}

#[rocket::post("/login", data = "<form>")]
async fn login(db: DB, form: Json<UserInfo<'_>>) -> Option<Json<UserRes>> {
    let uname: String = String::from(form.username);
    let pwd: Vec<String> = db
        .run(move |conn| {
            conn.prepare("SELECT * FROM users WHERE username = ?")?
                .query_map(params![uname], |row| row.get(1))?
                .collect::<Result<Vec<String>, _>>()
        })
        .await
        .ok()?;

    let mut res: UserRes = UserRes {
        username: String::new(),
        token: String::new(),
        status: 0,
        message: String::new(),
    };

    if pwd.is_empty() {
        res.status = 401;
        res.message = String::from("User not found!");
        return Some(Json(res));
    }

    let hashed_pwd: &str = pwd[0].as_str();

    if bcrypt::verify(form.password, hashed_pwd).unwrap() {
        let token: Token<Header, BTreeMap<&str, &str>, jwt::token::Signed> =
            generate_token(form.username)?;

        res.username = String::from(form.username);
        res.token = token.as_str().to_owned();
        res.status = 200;
        res.message = String::from("Login Successful!");

        Some(Json(res))
    } else {
        res.status = 401;
        res.message = String::from("Incorrect Password!");

        Some(Json(res))
    }
}

// Utils

fn generate_token(
    username: &str,
) -> Option<Token<Header, BTreeMap<&str, &str>, jwt::token::Signed>> {
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let mut claims = BTreeMap::new();
    claims.insert("username", username);

    let key: Hmac<Sha384> = Hmac::new_from_slice(TEST_SECRET).ok()?;
    let token: Token<Header, BTreeMap<&str, &str>, jwt::token::Signed> =
        Token::new(header, claims).sign_with_key(&key).ok()?;

    Some(token)
}

async fn init_db(rocket: Rocket<Build>) -> Rocket<Build> {
    let connection = DB::get_one(&rocket).await.expect("Database Mounted");
    connection.run(|conn| {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS links(id TEXT PRIMARY KEY, link TEXT NOT NULL, expiry DATETIME DEFAULT (DATETIME(CURRENT_TIMESTAMP, '+30 days')))",
                params![],
            )
        })
        .await
        .expect("Can Init DB");
    connection.run(|conn| {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS users(username TEXT PRIMARY KEY, password TEXT NOT NULL, admin INTEGER DEFAULT FALSE)",
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
            .mount(
                "/",
                routes![shorten, remove_expired, register, login, get_link],
            )
    })
}
