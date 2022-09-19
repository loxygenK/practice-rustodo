#[derive(Debug)]
pub struct APIError(anyhow::Error);

impl warp::reject::Reject for APIError {}

pub fn reject_anyhow(error: anyhow::Error) -> warp::Rejection {
    warp::reject::custom(APIError(error))
}
