//! Default Compute template program.

use fastly::{ConfigStore, Error, Request, Response};
use log;
use log_fastly;

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    log_fastly::init_simple("introduction_to_compute_rs", log::LevelFilter::Info);
    log::info!("Request: {}", req.get_url());
    log::info!(
        "User-Agent: {}",
        req.get_header("User-Agent")
            .and_then(|ua| ua.to_str().ok())
            .unwrap_or("not provided")
    );

    // Return robots.txt with a custom response
    if req.get_path().ends_with("/robots.txt") {
        let robots_txt = include_str!("robots.txt");
        let robots_response = Response::from_body(robots_txt)
            .with_status(200)
            .with_content_type(fastly::mime::TEXT_PLAIN);
        return Ok(robots_response);
    }

    // Check if there is a redirect for the URL requested.
    // If there is, redirect the client.
    let config = ConfigStore::open("redirects");
    let req_path = req.get_path();
    if let Some(dest) = config.get(req_path) {
        let redirect_response = Response::from_body("")
            .with_status(301)
            .with_header("Location", dest);
        return Ok(redirect_response);
    }

    // Forward all requests to the backend with cache override set to pass
    let mut resp = req.with_pass(true).send("vcl-origin")?;

    if resp.get_status() == 404 {
        // Load static HTML file for 404 page
        let not_found_html = include_str!("not-found.html");
        resp.set_body(not_found_html);
        resp.set_content_type(fastly::mime::TEXT_HTML_UTF_8);
        return Ok(resp);
    }

    // Add response header
    resp.set_header("x-tacos", "We love tacos!");
    Ok(resp)
}
