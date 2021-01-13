if(document.getElementById) {
	window.custom_alert = function(txt, focus_back) {
		Alert(txt, focus_back);
	}
  window.onerror = function(errorMsg, url, lineNumber) {
      Alert(errorMsg, null);
  }
}

function Alert(txt, focus_back) {
  var ALERT_TITLE = "Error";
  var ALERT_BUTTON_TEXT = "Ok";

	if(document.getElementById("alertContainer")) return;

	mObj = document.getElementsByTagName("body")[0].appendChild(document.createElement("div"));
	mObj.id = "alertContainer";
	mObj.style.height = document.documentElement.scrollHeight + "px";

	alertObj = mObj.appendChild(document.createElement("div"));
	alertObj.id = "alertBox";
	if(document.all && !window.opera) alertObj.style.top = document.documentElement.scrollTop + "px";
	alertObj.style.left = (document.documentElement.scrollWidth - alertObj.offsetWidth)/2 - 2 + "px";
	alertObj.style.visiblity="visible";

	h1 = alertObj.appendChild(document.createElement("h1"));
	h1.appendChild(document.createTextNode(ALERT_TITLE));

	msg = alertObj.appendChild(document.createElement("p"));
	msg.innerHTML = txt;
	msg.style.color = "red";
	msg.style.position = "relative";
	msg.style.left = "-20px";

	btn = alertObj.appendChild(document.createElement("a"));
	btn.id = "closeBtn";
	btn.appendChild(document.createTextNode(ALERT_BUTTON_TEXT));
	btn.href = "#";
	btn.focus();
	btn.onclick = function() { DestroyAlert(); if (focus_back != null){focus_back.focus();} return false; }

	alertObj.style.display = "block";
}

function DestroyAlert() {
	document.getElementsByTagName("body")[0].removeChild(document.getElementById("alertContainer"));
}
