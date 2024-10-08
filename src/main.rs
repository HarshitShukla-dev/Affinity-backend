use axum::{
    http::{self, Method},
    routing::{get, post, put},
    Extension, Router,
};
use handlers::{
    auth_handlers::{
        login_handler, new_password_handler, otp_handler, send_pass_reset_handler, signup_handler,
    },
    cp_handler::code_handler,
    crud_handlers::{
        add_friend_handler, change_flag_handler, create_matched_handler, get_accepted_boys_handler,
        get_all_users_handler, get_boys_handler, get_girl_request_handler, get_girls_handler,
        get_matched_handler, get_user_by_id_handler, get_user_handler, reject_handler,
        update_contest_score_handler, update_score_handler, update_user_character_handler,
    },
};
use sea_orm::Database;
use tower_http::cors::{AllowOrigin, CorsLayer};
mod bcrypts;
mod configs;
mod handlers;
mod model;
mod utils;

#[tokio::main]
async fn main() {
    let db_string = (*utils::constants::DATABASE_URL).clone();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(AllowOrigin::exact(
            "http://ec2-13-232-176-18.ap-south-1.compute.amazonaws.com:5173"
                .parse()
                .unwrap(),
        ))
        .allow_origin(AllowOrigin::exact(
            "http://ec2-13-232-176-18.ap-south-1.compute.amazonaws.com"
                .parse()
                .unwrap(),
        ))
        .allow_origin(AllowOrigin::exact(
            "http://ec2-13-126-149-80.ap-south-1.compute.amazonaws.com:5173"
                .parse()
                .unwrap(),
        ))
        .allow_origin(AllowOrigin::exact("http://localhost:5173".parse().unwrap()))
        .allow_headers([
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::ORIGIN,
            http::header::SET_COOKIE,
        ])
        .allow_credentials(true);

    let db = Database::connect(db_string)
        .await
        .expect("could not connect");
    let app: Router<()> = Router::new()
        .route("/sendpassreset", get(send_pass_reset_handler))
        .route("/newpassword", get(new_password_handler))
        .route("/otp", get(otp_handler))
        .route("/login", post(login_handler))
        // .route("/decode", get(decode_jwt))
        .route("/signup", post(signup_handler))
        // .route("/runcode", post(code_handler))
        .route("/getuser", post(get_user_handler))
        .route("/getboys", get(get_boys_handler))
        .route("/getgirls", get(get_girls_handler))
        .route("/updatescore", put(update_score_handler))
        .route("/getallusers", get(get_all_users_handler))
        .route("/addfriend", post(add_friend_handler))
        .route("/updatecharacter", put(update_user_character_handler))
        .route("/getgirlrequests", post(get_girl_request_handler))
        .route("/getacceptedboys", post(get_accepted_boys_handler))
        .route("/changeflag", post(change_flag_handler))
        .route("/creatematch", post(create_matched_handler))
        .route("/getmatched", post(get_matched_handler))
        .route("/updatecontestscore", put(update_contest_score_handler))
        .route("/getuserbyid", post(get_user_by_id_handler))
        .route("/reject", post(reject_handler))
        .layer(cors)
        .layer(Extension(db));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Listening");

    axum::serve(listner, app).await.unwrap();
}
