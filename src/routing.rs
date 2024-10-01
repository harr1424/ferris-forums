use crate::api::comment::*;
use crate::api::post::*;
use crate::api::sub::*;
use crate::api::user::*;
use actix_web::web::ServiceConfig;

pub fn configure_user_routes(cfg: &mut ServiceConfig) {
    cfg.service(create_user)
        .service(get_user_by_id)
        .service(get_user_by_username)
        .service(get_users_by_sub)
        .service(verify_user_password)
        .service(username_exists)
        .service(grant_mod_status)
        .service(remove_mod_status)
        .service(update_user_password)
        .service(delete_user);
}

pub fn configure_sub_routes(cfg: &mut ServiceConfig) {
    cfg.service(create_sub)
        .service(get_all_subs)
        .service(get_subs_by_user_id)
        .service(get_sub_by_name)
        .service(update_sub)
        .service(delete_sub)
        .service(subscribe_user_to_sub);
}

pub fn configure_comment_routes(cfg: &mut ServiceConfig) {
    cfg.service(create_comment)
        .service(get_comments)
        .service(update_comment)
        .service(delete_comment);
}

pub fn configure_post_routes(cfg: &mut ServiceConfig) {
    cfg.service(create_post)
        .service(get_post)
        .service(get_posts_by_sub)
        .service(update_post)
        .service(delete_post);
}
