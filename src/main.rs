
use serde::{Deserialize, Serialize};
use tide::{Request, Response, http::{mime}};
use std::{collections::LinkedList};



#[derive(Serialize, Deserialize)]
struct LobbyUser{
	username : String,
	last_fetch_action : u64,
	is_joining : bool,
}

#[derive(Serialize, Deserialize)]
struct Lobby{
	latest_action: u64,
	users_in_lobby: LinkedList<LobbyUser>,
}

#[derive(Serialize, Deserialize)]
struct ClientConnection {
	username: String,
}
static mut LOBBY : Lobby = 
Lobby{
	latest_action: 0,
	users_in_lobby : LinkedList::new()
};

async fn connect(mut request: Request<()> ) -> tide::Result<impl Into<Response>> {
	println!("Recieved new connection request");
	let str : &str = &request.take_body().into_string().await?;
	let client_connection : ClientConnection = serde_json::from_str(str)?;
	println!("Got name {}", client_connection.username);


	// Add user to lobby
	unsafe {
		LOBBY.users_in_lobby.push_back(LobbyUser{ username: client_connection.username, last_fetch_action: 0, is_joining: false});
	}

	Ok(Response::builder(200).build())

}

async fn get_lobby(_: Request<()> ) -> tide::Result<impl Into<Response>> {
	let lobby_str = serde_json::to_string(unsafe{ &LOBBY }).unwrap();

	Ok(Response::builder(200)
		.body(lobby_str)
		.content_type(mime::HTML))

}

#[async_std::main]
async fn main() -> std::result::Result<(), std::io::Error> {
	println!("{}", u64::MAX);

	tide::log::start();
	let mut app = tide::new();

	app.at("/").serve_dir("Assets/")?;
	app.at("/").serve_file("Assets/Index.html")?;
	app.at("/connect").post(connect);
	app.at("/getlobby").get(get_lobby);
	app.listen("127.0.0.1:6543").await?;
	Ok(())
	
}
