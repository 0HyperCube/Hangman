function buttonClick() {
    var name = document.getElementById('name').value
    if (name.length < 2){
        document.getElementById('confirmButton').style.visibility = 'hidden';
    }
    else {
        document.getElementById('name').style.visibility = 'hidden';
        document.getElementById('confirmButton').style.visibility = 'hidden';
    }
}
