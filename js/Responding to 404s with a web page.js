/// <reference types="@fastly/js-compute" />

import { includeBytes } from "fastly:experimental";

// Load a static file as a Uint8Array at compile time.
// File path is relative to root of project, not to this file
const notfoundPage = includeBytes("./src/not-found.html");

const handler = async (event) => {
 // get the request from the client
 const req = event.request

 const backendResponse = await fetch(req, {
   backend: "vcl-origin",
   cacheOverride: new CacheOverride("pass")
 });

 // Handle 404s with a custom response
 if (backendResponse.status == 404) {
   return new Response(notfoundPage, {
     status: 404,
   });
 }

 // If status is not 404, send the backend response to the client
 if (backendResponse.status != 404) {
   // Create a new headers object to avoid modifying the original
   const newHeaders = new Headers(backendResponse.headers);
   newHeaders.set("x-tacos", "We love tacos!");

   // Create new response with the modified headers
   return new Response(backendResponse.body, {
     status: backendResponse.status,
     headers: newHeaders
   });
 }
}

addEventListener("fetch", event => event.respondWith(handler(event)));
