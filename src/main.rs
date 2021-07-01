use cmd_lib::run_fun;
use netlify_lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda(http)]
#[tokio::main]
async fn main(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let v = run_fun!(rustc --version)?;
    Ok(format!("🦀 Hello, Netlify 🦀 Rust version: {}", v))
}
