function DropzoneInit() {
  var filedrag = document.getElementById('explorer');
  var dropzone = document.getElementById('dropzone');
  var dropzone_hover = document.getElementById('dropzone-hover');
  var slider = document.getElementsByClassName("gutter");
  // is XHR2 available?
  var xhr = new XMLHttpRequest();
  if (xhr.upload) {
    filedrag.addEventListener("dragover", DropzoneDragedHover, false);
    dropzone_hover.addEventListener("dragleave", DropzoneDragedHover, false);
    slider[0].addEventListener("dragleave", DropzoneDragedHover, false);
    dropzone_hover.addEventListener("drop", DropzoneDroped, false);
  }
  else {
    custom_alert("Drag&drop not available !", null);
  }
}

function DropzoneDragedHover(e) {
	e.stopPropagation();
	e.preventDefault();
  var dropzone = document.getElementById('dropzone');
  dropzone.style.display = (e.type == "dragover" ? "block" : "none");
}

function DropzoneDroped(e) {
	// cancel event and hover styling
	DropzoneDragedHover(e);

	// fetch FileList object
	var files = e.target.files || e.dataTransfer.files;
  remote.getGlobal('data').selected_files = [];
  var folder_in = 0;
	// process all File objects
	for (var i = 0, f; f = files[i]; i++) {
		folder_in += DropzoneParse(f);
	}
  if (folder_in > 0)
  {
    custom_alert("Folders can't be upload, they are gonna be ignored.", null);
  }
  Window(340, 500, 'html/upload.html', true).on('close', function() {
      UpdateDiskSpace();
      DownloadFilesList().then(() => {
        DisplayExplorer();
      });
    });
}

function DropzoneParse(file) {
  buff_array = remote.getGlobal('data').selected_files;
  var folder_in = 0;
  if (fs.lstatSync(file.path).isFile())
  {
    buff_array.push(file.path.replace(/[\\]/g, '/'));
  }
  else if (fs.lstatSync(file.path).isDirectory())
  {
    //Do nothing here 'cause we don't want to upload folders
    folder_in = 1; //return that a folder as been drop to display error
  }
  remote.getGlobal('data').selected_files = buff_array;
  return folder_in;
}
