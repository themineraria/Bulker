function Quit() {
	remoteapp.quit();
}

function Minimize() {
	remote.getCurrentWindow().minimize();
}

function Maximize() {
	var window = remote.getCurrentWindow();
	window.isMaximized() ? window.unmaximize() : window.maximize();
}

function Window(height, width, url, console) {
  var win = new BrowserWindow({
  height: height,
  width: width,
  minHeight: height,
  minWidth: width,
  webPreferences: { nodeIntegration: true },
	frame: false
  });
  win.loadFile(url);
  if (console === true)
		win.webContents.openDevTools();
  return win;
}

function Close() {
	remote.getCurrentWindow().close();
}
