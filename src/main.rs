use serde::{Deserialize, Serialize};
use tide::{http::mime, Request, Response};


async fn connect(mut request: Request<()> ) -> tide::Result<impl Into<Response>> {
	println!("got a connect request with a username object");
	let text : &str = &request.take_body().into_string().await?;
	println!("Got connect {}", text);
	let query : Username = serde_json::from_str(text)?;
	println!("Got name {}", query.name);
	

	Ok(Response::builder(200).build())

}

async fn request(_: Request<()> ) -> tide::Result<impl Into<Response>> {
	
	let response : TestResponse = TestResponse{is_bob:true,height:43};

	Ok(Response::builder(200)
		.body(serde_json::to_string(&response)?)
		.content_type(mime::HTML))

}

#[derive(Serialize, Deserialize)]
struct Username {
	 name: String,
}

#[derive(Serialize, Deserialize)]
struct TestResponse {
	is_bob : bool,
	height : u32,

}

#[async_std::main]
async fn main() -> std::result::Result<(), std::io::Error> {
	tide::log::start();
	let mut app = tide::new();
	app.at("/").serve_dir("Assets/")?;
	app.at("/").serve_file("Assets/Index.html")?;
	app.at("/connect").post(connect);
	app.at("/request").get(request);
	app.listen("127.0.0.1:6543").await?;
	Ok(())
	
}
