use crate::errors::EzyTutorError;
use crate::models::tutor::*;
use sqlx::postgres::PgPool;

pub async fn get_all_tutors_db(pool: &PgPool) -> Result<Vec<Tutor>, EzyTutorError> {
    // Prepare SQL statement
    let tutor_rows = sqlx::query_as!(Tutor, "SELECT * FROM ezy_tutor_c4")
        .fetch_all(pool)
        .await?;

    match tutor_rows.len() {
        0 => Err(EzyTutorError::NotFound("No tutors found".into())),
        _ => Ok(tutor_rows),
    }
}

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, EzyTutorError> {
    // Prepare SQL statement
    let tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url,tutor_profile FROM ezy_tutor_c4 where tutor_id = $1",tutor_id
    )
    .fetch_one(pool)
    .await
    .map(|tutor_row| Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;
    Ok(tutor_row)
}

pub async fn post_new_tutor_db(pool: &PgPool, new_tutor: NewTutor) -> Result<Tutor, EzyTutorError> {
    let tutor_row = sqlx::query!(
        "insert into ezy_tutor_c4(tutor_name, tutor_pic_url, tutor_profile) values ($1,$2,$3) returning tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        new_tutor.tutor_name,
        new_tutor.tutor_pic_url,
        new_tutor.tutor_profile
    )
    .fetch_one(pool)
    .await?;
    //Retrieve result
    Ok(Tutor {
        tutor_id: tutor_row.tutor_id,
        tutor_name: tutor_row.tutor_name,
        tutor_pic_url: tutor_row.tutor_pic_url,
        tutor_profile: tutor_row.tutor_profile,
    })
}

pub async fn update_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
    update_tutor: UpdateTutor,
) -> Result<Tutor, EzyTutorError> {
    let current_tutor_row = sqlx::query_as!(
        Tutor,
        "SELECT * FROM ezy_tutor_c4 where tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound("tutor id not found".into()))?;
    let name: String = if let Some(name) = update_tutor.tutor_name {
        name
    } else {
        current_tutor_row.tutor_name
    };
    let tutor_pic_url: String = if let Some(pic_url) = update_tutor.tutor_pic_url {
        pic_url
    } else {
        current_tutor_row.tutor_pic_url
    };
    let tutor_profile: String = if let Some(profile) = update_tutor.tutor_profile {
        profile
    } else {
        current_tutor_row.tutor_profile
    };
    let tutor_row = sqlx::query_as!(
        Tutor,
        "UPDATE ezy_tutor_c4 set tutor_name=$1,tutor_pic_url=$2,tutor_profile=$3,tutor_id=$4 returning tutor_id,tutor_name,tutor_pic_url,tutor_profile",
        name,
        tutor_pic_url,
        tutor_profile,
        tutor_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(tutor) = tutor_row {
        Ok(tutor)
    } else {
        Err(EzyTutorError::NotFound("tutor id not found".into()))
    }
}
pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<String, EzyTutorError> {
    let tutor_row = sqlx::query!("DELETE FROM ezy_tutor_c4 where tutor_id = $1", tutor_id,)
        .execute(pool)
        .await?;
    Ok(format!("Deleted {:#?} record", tutor_row))
}
