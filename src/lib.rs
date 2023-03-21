use std::str;

use base64::engine::general_purpose;
use base64::Engine;
use worker::*;

mod ip_loggers;
mod logic;
mod response;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok(""))
        .get_async("/:url", logic::check_for_iplogger)
        .run(req, env)
        .await
}

fn decode_base64(base64_input: &String) -> Option<String> {
    let Ok(decoded) = general_purpose::URL_SAFE_NO_PAD.decode(base64_input) else {
        return None;
    };

		let Ok(link) = str::from_utf8(decoded.as_slice()) else {
				return None;
		};

		Some(link.to_string())
}
