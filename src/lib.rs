pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use models::Post;
use std::env;

use self::models::NewPost;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post{
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    let result = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(conn);

    result.unwrap()
    
    /*diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    let results = posts::table
        .filter(posts::dsl::title.like(format!("%{}%", new_post.title)))
        .load::<Post>(conn)
        .expect("Error getting new post");

    for result in results {
        println!("{:?}", result);
    }
    */
        
}