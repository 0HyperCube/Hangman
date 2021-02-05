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

	// If the submit button is pressed, the username is valid.
	Submit: async function () {

		this.UsernameBox.disabled = true;
		this.ConnectButton.disabled = true;
		this.ConnectButton.classList.add("ConnectingInfo");
		this.ConnectButton.innerHTML="Connecting...";
		
		let request = JSON.stringify({
				"name": this.UsernameBox.value
			});
		console.log(request);
		await fetch("/connect", {
			method: "POST",
			body: request
		});

		let x = await fetch("/getallmembers");
		let json = await x.json();
		console.log(json);
		console.log("isbob is ",json.is_bob," and if is bob is "+json.height+"cm");
	},
}

window.onload = function(){
	UsernameInput.Init();
};
