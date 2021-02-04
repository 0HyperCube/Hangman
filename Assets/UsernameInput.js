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
	Submit: function () {
		let name = this.UsernameBox.value;
		this.style.visibility = "hidden";
		this.ConnectButton.style.visibility = "hidden";
		alert("TODO: Implement GraphQL to send the username ", name);
	},
}

window.onload = function(){
	UsernameInput.Init();
};
