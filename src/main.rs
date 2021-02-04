

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
	tide::log::start();
	let mut app = tide::new();
	app.at("/").serve_dir("Assets/")?;
	app.at("/").serve_file("Assets/Index.html")?;
	app.listen("127.0.0.1:6543").await?;
	Ok(())
}
