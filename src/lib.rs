use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _context: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("owo"))
        .run(req, env)
        .await
}
