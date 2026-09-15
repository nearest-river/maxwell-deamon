
use super::prelude::*;




#[derive(Template)]
#[template(path="index.html")]
struct Home;


pub async fn home()-> impl IntoResponse {
  Html(Home.render().unwrap())
}








