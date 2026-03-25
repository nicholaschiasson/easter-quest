use std::{fs, io, time::Duration};

use askama::Template;
use axum::{
    extract::{OriginalUri, Path, Query, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{Html, IntoResponse, Redirect},
    routing::{get, get_service},
    Form, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgConnectOptions, FromRow, PgPool};
use tokio::try_join;
use tower_http::{
    services::{ServeDir, ServeFile},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use uuid::Uuid;

const COOKIE_USER_ID: &str = "user_id";
const RESPONSE_FORBIDDEN: &str =
    "Looks like you skipped an egg... Don't rush things, mate. Where is the joy in that?";
const RESPONSE_NOT_FOUND: &str = "You lookin for something mate?";
const RESPONSE_NOT_INVITED: &str = "Sorry bro, you weren't invited...";
const RESPONSE_UNAUTHORIZED: &str = "Who the fuck are you...?";

const ENV_NAME_HOST: &str = "HOST";
const ENV_NAME_PORT: &str = "PORT";
const ENV_NAME_INVITE_CODE: &str = "INVITE_CODE";

#[derive(Template)]
#[template(path = "error.html", escape = "none")]
struct ErrorResponse {
    status: StatusCode,
    reason: Option<String>,
}

impl ErrorResponse {
    fn new(status: StatusCode, message: Option<&str>) -> Self {
        Self {
            status,
            reason: message.map(str::to_string),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let reason_string = match &self.reason {
            Some(reason) => format!("<p>{reason}</p>"),
            None => String::new(),
        };
        match self.render() {
            Ok(rendered) => (self.status, Html(rendered)).into_response(),
            _ => (
                self.status,
                Html(format!(
                    "<!doctype html><html><body><h1>{}</h1>{}</body></html>",
                    self.status.as_u16(),
                    reason_string
                )),
            )
                .into_response(),
        }
    }
}

#[derive(Serialize, FromRow)]
struct Egg {
    pub id: Uuid,
    pub title: String,
}

#[derive(Serialize, FromRow, Template)]
#[template(path = "index.html", escape = "none")]
struct Index {
    pub user: User,
    pub eggs: Vec<Egg>,
    pub first_egg: Egg,
}

async fn get_index(
    jar: CookieJar,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Some(user_id) = jar.get(COOKIE_USER_ID) {
        let user_id: Uuid = user_id.value().parse().unwrap_or_default();
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&state.pool);
        let eggs = sqlx::query_as::<_, Egg>(
            "
                SELECT *
                FROM user_eggs
                INNER JOIN eggs ON user_eggs.egg_id = eggs.id
                WHERE user_id = $1
                ORDER BY idx ASC
            ",
        )
        .bind(user_id)
        .fetch_all(&state.pool);
        let first_egg =
            sqlx::query_as::<_, Egg>("SELECT * FROM eggs WHERE idx = 1").fetch_one(&state.pool);
        let index = try_join!(user, eggs, first_egg)
            .map_err(|e| e.to_string())
            .and_then(|(user, eggs, first_egg)| {
                Index {
                    user,
                    eggs,
                    first_egg,
                }
                .render()
                .map_err(|e| e.to_string())
            });
        match index {
            Ok(index) => Ok((StatusCode::OK, Html(index)).into_response()),
            Err(e) => Err(ErrorResponse::new(StatusCode::BAD_REQUEST, Some(&e))),
        }
    } else {
        Ok(Redirect::to("/login").into_response())
    }
}

#[derive(Serialize, FromRow)]
struct EggRecord {
    pub id: Uuid,
    pub title: String,
    pub content_uri: String,
    pub next: Option<Uuid>,
    pub previous: Option<Uuid>,
}

#[derive(Template)]
#[template(path = "egg.html", escape = "none")]
struct EggPage {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub next: Option<Uuid>,
    pub previous: Option<Uuid>,
}

impl TryFrom<EggRecord> for EggPage {
    type Error = io::Error;

    fn try_from(
        EggRecord {
            id,
            title,
            content_uri,
            next,
            previous,
        }: EggRecord,
    ) -> Result<Self, Self::Error> {
        let content = fs::read_to_string(format!("rsrc/private/{content_uri}"))?;
        Ok(Self {
            id,
            title,
            content,
            next,
            previous,
        })
    }
}

async fn get_egg(
    jar: CookieJar,
    Path(id): Path<Uuid>,
    State(state): State<MyState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let user_id: Option<Uuid> = jar
        .get(COOKIE_USER_ID)
        .map(|id| id.value().parse().unwrap_or_default());

    let egg = sqlx::query_as::<_, EggRecord>(
        r#"
        SELECT *
        FROM (
            SELECT
                *,
                LAG(id) OVER (ORDER BY idx ASC) AS previous,
                LEAD(id) OVER (ORDER BY idx ASC) AS next
            FROM eggs
        )
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())
    .and_then(|egg| EggPage::try_from(egg).map_err(|e| e.to_string()));

    match (user_id, egg) {
        (Some(user_id), Ok(mut egg)) => {
            if let Some(next_id) = egg.next {
                if let Ok(false) = sqlx::query_scalar::<_, i64>(
                    "SELECT count(*) FROM user_eggs WHERE user_id = $1 AND egg_id = $2",
                )
                .bind(user_id)
                .bind(next_id)
                .fetch_one(&state.pool)
                .await
                .map(|count| count > 0)
                {
                    egg.next = None;
                }
            }

            let rendered = egg.render();

            if let (Ok(true), Ok(rendered)) = (
                sqlx::query_scalar::<_, i64>(
                    "SELECT count(*) FROM user_eggs WHERE user_id = $1 AND egg_id = $2",
                )
                .bind(user_id)
                .bind(egg.id)
                .fetch_one(&state.pool)
                .await
                .map(|count| count > 0),
                rendered.as_deref(),
            ) {
                return Ok((StatusCode::OK, Html(rendered.to_string())));
            }

            let has_previous = if let Some(previous_id) = egg.previous {
                sqlx::query_scalar::<_, i64>(
                    "SELECT count(*) FROM user_eggs WHERE user_id = $1 AND egg_id = $2",
                )
                .bind(user_id)
                .bind(previous_id)
                .fetch_one(&state.pool)
                .await
                .map(|count| count > 0)
            } else {
                Ok(true)
            };

            match (has_previous, rendered.as_deref()) {
                (Ok(true), Ok(rendered)) => {
                    let _ = sqlx::query_scalar::<_, i64>(
                        "INSERT INTO user_eggs (user_id, egg_id) VALUES ($1, $2) RETURNING 1",
                    )
                    .bind(user_id)
                    .bind(egg.id)
                    .fetch_optional(&state.pool)
                    .await;
                    Ok((StatusCode::OK, Html(rendered.to_string())))
                }
                (Ok(false), _) => Err(ErrorResponse::new(
                    StatusCode::FORBIDDEN,
                    Some(RESPONSE_FORBIDDEN),
                )),
                (Err(e), _) => Err(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(&e.to_string()),
                )),
                (_, Err(e)) => Err(ErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Some(&e.to_string()),
                )),
            }
        }
        (None, _) => Err(ErrorResponse::new(
            StatusCode::UNAUTHORIZED,
            Some(RESPONSE_UNAUTHORIZED),
        )),
        (_, Err(e)) => Err(ErrorResponse::new(StatusCode::BAD_REQUEST, Some(&e))),
    }
}

#[derive(Debug, Deserialize)]
struct CreateSessionQuery {
    referer: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateSessionPayload {
    email: String,
}

async fn create_session(
    jar: CookieJar,
    State(state): State<MyState>,
    Query(query): Query<CreateSessionQuery>,
    Form(form): Form<CreateSessionPayload>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(form.email)
        .fetch_one(&state.pool)
        .await
    {
        Ok(user) => Ok((
            jar.add(Cookie::new(COOKIE_USER_ID, user.id.to_string())),
            Redirect::to(&query.referer.unwrap_or("/".to_string())),
        )),
        Err(e) => Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            Some(&e.to_string()),
        )),
    }
}

#[derive(Debug, Deserialize)]
struct CreateUserQuery {
    invite: Option<String>,
    referer: Option<String>,
}

#[derive(Deserialize)]
struct CreateUserPayload {
    pub email: String,
    pub name: String,
}

#[derive(Serialize, FromRow)]
struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}

async fn create_user(
    jar: CookieJar,
    State(state): State<MyState>,
    Query(query): Query<CreateUserQuery>,
    Form(data): Form<CreateUserPayload>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match query.invite {
        Some(invite) if invite == state.invite_code => (),
        _ => {
            return Err(ErrorResponse::new(
                StatusCode::FORBIDDEN,
                Some(RESPONSE_NOT_INVITED),
            ));
        }
    }
    match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO users (email, name) VALUES ($1, $2) RETURNING id",
    )
    .bind(&data.email)
    .bind(&data.name)
    .fetch_one(&state.pool)
    .await
    {
        Ok(user_id) => Ok((
            jar.add(Cookie::new(COOKIE_USER_ID, user_id.to_string())),
            Redirect::to(&query.referer.unwrap_or("/".to_string())),
        )),
        Err(e) => Err(ErrorResponse::new(
            StatusCode::BAD_REQUEST,
            Some(&e.to_string()),
        )),
    }
}

#[derive(Debug, Deserialize)]
struct Invite {
    invite: Option<String>,
}

async fn validate_invite(
    State(state): State<MyState>,
    jar: CookieJar,
    uri: OriginalUri,
    Query(Invite { invite }): Query<Invite>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    match (jar.get(COOKIE_USER_ID), uri.path(), invite) {
        (None, "/", Some(invite)) if invite == state.invite_code => {
            Redirect::to(&format!("/signup?referer={}&invite={}", uri.path(), invite))
                .into_response()
        }
        _ => next.run(request).await,
    }
}

async fn validate_session(
    State(state): State<MyState>,
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Some(user_id) = jar.get(COOKIE_USER_ID) {
        match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id.value().parse::<Uuid>().unwrap_or_default())
            .fetch_one(&state.pool)
            .await
        {
            Ok(_) => Ok(next.run(request).await),
            Err(sqlx::Error::RowNotFound) => Err(ErrorResponse::new(
                StatusCode::UNAUTHORIZED,
                Some(RESPONSE_UNAUTHORIZED),
            )),
            Err(e) => Err(ErrorResponse::new(
                StatusCode::BAD_REQUEST,
                Some(&e.to_string()),
            )),
        }
    } else {
        Ok(Redirect::to(&format!("/login?referer={}", request.uri())).into_response())
    }
}

async fn not_found() -> impl IntoResponse {
    ErrorResponse::new(StatusCode::NOT_FOUND, Some(RESPONSE_NOT_FOUND))
}

#[derive(Clone)]
struct MyState {
    pool: PgPool,
    invite_code: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool_connect_options = PgConnectOptions::new();

    let pool: PgPool = sqlx::PgPool::connect_with(pool_connect_options)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let invite_code = std::env::var(ENV_NAME_INVITE_CODE)
        .unwrap_or_else(|_| panic!("{ENV_NAME_INVITE_CODE} environment variable must be defined"));
    let state = MyState { pool, invite_code };
    let app = Router::new()
        .route("/", get(get_index))
        .route("/eggs/{id}", get(get_egg))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            validate_session,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            validate_invite,
        ))
        .route(
            "/signup",
            get_service(ServeFile::new("rsrc/private/signup.html")).post(create_user),
        )
        .route(
            "/login",
            get_service(ServeFile::new("rsrc/private/login.html")).post(create_session),
        )
        .nest_service("/favicon.ico", ServeFile::new("rsrc/public/favicon.svg"))
        .nest_service("/favicon.svg", ServeFile::new("rsrc/public/favicon.svg"))
        .nest_service(
            "/static",
            ServeDir::new("rsrc/public")
                .not_found_service(ServeFile::new("rsrc/private/notfound.html")),
        )
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .with_state(state);

    let listen_host = std::env::var(ENV_NAME_HOST)
        .unwrap_or_else(|_| panic!("{ENV_NAME_HOST} environment variable must be defined"));
    let listen_port = std::env::var(ENV_NAME_PORT)
        .unwrap_or_else(|_| panic!("{ENV_NAME_PORT} environment variable must be defined"));
    let listen_addr = format!("{listen_host}:{listen_port}");
    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();

    log::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
