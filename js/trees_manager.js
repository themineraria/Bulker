async function DownloadFilesList()
{
  //update dropbox files tree using a thread
  return new Promise(resolve => {
      remote.getGlobal('data').gdrive_tree = JSON.stringify(JSON.parse(require('./neon').getGdriveFilesList(remote.getGlobal('data').user_config['gdrive']['token'])).files);
      require('./neon').getDropboxFilesList(remote.getGlobal('data').user_config['dropbox']['token'], "", "true", (err, value) => {
        try {
          remote.getGlobal('data').dropbox_tree = JSON.stringify(JSON.parse(value));
        }
        catch (e) {
          resolve();
        }
        resolve();
      });
  });
}

function UpdateTrees()
{
  if (remote.getGlobal('data').dropbox_connected)
  {
    var dropbox_tree = {};
    require('./neon').getDropboxFilesList(remote.getGlobal('data').user_config['dropbox']['token'], "", "true", (err, value) => {
      var dropbox_files_list = value;
      dropbox_files_list = JSON.parse(dropbox_files_list.replace(/"\.tag":/gi, '"type":').replace(/"path_display":/gi, '"path":'));
      dropbox_files_list.forEach( (element, index) => {
        var splitpath = element.path.replace(/^\/|\/$/g, "").split('/');
      });
    });
  }
}

function addnode(obj){
  var splitpath = obj.path.replace(/^\/|\/$/g, "").split('/');
  var ptr = tree;
  for (i=0;i<splitpath.length;i++)
  {
    node = { name: splitpath[i],
    type: 'directory'};
    if(i == splitpath.length-1)
    {node.size = obj.size;node.type = obj.type;}
    ptr[splitpath[i]] = ptr[splitpath[i]]||node;
    ptr[splitpath[i]].children=ptr[splitpath[i]].children||{};
    ptr=ptr[splitpath[i]].children;
  }
}

function AddTreeElement(cloud, element)
{
  //TODO
}

function RemoveTreeElement(tree, element)
{
  //TODO
}
