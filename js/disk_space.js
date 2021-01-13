function DisplayDiskRawValues() {
  document.getElementById('percent').style.display = "none";
  document.getElementById('value').style.display = "block";
}
function DisplayDiskUsedPercentage() {
  document.getElementById('value').style.display = "none";
  document.getElementById('percent').style.display = "block";
}
async function UpdateDiskSpace()
{
  if (remote.getGlobal('data').dropbox_connected)
  {
    return new Promise((resolve, reject) => {
      require('./neon').getDropboxSpace(remote.getGlobal('data').user_config['dropbox']['token'], (err, value) => {
        if (value)
        {
          remote.getGlobal('data').dropbox_space = value;
          var dropbox_space = JSON.parse(remote.getGlobal('data').dropbox_space);
          var dropbox_used = dropbox_space.used/1024/1024;
          var dropbox_allocated = dropbox_space.allocation.allocated/1024/1024;
          document.getElementById('value').innerHTML = Math.round(dropbox_used) + "/" + Math.round(dropbox_allocated) + "Mb";
          document.getElementById('percent').innerHTML = Math.round(((dropbox_used*100)/dropbox_allocated)*10)/10 + "%";
          document.getElementById('progress').style.width = Math.round((dropbox_used*100)/dropbox_allocated) + "%";
          resolve();
        }
        else {
          console.log(err);
          reject();
        }
      });
    });
  }
}
