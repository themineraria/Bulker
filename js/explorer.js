function DisplayExplorer(path)
{
  document.getElementById("explorer").innerHTML = ''; //Clear everything displayed in the explorer
  var clouds_tree = {dropbox: remote.getGlobal('data').dropbox_tree, gdrive: remote.getGlobal('data').gdrive_tree}; //array of all trees
  //console.log(clouds_tree["dropbox"]);
  /*array of all the entries name in trees*/
  var trees_entries = {
    dropbox: {tag: ".tag", path: "path_lower", name: "name", date: "client_modified", size: "size", hash: "content_hash"},
    gdrive: {tag: "kind", path: "name", name: "name", date: "none", size: "none", hash: "none"} //FIX GDRIVE TREE
  };
  if (path == null || path == "/" || path == "\\" || (path.split(":").length > 1 && path.split(":")[1] == "")) {path = "";} //root path in anyway
  remote.getGlobal('data').current_explorer_path = path;
  if (path.split(":").length == 2) //if we want to display something from a specific cloud (!= root)
  {
    var cloud = path.split(":")[0];
    var path = path.split(":")[1];
    var previous_path = path.split("/");
    previous_path.pop();
    /*add an entry for the parent directory*/
    document.getElementById("explorer").innerHTML += `
    <div class="row" onclick="DisplayExplorer(\'` + cloud + ":" + previous_path.join('/') + `\');">
      <div class="logo">
        <img src="img/parent_folder.png" draggable="false">
      </div>
      <div class="name"> <p>. . .</p> </div>
    </div>`;
    JSON.parse(clouds_tree[cloud]).forEach(element => { //For the tree of our specific cloud
      /*check if the element is in the right path*/
      if (element['path_lower'].split("/").length-1 == path.split("/").length && element['path_lower'].startsWith(path))
      {
        /*check if element as been selected*/
        var is_checked = AsBeenSelected(element[trees_entries[cloud]["tag"]], cloud, element[trees_entries[cloud]["path"]]);
        /*check if element is encrypted*/
        var is_encrypted = true;
        var element_name = UnencryptedName(element[trees_entries[cloud]["name"]]);
        if (element_name == "")
        {
          element_name = element[trees_entries[cloud]["name"]];
          is_encrypted = false;
        }
        /*create entry from element*/
        var entry = GenerateEntry(cloud, element[trees_entries[cloud]["tag"]], element[trees_entries[cloud]["path"]], element_name,
        is_checked, is_encrypted, element[trees_entries[cloud]["date"]], element[trees_entries[cloud]["size"]], element[trees_entries[cloud]["hash"]]);
        document.getElementById("explorer").innerHTML += entry; //add entry to explorer
      }
    });
  }
  else { //if we want to display root
    for (cloud in clouds_tree) { //for each tree in clouds_tree
        JSON.parse(clouds_tree[cloud]).forEach(element => {
          /*check if the element is in the root path
          because all paths starts with a '/' splitting the path of anything in root will give an array with a length of 2*/
          if (element[trees_entries[cloud]["path"]].split("/").length == 2 || element[trees_entries[cloud]["path"]].split("/").length == 1) //Second case is for Gdrive because we're using the name as a path since it's not in the tree
          {
            /*check if element as been selected*/
            var is_checked = AsBeenSelected(element[trees_entries[cloud]["tag"]], cloud, element[trees_entries[cloud]["path"]]);
            /*check if element is encrypted*/
            var is_encrypted = true;
            var element_name = UnencryptedName(element[trees_entries[cloud]["name"]]);
            if (element_name == "")
            {
              element_name = element[trees_entries[cloud]["name"]];
              is_encrypted = false;
            }
            /*create entry from element*/
            var entry = GenerateEntry(cloud, element[trees_entries[cloud]["tag"]], element[trees_entries[cloud]["path"]], element_name,
            is_checked, is_encrypted, element[trees_entries[cloud]["date"]], element[trees_entries[cloud]["size"]], element[trees_entries[cloud]["hash"]]);
            document.getElementById("explorer").innerHTML += entry; //add entry to explorer
          }
        });
      }
  }
  /*Add the drag & drop zone to upload files*/
  document.getElementById("explorer").innerHTML += "<div id=\"dropzone\"><p>Drop file(s) here to upload...</p><div id=\"dropzone-hover\"></div></div>";
  if (window.File && window.FileList && window.FileReader)
  	DropzoneInit();
}

function AsBeenSelected(tag, cloud, path)
{
  var result = false;
  remote.getGlobal('data').selected_files.forEach(selected_file => {
    if (selected_file.split(':')[0] == tag && selected_file.split(':')[1] == cloud && selected_file.split(':')[2] == path)
      result = true;
  });
  return result;
}

function UnencryptedName(encrypted_name)
{
  var result = "";

  if (!remote.getGlobal('data').database) //Database is empty
    return "";

  remote.getGlobal('data').database.forEach(item => {
    if (item.encrypted_name == encrypted_name)
      result = item.real_name;
  });
  return result;
}

function GenerateEntry(cloud, tag, path, name, is_checked, is_encrypted, date, size, hash)
{
  if (tag == "drive#file") //TODO: FIX
    tag = "file";
  var entry = `
  <div class="row">
    <span id="path" hidden>` + cloud + ":" + path + `</span>
    <div class="logo"> <img src="img/`+ cloud +`.png" draggable="false"> </div>
    <div class="selector">
      <input type="checkbox" ` + (is_checked ? "checked" : "") + `>
      <div class="selectorswitcher" onclick="
        this.parentNode.children[0].checked=!this.parentNode.children[0].checked;
        SelectEntries(\'`+ tag + `\',\'`+ cloud +`\',\'` + path + `\', this.parentNode.children[0].checked);
        ">
      </div>
    </div>
    <div class="name" onclick="`;
    if (tag == "folder") //TODO: FIX GDRIVE NO FOLDER TAG
      entry += `DisplayExplorer(this.parentNode.children[0].textContent)`;
    else
      entry += `DisplayEntryInfo(\'` + name + `\',\'` + date + `\',\'` + size + `\',\'` + path + `\',\'` + hash + `\',\'`+ is_encrypted +`\')`;
    entry += `">
      <img src="img/`+ tag +`.png" draggable="false">
      <p>` + name + `</p>
    </div>`;
    if (tag != "folder")
    {
      if (Boolean(is_encrypted) == true)
        entry += `<div class="attributes"> <img src="img/lock.png" draggable="false"> </div>`;
      else
        entry += `<div class="attributes"> <img src="img/open_lock.png" draggable="false"> </div>`;
    }
    entry += `</div>`;
    return entry;
}

function DisplayEntryInfo(name, date, size, path, hash, encryption)
{
  document.getElementById("file_name").textContent = name;
  document.getElementById("file_date").textContent = date;
  document.getElementById("file_path").textContent = path;
  document.getElementById("file_hash").textContent = hash;
  sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  i = parseInt(Math.floor(Math.log(size) / Math.log(1024)));
  document.getElementById("file_size").textContent = Math.round(size / Math.pow(1024, i), 2) + ' ' + sizes[i];
  document.getElementById("file_encryption").textContent = encryption;
}

function SelectEntries(tag, cloud, path, selected)
{
  var selected_files = remote.getGlobal('data').selected_files;
  if (selected) //Means we want to add this file to the selected ones
  {
    if (tag == "folder")
    {
      var toAdd = ListFolderEntriesRecursive(tag, cloud, path); //List all files in the folder
      selected_files = selected_files.concat(toAdd); //Add the files
    }
    else {
      selected_files.push(tag + ':' + cloud + ':' + path); //Add the file
    }
  }
  else {
    if (tag == "folder") {
      var toRemove = ListFolderEntriesRecursive(tag, cloud, path); //List all files in the folder
      selected_files = selected_files.filter(element => !toRemove.includes(element)); //Remove the files
    }
    else {
      var index = selected_files.indexOf(tag + ':' + cloud + ':' + path); //Search the file to remove
      if (index !== -1) {selected_files.splice(index, 1)}; //Remove the file
    }
  }
  remote.getGlobal('data').selected_files = selected_files;
}

function ListFolderEntriesRecursive(tag, cloud, path)
{
  var clouds_tree = {dropbox: remote.getGlobal('data').dropbox_tree};
  var trees_entries = {
    dropbox: {tag: ".tag", path: "path_lower", name: "name", date: "client_modified", size: "size", hash: "content_hash"}
  };
  var result = new Array();
  JSON.parse(clouds_tree[cloud]).forEach(
    element => {
      if (element[trees_entries[cloud]["path"]].startsWith(path))
      {
        result.push(tag+":"+cloud+":"+path);
      }
  });
  return result;
}
