use http::StatusCode;
use std::error::Error;
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

use ::random::get_random_name;

mod names;

fn handler(_: Request) -> Result<impl IntoResponse, VercelError> {
    let data = get_random_name(names::get_names());

    match data {
        Ok(data) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Access-Control-Allow-Headers", "Content-Type")
                .header("Access-Control-Allow-Methods", "GET")
                .header("Access-Control-Allow-Origin", "*")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&data).unwrap())
                .expect("Internal Server Error");

            Ok(response)
        }
        Err(error) => Err(VercelError::new(&error)),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use vercel_lambda::Body;

    #[test]
    fn test_lambda_handler() {
        let result = handler(Request::new(Body::Empty));
        assert_eq!(result.is_err(), false);
    }
}
