extern crate diesel;
extern crate diesel_demo;

fn main() {
    let conn = diesel_demo::establish_connection();

    // insert
    let post_id = diesel_demo::insert_posts(&conn, "llj");
    // query
    let post = diesel_demo::query_posts_by_id(&conn, &post_id);
    
    // 打印
    println!("{:?}", post);
}
