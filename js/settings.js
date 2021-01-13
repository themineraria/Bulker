document.addEventListener('DOMContentLoaded', function()
{
  var connectGdriveBtn = document.getElementById('gdrive_connect');
  var disconnectGdriveBtn = document.getElementById('gdrive_disconnect');
  var connectDropboxBtn = document.getElementById('dropbox_connect');
  var disconnectDropboxBtn = document.getElementById('dropbox_disconnect');
  var code_thread = null;

  if (remote.getGlobal('data').dropbox_connected)
  {
    document.getElementById('dropbox_status').textContent = 'Connected';
    document.getElementById('dropbox_status').style.color = 'green';
    document.getElementById('dropbox_acc_name').textContent = JSON.parse(remote.getGlobal('data').dropbox_acc_info).name.display_name;
    document.getElementById('dropbox_acc_email').textContent = JSON.parse(remote.getGlobal('data').dropbox_acc_info).email;
    document.getElementById('dropbox_acc_country').textContent = JSON.parse(remote.getGlobal('data').dropbox_acc_info).country;
    connectDropboxBtn.style.display = 'none';
    disconnectDropboxBtn.style.display = 'inline-block';
  }
  else
  {
    disconnectDropboxBtn.style.display = 'none';
    connectDropboxBtn.style.display = 'inline-block';
  }

  if (remote.getGlobal('data').gdrive_connected)
  {
    document.getElementById('gdrive_status').textContent = 'Connected';
    document.getElementById('gdrive_status').style.color = 'green';
    document.getElementById('gdrive_acc_name').textContent = JSON.parse(remote.getGlobal('data').gdrive_acc_info).user.displayName;
    document.getElementById('gdrive_acc_email').textContent = JSON.parse(remote.getGlobal('data').gdrive_acc_info).user.emailAddress;
    connectGdriveBtn.style.display = 'none';
    disconnectGdriveBtn.style.display = 'inline-block';
  }
  else
  {
    disconnectGdriveBtn.style.display = 'none';
    connectGdriveBtn.style.display = 'inline-block';
  }

  connectGdriveBtn.addEventListener('click', (event) =>
  {
    var token_reply = JSON.parse(JSON.parse(require('../neon').gdrivePkceHandshake()));
    if (token_reply.access_token)
    {
      remote.getGlobal('data').user_config['gdrive']['token'] = token_reply.access_token;
      var gdrive_info = require('../neon').getGdriveInfo(remote.getGlobal('data').user_config['gdrive']['token']);
      remote.getGlobal('data').gdrive_connected = true;
      remote.getGlobal('data').gdrive_acc_info = gdrive_info;
      require('../neon').updateProfile(remote.getGlobal('data').encrypted_user_config_handler, TOML.stringify(remote.getGlobal('data').user_config));

      document.getElementById('gdrive_status').textContent = 'Connected';
      document.getElementById('gdrive_status').style.color = 'green';
      document.getElementById('gdrive_acc_name').textContent = JSON.parse(remote.getGlobal('data').gdrive_acc_info).user.displayName;
      document.getElementById('gdrive_acc_email').textContent = JSON.parse(remote.getGlobal('data').gdrive_acc_info).user.emailAddress;
      connectGdriveBtn.style.display = 'none';
      disconnectGdriveBtn.style.display = 'inline-block';
    }
  });

  disconnectGdriveBtn.addEventListener('click', (event) =>
  {
    remote.getGlobal('data').gdrive_connected = false;
    remote.getGlobal('data').gdrive_acc_info = null;
    remote.getGlobal('data').user_config['gdrive']['token'] = "";
    require('../neon').updateProfile(remote.getGlobal('data').encrypted_user_config_handler, TOML.stringify(remote.getGlobal('data').user_config));

    document.getElementById('gdrive_status').textContent = 'Not connected';
    document.getElementById('gdrive_status').style.color = 'red';
    document.getElementById('gdrive_acc_name').textContent = '';
    document.getElementById('gdrive_acc_email').textContent = '';
    disconnectGdriveBtn.style.display = 'none';
    connectGdriveBtn.style.display = 'inline-block';
  });

  connectDropboxBtn.addEventListener('click', (event) =>
  {
    state = makeid(20);
    shell.openExternal('https://www.dropbox.com/1/oauth2/authorize?client_id=2ezekn40iurcs2i&response_type=code&redirect_uri=http://localhost:65000/callback&state=' + state);
    try
    {
      if (code_thread === null)
        code_thread = new Promise(resolve => {
          require('../neon').waitUntilDropboxRedirect((err, value) => {
            remote.getGlobal('data').dropbox_code = value.split('&')[1].replace('code=', '').replace(' ', '');
            resolve();
          });
        });
      code_thread.then(() => {
        code_thread = null;
        var code = remote.getGlobal('data').dropbox_code;
        token_reply = JSON.parse(require('../neon').getDropboxToken(String(code)));
        if (token_reply.access_token)
        {
          remote.getGlobal('data').user_config['dropbox']['token'] = token_reply.access_token;
          require('../neon').getDropboxInfo(token_reply.access_token, (err, value) => {
            if (value && value != 'null')
            {
              remote.getGlobal('data').dropbox_connected = true;
              remote.getGlobal('data').dropbox_acc_info = value;
              require('../neon').updateProfile(remote.getGlobal('data').encrypted_user_config_handler, TOML.stringify(remote.getGlobal('data').user_config));

              document.getElementById('dropbox_status').textContent = 'Connected';
              document.getElementById('dropbox_status').style.color = 'green';
              document.getElementById('dropbox_acc_name').textContent = JSON.parse(remote.getGlobal('data').dropbox_acc_info).name.display_name;
              document.getElementById('dropbox_acc_email').textContent = JSON.parse(remote.getGlobal('data').dropbox_acc_info).email;
              document.getElementById('dropbox_acc_country').textContent = JSON.parse(remote.getGlobal('data').dropbox_acc_info).country;
              document.getElementById('dropbox_connect').style.display = 'none';
              disconnectDropboxBtn.style.display = 'inline-block';
            }
            else {
              console.log(err);
            }
          });
        }
      });
    }
    catch (e)
    {
      document.getElementById('dropbox_status').textContent = e;
      document.getElementById('dropbox_status').style.color = 'red';
    }
  })

  disconnectDropboxBtn.addEventListener('click', (event) =>
  {
    remote.getGlobal('data').dropbox_connected = false;
    remote.getGlobal('data').dropbox_acc_info = null;
    remote.getGlobal('data').user_config['dropbox']['token'] = "";
    require('../neon').updateProfile(remote.getGlobal('data').encrypted_user_config_handler, TOML.stringify(remote.getGlobal('data').user_config));

    document.getElementById('dropbox_status').textContent = 'Not connected';
    document.getElementById('dropbox_status').style.color = 'red';
    document.getElementById('dropbox_acc_name').textContent = '';
    document.getElementById('dropbox_acc_email').textContent = '';
    document.getElementById('dropbox_acc_country').textContent = '';
    disconnectDropboxBtn.style.display = 'none';
    connectDropboxBtn.style.display = 'inline-block';
  })
})
