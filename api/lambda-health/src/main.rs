use lambda_http::{run, service_fn, Error, Request, Response};

async fn handler(_: Request) -> Result<Response<String>, Error> {
    Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("ok".to_string())
        .map_err(Error::from)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}
