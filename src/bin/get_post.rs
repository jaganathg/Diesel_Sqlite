use diesel::prelude::*;
use diesel_dmo::*;
use diesel_dmo::models::Post;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::posts;

    let post_id = args()
        .nth(1)
        .expect("get_post requires a post_id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();

    match post {
        Ok(Some(post)) => println!("Post with id: {} and title: {}", post.id, post.title),
        Ok(None) => println!("Post not found with id:{}", post_id),
        Err(_) => println!("An error occurred while fetching post with id:{}", post_id),
    }
}