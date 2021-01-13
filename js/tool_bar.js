document.addEventListener('DOMContentLoaded', function()
{
  var checker = document.getElementById('check');
  var refresh = document.getElementById('refresh');
  var download = document.getElementById('download');
  var upload = document.getElementById('upload');
  var deleter = document.getElementById('delete');
  var create_dir = document.getElementById('create_dir');
  var rename = document.getElementById('rename');
  var move = document.getElementById('move');
  var settings = document.getElementById('settings');

  checker.addEventListener('click', (event) =>
  {
    custom_alert("This button is not working atm !", null);
    /*if (remote.getGlobal('data').selected_files.length == 0) {}*/
  })

  create_dir.addEventListener('click', (event) =>
  {
    custom_alert("This button is not working atm !", null);
  })

  rename.addEventListener('click', (event) =>
  {
    custom_alert("This button is not working atm !", null);
  })

  move.addEventListener('click', (event) =>
  {
    custom_alert("This button is not working atm !", null);
  })

  deleter.addEventListener('click', (event) =>
  {
    if (remote.getGlobal('data').selected_files.length > 0) {
      remote.getGlobal('data').selected_files.forEach(
        element => {
          parsed_element = element.split(":");
          if (parsed_element[1] == "dropbox") {
            require('./neon').DropboxDelete(remote.getGlobal('data').user_config['dropbox']['token'], parsed_element[2]);
            remote.getGlobal('data').selected_files = [];
            //Never remove any element from database, even if deleted, to recover it in case it's needed
            /*db = remote.getGlobal('data').database;
            db.forEach((item, i) => {
              if (item.encrypted_name.toLowerCase() == parsed_element[2].split("/")[parsed_element[2].split("/").length-1])
              {
                db.splice(i, 1);
              }
            });
            remote.getGlobal('data').selected_files = [];
            remote.getGlobal('data').database = db;
            //require('./neon').updateDatabase(remote.getGlobal('data').encrypted_database_handler, JSON.stringify(remote.getGlobal('data').database));*/
            UpdateDiskSpace();
            DownloadFilesList().then(() => {
              DisplayExplorer();
            });
          }
      });
    }
    else
      custom_alert("No file as been selected !", null);
  });

  refresh.addEventListener('click', (event) =>
  {
    try {
      if (remote.getGlobal('data').dropbox_connected)
      {
        UpdateDiskSpace();
        DownloadFilesList().then(() => {
          DisplayExplorer();
        });
      }
    }
    catch (e) {
      custom_alert(e, null);
    }
  });

  download.addEventListener('click', (event) =>
  {
    if (remote.getGlobal('data').selected_files.length > 0)
      Window(340, 500, 'html/download.html');
    else
      custom_alert("No file as been selected !", null);
  });

  upload.addEventListener('click', (event) =>
  {
    ipcRenderer.send('open-files-dialog');
  });

  settings.addEventListener('click', (event) =>
  {
    Window(300, 450, 'html/settings.html');
  });

});


ipcRenderer.on('selected-files', (event, path) => {
   var buff_array = [];
    path.forEach(
      element => {
        buff_array.push(element.replace(/[\\]/g, '/'));
    });
    remote.getGlobal('data').selected_files = buff_array;
    Window(340, 500, 'html/upload.html').on('close', function() {
        UpdateDiskSpace();
        DownloadFilesList().then(() => {
          DisplayExplorer();
        });
    });
});
