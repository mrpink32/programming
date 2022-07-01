function HideAndShow(x) {
	if (document.getElementById("display-switch" + x).style.display == "initial") {
		document.getElementById("display-switch" + x).style.display = "none";
	}
	else {
		document.getElementById("display-switch" + x).style.display = "initial";
	}
}