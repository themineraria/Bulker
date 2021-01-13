var encryption = false;
document.addEventListener('DOMContentLoaded', function()
{
  if (remote.getGlobal('data').selected_files.length <= 1)
  {document.getElementById('upload-title').innerHTML = remote.getGlobal('data').selected_files.length + " file to upload:";}
  else {document.getElementById('upload-title').innerHTML = remote.getGlobal('data').selected_files.length + " files to upload:";}

  remote.getGlobal('data').selected_files.forEach(
    element => {
      document.getElementById('files-to-upload').value += element + "\n";
  })

  document.getElementById('upload').addEventListener('click', (event) =>
  {
    try {
      if (document.getElementById('selected-path').value != "")
      {
        var cloud_selector = document.getElementById('cloud-selector');
        if (cloud_selector.value == "dropbox")
        {
          remote.getGlobal('data').selected_files.forEach(
            element => {
              if (encryption == true)
              {
                encrypted_name = makeid(20);
                key_username = remote.getGlobal('data').user_config['general']['username'];
                key_pass = makeid(20);
                var reply = require('../neon').DropboxUpload(remote.getGlobal('data').user_config['dropbox']['token'], element, document.getElementById('selected-path').value, encrypted_name, key_username, key_pass, remote.getGlobal('data').encrypted_database_handler);
                name = element.split("/")[element.split("/").length-1];
                db = remote.getGlobal('data').database;
                db.push({"encrypted_name":encrypted_name,"real_name":name,"remote_path":document.getElementById('selected-path').value,"password":key_pass});
                remote.getGlobal('data').database = db;
                //require('../neon').updateDatabase(remote.getGlobal('data').encrypted_database_handler, JSON.stringify(remote.getGlobal('data').database));
                if (reply != null)
                  AddTreeElement("dropbox", reply);
              }
              else {
                encrypted_name = "";
                var reply = require('../neon').DropboxUpload(remote.getGlobal('data').user_config['dropbox']['token'], element, document.getElementById('selected-path').value, encrypted_name);
                if (reply != null)
                  AddTreeElement("dropbox", reply);
              }
          })
          remote.getGlobal('data').selected_files = [];
        	remote.getCurrentWindow().close();
        }
      }
    } catch (e) {
      alert(e);
    }
  });
});

function UploadEncryption()
{
  if (document.getElementById('encryption').checked == true)
  {
    encryption = true;
  }
  else
  {
    encryption = false;
  }
}
