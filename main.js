const electron = require('electron');
const path = require('path');
const url = require('url');

// Module to control application life.
// Module to create native browser window.
// Module to control file selection.
// Module to create dialog.
const { app, BrowserWindow, ipcMain, dialog } = electron;

// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the JavaScript object is garbage collected.
let mainWindow;

global.data = {
  encrypted_user_config_handler: null, //Rust struct
  user_config: null, //TOML object

  encrypted_database_handler: null, //Rust struct
  database: null, //String JSON

  user_connected: false, //Boolean for user connexion

  current_explorer_path: "", //String like dropbox:/file.exe
  selected_files: [], //Array of elements like: file:dropbox:/path/to/element, folder:dropbox:/path/to/directory
  trees: {dropbox: null},

  dropbox_connected: false, //Boolean for dropbox connexion
  dropbox_code: "", //String containing the first dropbox code replied used to retrieve token
  dropbox_acc_info: "", //String JSON object
  dropbox_tree: "",  //String JSON tree
  dropbox_space: "",  //String JSON tree

  gdrive_connected: false,
  gdrive_acc_info: "",
  gdrive_tree: "",
};

function createWindow()
{
  // Create the browser window.
  mainWindow = new BrowserWindow(
  {
    height: 600,
    width: 800,
    minHeight: 600,
    minWidth: 800,
    webPreferences:
    {
      nodeIntegration: true
    },
    frame: false
  });

  // open dev tools for debugging
  mainWindow.webContents.openDevTools();

  // and load the index.html of the app.
  mainWindow.loadFile('index.html');

  // Emitted when the window is closed.
  mainWindow.on('closed', () =>
  {
    // Dereference the window object, usually you would store windows
    // in an array if your app supports multi windows, this is the time
    // when you should delete the corresponding element.
    mainWindow = null;
  });
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on('ready', createWindow);

// Quit when all windows are closed.
app.on('window-all-closed', () =>
{
  // On OS X it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== 'darwin')
  {
    app.quit();
  }
});

app.on('activate', () =>
{
  // On OS X it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (mainWindow === null)
  {
    createWindow();
  }
});

ipcMain.on('open-folder-dialog', (event) => {
  dialog.showOpenDialog({
    properties: ['openFile', 'openDirectory']
  }).then(result => {
        if(!result.canceled)
        {
          event.sender.send('selected-directory', result.filePaths)
        }
      }).catch(err => {
        custom_alert(err)
      })
})

ipcMain.on('open-files-dialog', (event) => {
  dialog.showOpenDialog({
    properties: ['openFile', 'multiSelections']
  }).then(result => {
        if(!result.canceled)
        {
          event.sender.send('selected-files', result.filePaths)
        }
      }).catch(err => {
        custom_alert(err)
      })
})
