
use super::prelude::*;
use maxwell_deamon_server::auth::LoginQuery;




pub async fn login(State(AppState { db: _db, .. }): State<AppState>,Query(_query): Query<LoginQuery>)-> impl IntoResponse {
}





