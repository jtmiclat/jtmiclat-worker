use worker::*;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();
    let version = env.var("WORKERS_RS_VERSION")?.to_string();
    let response = worker::Fetch::Request(req).send().await?;

    let mut headers = response.headers().clone();
    headers.set(
        "strict-transport-security",
        "max-age=31536000; includeSubDomains",
    )?;
    headers.set("x-frame-options", "SAMEORIGIN")?;
    headers.set("x-content-type-options", "nosniff")?;
    headers.set("referrer-policy", "no-referrer")?;
    headers.set("permissions-policy", "microphone 'none'")?;
    headers.set(
        "content-security-policy",
        "default-src 'self' 'unsafe-inline' *.google-analytics.com  cloudflareinsights.com *.cloudflareinsights.com www.googletagmanager.com avatars.githubusercontent.com",
    )?;
    headers.set("cf-worker-version", &version)?;
    Ok(response.with_headers(headers))
}
