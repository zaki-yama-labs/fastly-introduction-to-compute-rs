//! Default Compute template program.

use fastly::{Error, Request, Response};

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
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
