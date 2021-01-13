document.addEventListener('DOMContentLoaded', function()
{
  var login_button = document.getElementById('login_button');
  var usernames_selector = document.getElementById('usernames_selector');
  var password_field = document.getElementById('password_field');
  var login_form_container = document.getElementById('login_form_container');
  var animation_switch = document.getElementById('animation_switch');

  usernames_selector.addEventListener('mousedown', (event) =>
  {
    usernames_selector.innerHTML = ''; //remove all choices
    usernames_selector.appendChild(document.createElement('option')); //add the first empty choice

    var users = require('./neon').listProfiles('profiles/'); //list all profiles

    if (users != '')
    {
      var users_array = users.split(';');
      for (i = 0; i < users_array.length; i++)
      {
        var opt = document.createElement('option');
        opt.value = users_array[i];
        opt.innerHTML = users_array[i];
        usernames_selector.appendChild(opt); //add profiles to menu
      }
    }
  });

  login_button.addEventListener('click', (event) =>
  {
    if (usernames_selector.value != '' && password_field.value != '') //if fields are not empty
    {
      var username = usernames_selector.value;
      var password = password_field.value.replace(/[^a-zA-Z0-9]/gi, '');
      login(username, password).then(success => {
        if (success)
        {
          console.log("Info: Logged in");
        }
        else {
          custom_alert("Unable to log in, please verify username & password", password_field);
        }
      });
    }
    else
    {
      custom_alert("Field(s) can't be empty !", password_field);
    }
  });

  password_field.addEventListener('keydown', function(event)
  {
    if (event.keyCode === 13)
    {
      event.preventDefault();
      document.getElementById('login_button').click();
    }
  });
});

function OpenRegistration()
{
  Window(300, 450, 'html/registration.html');
}

function BackgroundAnimation()
{
  var particles = document.getElementById('particles-js');
  if (document.getElementById('animated').checked == true)
  {
    particles.style.display = 'block';
  }
  else
  {
    particles.style.display = 'none';
  }
}

async function login(username, password)
{
  //Reading user profile
  var profile_promise = new Promise((resolve, reject) => {
    require('./neon').generateEncryptedHandler(username, password, 'profiles/' + username + '/' + username + '.user', (err, value) => {
      remote.getGlobal('data').encrypted_user_config_handler = value;
      console.log("Info: User profile handler created");
      require('./neon').readProfile(remote.getGlobal('data').encrypted_user_config_handler, (err, value) => {
        if (value) {
          remote.getGlobal('data').user_config = TOML.parse(value);
          console.log("Info: User profile read");
          resolve();
        }
        else {
          reject(err);
        }
      });
    });
  });

  //Reading user database
  var database_promise = new Promise((resolve, reject) => {
    require('./neon').generateEncryptedHandler(username, password, 'profiles/' + username + '/' + username + '.db', (err, value) => {
      remote.getGlobal('data').encrypted_database_handler = value;
      console.log("Info: Database handler created");
      require('./neon').readDatabase(remote.getGlobal('data').encrypted_database_handler, (err, value) => {
        if (value) {
          remote.getGlobal('data').database = JSON.parse(value);
          console.log("Info: Database read");
          resolve();
        }
        else {
          reject(err);
        }
      });
    });
  });

  //Login
  return Promise.all([profile_promise, database_promise]).then((values) => {
    remote.getGlobal('data').user_connected = true;
    var dropbox_token = remote.getGlobal('data').user_config['dropbox']['token'];
    var gdrive_token = remote.getGlobal('data').user_config['gdrive']['token'];
    if (dropbox_token)
    {
      var dropbox_connected_promise = new Promise((resolve, reject) => {
        require('./neon').getDropboxInfo(dropbox_token, (err, value) => {
          if (value && value != 'null')
          {
            remote.getGlobal('data').dropbox_acc_info = value;
            remote.getGlobal('data').dropbox_connected = true;
            console.log("Info: Dropbox account info retrieved");
            console.log("Info: Dropbox account connected");
            resolve();
          }
          else {
            console.log(err);
            reject();
          }
        });
      });
    }
    if (gdrive_token)
    {
      var gdrive_connected_promise = new Promise((resolve, reject) => {
        var value = require('./neon').getGdriveInfo(gdrive_token);
        remote.getGlobal('data').gdrive_acc_info = value;
        remote.getGlobal('data').gdrive_connected = true;
        console.log("Info: Gdrive account info retrieved");
        console.log("Info: Gdrive account connected");
        resolve();
      });
    }
    //AnyPromise is a const implementing Promise.any()
    AnyPromise([dropbox_connected_promise, gdrive_connected_promise]).then((values) => {
      if (remote.getGlobal('data').dropbox_connected || remote.getGlobal('data').gdrive_connected)
      {
        UpdateDiskSpace().then(() => {
          console.log("Info: DiskSpace updated");
        });
        DownloadFilesList().then(() => {
          console.log("Info: Files list downloaded");
          //UpdateTrees(); // WIP
          DisplayExplorer();
          console.log("Info: Explorer displayed");
        });
      }
    });

    //Hide login form
    login_form_container.style.display = 'none';
    animation_switch.style.display = 'none';

    return true;
  }, reason => {
    console.log(reason);
    return false;
  });
}
