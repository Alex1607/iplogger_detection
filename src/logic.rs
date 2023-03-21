use worker::{Fetch, Request, RequestInit, RequestRedirect, Response, RouteContext, Url};
use worker::Method::Get;
use crate::ip_loggers::IP_LOGGERS;
use crate::response::{LoggerResponse, ResponseType};

pub async fn check_for_iplogger(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let Some(base64_input) = ctx.param("url") else {
				return crate::response::build_response(
						LoggerResponse {
								response_type: ResponseType::BadRequest,
								details: None,
						},
						400,
				);
		};

    let mut redirect_urls = Vec::new();
    let mut request_init = RequestInit::new();
    request_init
        .with_method(Get)
        .with_redirect(RequestRedirect::Manual);

    let option_link = crate::decode_base64(base64_input);

    if option_link.is_none() {
        return crate::response::build_response(
            LoggerResponse {
                response_type: ResponseType::BadRequest,
                details: None,
            },
            400,
        );
    }

    let link = option_link.unwrap();

    if !link.starts_with("https://") && !link.starts_with("http://") {
        return crate::response::build_response(
            LoggerResponse {
                response_type: ResponseType::BadRequest,
                details: Some(
                    "Links have to be absolute and start with either a http or https".to_string(),
                ),
            },
            400,
        );
    }

    redirect_urls.push(link.to_string());

    let url = Url::parse(link.as_str())?;
    let request = Request::new_with_init(url.as_str(), &request_init)?;
    let mut response = Fetch::Request(request).send().await?;

    while response.status_code() == 301 || response.status_code() == 302 {
        let Some(location) = response.headers().get("location")? else {
						break;
				};

        redirect_urls.push(location.to_string());

        let new_request = Request::new_with_init(location.as_str(), &request_init)?;
        response = Fetch::Request(new_request).send().await?;
    }
    for redirect_url in redirect_urls {
				let url = Url::parse(redirect_url.as_str())?;

        let Some(host) = url.host_str() else {
						continue;
				};

        if IP_LOGGERS.contains(&host) {
            return crate::response::build_response(
                LoggerResponse {
                    response_type: ResponseType::IpLoggerDetected,
                    details: Some(host.to_string()),
                },
                200,
            );
        }
    }

    crate::response::build_response(
        LoggerResponse {
            response_type: ResponseType::Ok,
            details: None,
        },
        200,
    )
}
