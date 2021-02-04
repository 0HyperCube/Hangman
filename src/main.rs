use serde::{Deserialize, Serialize};
use tide::prelude::*; // Pulls in the json! macro.
use tide::{http::mime, Body, Request, Response};

#[derive(Deserialize, Serialize)]
struct Username {
    name: String,
}

async fn connect( mut request: Request<()> ) -> tide::Result<impl Into<Response>> {
	//let query : Username = request.body_json().await?;
	//println!("Got query {}", query.name);
	

	Ok(Response::builder(200)
        .body("hello from the other side!!")
        .content_type(mime::HTML))

}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
	tide::log::start();
	let mut app = tide::new();
	app.at("/").serve_dir("Assets/")?;
	app.at("/").serve_file("Assets/Index.html")?;
	app.at("/connect").get(connect);
	app.listen("127.0.0.1:6543").await?;
	Ok(())
}
