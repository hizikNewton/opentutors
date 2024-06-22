use actix_web::{web, HttpResponse};
use crate::db_access::*;
use crate::errors::EzyTutorError;
use crate::state::*;

pub async fn get_all_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, EzyTutorError> {
    get_all_tutors_db(&app_state.db)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}
