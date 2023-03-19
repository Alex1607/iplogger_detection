use std::str;

use base64::engine::general_purpose;
use base64::Engine;
use serde::{Deserialize, Serialize};
use worker::Method::Get;
use worker::*;

const IP_LOGGERS: [&str; 60] = [
    "over-blog.com",
    "gyazo.in",
    "ps3cfw.com",
    "urlz.fr",
    "webpanel.space",
    "steamcommumity.com",
    "imgur.com.de",
    "fuglekos.com",
    "discord.kim",
    "prntcrs.com",
    "grabify.link",
    "leancoding.co",
    "stopify.co",
    "freegiftcards.co",
    "joinmy.site",
    "curiouscat.club",
    "catsnthings.fun",
    "catsnthings.com",
    "xn--yutube-iqc.com",
    "gyazo.nl",
    "yip.su",
    "iplogger.com",
    "iplogger.co",
    "iplogger.org",
    "iplogger.ru",
    "iplogger.info",
    "ipgraber.ru",
    "ipgrabber.ru",
    "2no.co",
    "02ip.ru",
    "iplis.ru",
    "iplo.ru",
    "ezstat.ru",
    "whatstheirip.com",
    "partpicker.shop",
    "sportshub.bar",
    "locations.quest",
    "lovebird.guru",
    "trulove.guru",
    "dateing.club",
    "shrekis.life",
    "headshot.monster",
    "gaming-at-my.best",
    "progaming.monster",
    "yourmy.monster",
    "imageshare.best",
    "screenshot.best",
    "gamingfun.me",
    "catsnthing.com",
    "fortnitechat.site",
    "fortnight.space",
    "hondachat.com",
    "bvog.com",
    "youramonkey.com",
    "pronosparadise.com",
    "freebooter.pro",
    "blasze.com",
    "blasze.tk",
    "ipgrab.org",
    "gyazos.com",
];

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/", |_, _| Response::ok(""))
        .get_async("/:url", |_, ctx| async move {
            if let Some(base64_input) = ctx.param("url") {
                let mut redirect_urls = Vec::new();
                let mut request_init = RequestInit::new();
                request_init
                    .with_method(Get)
                    .with_redirect(RequestRedirect::Manual);

                let option_link = decode_base64(base64_input);

                if option_link.is_none() {
                    return build_response(
                        LoggerResponse {
                            response_type: ResponseType::BadRequest,
                            details: None,
                        },
                        400,
                    );
                }

                let link = option_link.unwrap();

								if !link.starts_with("https://") && !link.starts_with("http://") {
										return build_response(
												LoggerResponse {
														response_type: ResponseType::BadRequest,
														details: Some("Links have to be absolute and start with either a http or https".to_string()),
												},
												400,
										);
								}

                redirect_urls.push(link.to_string());

                let url = Url::parse(link.as_str())?;
                let request = Request::new_with_init(url.as_str(), &request_init)?;
                let mut response = Fetch::Request(request).send().await?;

                while response.status_code() == 301 || response.status_code() == 302 {
                    if let Some(location) = response.headers().get("location")? {
                        redirect_urls.push(location.to_string());

                        let new_request = Request::new_with_init(location.as_str(), &request_init)?;
                        response = Fetch::Request(new_request).send().await?;
                    } else {
                        break;
                    }
                }
                for redirect_url in redirect_urls {
                    if let Some(host) = Url::parse(redirect_url.as_str())?.host_str() {
                        if IP_LOGGERS.contains(&host) {
                            return build_response(
                                LoggerResponse {
                                    response_type: ResponseType::IpLoggerDetected,
                                    details: Some(host.to_string()),
                                },
                                200,
                            );
                        }
                    }
                }

                return build_response(
                    LoggerResponse {
                        response_type: ResponseType::Ok,
                        details: None,
                    },
                    200,
                );
            }

            build_response(
                LoggerResponse {
                    response_type: ResponseType::BadRequest,
                    details: None,
                },
                400,
            )
        })
        .run(req, env)
        .await
}

fn decode_base64(base64_input: &String) -> Option<String> {
    if let Ok(decoded) = general_purpose::URL_SAFE_NO_PAD.decode(base64_input) {
        if let Ok(link) = str::from_utf8(decoded.as_slice()) {
            return Some(link.to_string());
        }
    }
    None
}

fn build_headers(status_code: u16) -> Result<Headers> {
    let mut headers = Headers::new();
    if (200..=299).contains(&status_code) {
        headers.set("Content-Type", "application/json")?;
    }
    Ok(headers)
}

fn build_response(logger_response: LoggerResponse, status_code: u16) -> Result<Response> {
    Response::ok(serde_json::to_string(&logger_response).unwrap())?
        .with_headers(build_headers(status_code)?)
        .with_status(status_code)
        .with_cors(&Cors::new().with_origins(vec!["*"]))
}

#[derive(Debug, Deserialize, Serialize)]
struct LoggerResponse {
    #[serde(rename = "response")]
    response_type: ResponseType,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
enum ResponseType {
    #[serde(rename = "BAD_REQUEST")]
    BadRequest,
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "IPLOGGER_DETECTED")]
    IpLoggerDetected,
}
