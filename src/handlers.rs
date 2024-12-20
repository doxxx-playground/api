use crate::db::DbPool;
use crate::models::{Item, NewItem};
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("API is running")
}

pub async fn get_items(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::items::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    match items.load::<Item>(&mut conn) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_item_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    use crate::schema::items::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let item_id = path.into_inner();

    match items.find(item_id).first::<Item>(&mut conn) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(diesel::NotFound) => HttpResponse::NotFound().body("Item not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_item(pool: web::Data<DbPool>, new_item: web::Json<NewItem>) -> impl Responder {
    use crate::schema::items;

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    match diesel::insert_into(items::table)
        .values(&new_item.0)
        .get_result::<Item>(&mut conn)
    {
        Ok(result) => HttpResponse::Created().json(result),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_item(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    updated_item: web::Json<NewItem>,
) -> impl Responder {
    use crate::schema::items::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let item_id = path.into_inner();

    match diesel::update(items.find(item_id))
        .set((
            name.eq(&updated_item.name),
            description.eq(&updated_item.description),
        ))
        .get_result::<Item>(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().json("Item updated successfully"),
        Err(diesel::NotFound) => HttpResponse::NotFound().body("Item not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_item(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    use crate::schema::items::dsl::*;

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let item_id = path.into_inner();

    match diesel::delete(items.find(item_id)).execute(&mut conn) {
        Ok(1) => HttpResponse::NoContent().finish(),
        Ok(0) => HttpResponse::NotFound().body("Item not found"),
        Ok(_) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
