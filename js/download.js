document.addEventListener('DOMContentLoaded', function()
{
  var selected_files = remote.getGlobal('data').selected_files;
  if (selected_files.length <= 1) {document.getElementById('download-title').innerHTML = selected_files.length + " file to download:";}
  else {document.getElementById('download-title').innerHTML = selected_files.length + " files to download:";}
  selected_files.forEach(
    file => {
      var parsed_file = file.split(":");
      if (parsed_file[0] == "file")
      {
        document.getElementById('files-to-download').value += parsed_file[2] + "\n";
      }
  });

  document.getElementById('download_directory_selector').addEventListener('click', (event) =>
  {
    ipcRenderer.send('open-folder-dialog');
  });

  document.getElementById('download').addEventListener('click', (event) =>
  {
      if (selected_files.length > 0)
      {
        selected_files.forEach(
          file => {
            var parsed_file = file.split(":");
            if (parsed_file[0] == "file")
            {
              if (parsed_file[1] == "dropbox") {
                var encrypted = false;
                var key_username = remote.getGlobal('data').user_config['general']['username'];
                var key_pass = "";
                var db = remote.getGlobal('data').database;
                db.forEach(
                  item => {
                    if (item.encrypted_name.toLowerCase() == parsed_file[2].split("/")[parsed_file[2].split("/").length-1])
                    {
                      encrypted = true;
                      real_name = item.real_name;
                      key_pass = item.password;
                      require('../neon').DropboxDownload(remote.getGlobal('data').user_config['dropbox']['token'], parsed_file[2], document.getElementById('selected-path').value, real_name, key_username, key_pass, remote.getGlobal('data').encrypted_database_handler);
                    }
                });
                if (encrypted != true)
                {
                  require('../neon').DropboxDownload(remote.getGlobal('data').user_config['dropbox']['token'], parsed_file[2], document.getElementById('selected-path').value, "");
                }
              }
            }
        });
      	remote.getCurrentWindow().close();
      }
  });
});

ipcRenderer.on('selected-directory', (event, path) => {
    document.getElementById('selected-path').value = path;
});
