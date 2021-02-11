var UsernameInput = {
	UsernameBox:	"hi",
	ConnectButton:	null,

	Init: function(){
		this.UsernameBox	= document.getElementById("Username");
		this.ConnectButton	= document.getElementById("ConnectButton");
		this.OnEdit();
	},

	// Handles editing of the username box. Will disable the button if invalid
	OnEdit: function(){
		
		if(this.UsernameBox.value.length<2){
			this.UsernameBox.classList.add("Disabled");
			this.ConnectButton.disabled = true;
		}else{
			this.UsernameBox.classList.remove("Disabled");
			this.ConnectButton.disabled = false;
		}
	},

	SendUsername: async function(){
		console.log("Creating connect request.");

		let request = JSON.stringify({
				"username": this.UsernameBox.value
			});

		console.log("Sending connect request. String of data sending: ",request);

		await fetch("/connect", {
			method: "POST",
			body: request
		});

		console.log("Sent connect request");
	},

	FetchLobby: async function(){
		console.log("Fetching lobby");
		let x = await fetch("/getlobby");
		console.log("Decoding lobby. Response recieved: ", x);
		let json = await x.json();
		console.log("Sucsessfully decoded JSON. JSON data: ",json);
	},

	// If the submit button is pressed, the username is valid.
	Submit: async function () {

		this.UsernameBox.disabled = true;
		this.ConnectButton.disabled = true;
		this.ConnectButton.classList.add("ConnectingInfo");
		this.ConnectButton.innerHTML="Connecting...";
		
		await this.SendUsername();

		await this.FetchLobby();
	},
}

window.onload = function(){
	UsernameInput.Init();
};
