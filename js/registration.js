document.addEventListener("DOMContentLoaded", function() {

  var registerBtn = document.getElementById('registration_button');
  var username_field = document.getElementById('username_field');
  var password_field = document.getElementById('password_field');
  var error = document.getElementById('error');

  var need_confirm = false;
  var register_click_count = 0;
  var previous_username = "";

  registerBtn.addEventListener('click', (event) => {

    username = username_field.value.replace(/[^a-zA-Z0-9]/gi,'');
    password = password_field.value.replace(/[^a-zA-Z0-9]/gi,'');

    if (previous_username != username)
    {
      register_click_count = 0;
    }

    register_click_count+=1;

    try {
      var profile_promise = new Promise((resolve, reject) => {
        require('../neon').generateEncryptedHandler(username, password, "profiles/" + username + "/" + username + ".user", (err, value) => {
          remote.getGlobal('data').encrypted_user_config_handler = value;
          resolve();
        });
      });
      var database_promise = new Promise((resolve, reject) => {
        require('../neon').generateEncryptedHandler(username, password, "profiles/" + username + "/" + username + ".db", (err, value) => {
          remote.getGlobal('data').encrypted_database_handler = value;
          resolve();
        });
      });

      return Promise.all([profile_promise, database_promise]).then((values) => {
        if (require('../neon').profileExists("profiles/" + username + "/" + username + ".user") == "true" && register_click_count == 2) {
          console.log("Profile already exist.");
          error.textContent = "";
          register_click_count = 0;
          require('../neon').deleteProfile("profiles/" + username + "/" + username + ".user");
          require('../neon').createProfile(remote.getGlobal('data').encrypted_user_config_handler, username);

          require('../neon').deleteDatabase("profiles/" + username + "/" + username + ".db");
          require('../neon').createDatabase(remote.getGlobal('data').encrypted_database_handler);

          Close();
        }
        else if (require('../neon').profileExists("profiles/" + username + "/" + username + ".user") == "true" && register_click_count == 1) {
          console.log("Are you sure");
          error.textContent = "User already exist! Click button again to force override anyway.";
        }
        else if (require('../neon').profileExists("profiles/" + username + "/" + username + ".user") == "false") {
          console.log("Creating");
          error.textContent = "";
          require('../neon').createProfile(remote.getGlobal('data').encrypted_user_config_handler, username);
          require('../neon').createDatabase(remote.getGlobal('data').encrypted_database_handler);
          Close();
        }
        previous_username = username;
        return true;
      }, reason => {
        console.log(reason);
        return false;
      });
    }
    catch (e) {
      console.log(e);
    }
  });

  password_field.addEventListener("keydown", function(event) {
      if (event.keyCode === 13) {
          event.preventDefault();
          registerBtn.click();
      }
  });

});
