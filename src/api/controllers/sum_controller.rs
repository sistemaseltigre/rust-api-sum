use axum::{extract::Path, Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct SumResponse {
    result: i32,
}

pub async fn sum(Path(number): Path<i32>) -> Result<(StatusCode, Json<SumResponse>), StatusCode> {
    let result = number + 1;
    Ok((StatusCode::OK, Json(SumResponse { result }))) 
}