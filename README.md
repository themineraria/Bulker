[![pipeline status](https://gitlab.com/cyprien.isnard/clouder/badges/master/pipeline.svg)](https://gitlab.com/cyprien.isnard/clouder/commits/master)

# Bulker

<table><tr><td><strong>This project is still under developpement, do not use in any kind of production !</strong></td></tr></table>

Bulker is a multi-cloud client to synchronize, encrypt & automate your files backups on Dropbox, Drive, OneDrive, etc.
This program is written in rust for the backend, using the [neon](https://neon-bindings.com/) crate to let [electron](https://www.electronjs.org/) manage the frontend.
It uses the api of the websites via libcurl encapsulated in the [curl](https://crates.io/crates/curl) rust crate.

## Getting Started

### Windows & Linux & Mac OS

	1) Make sure you have installed the latest versions of Node.js & Rust
	2) Git clone & cd in the folder
	3) Install electron globally using “npm install electron -g” if needed
	4) Run "npm install" & "npm start" to build project

### Other OS

    No idea, sorry ¯\_(ツ)_/¯
    (But if you can install Node.js & Rust, the steps should be quite similar. I'm sure you'll find out without too much difficulty.)

## Build Release

### Using electron-builder (recommended)

	-Run “npm run builder-release”. The builded installer & program will then be available in the electron-builder-release directory.

### Using electron-packager

	-Run “npm run packager-release”. The builded program will then be available in the bulker-[distrib]-[x64/x86] directory.

## Known Errors

### The SUID sandbox helper binary was found, but is not configured correctly. [...]

The fix is pretty simple:

	sudo chown root /path/to/app/bulker/node_modules/electron/dist/chrome-sandbox
	sudo chmod 4755 /path/to/app/bulker/node_modules/electron/dist/chrome-sandbox

### Class FIFinderSyncExtensionHost is implemented in both [...] and [...]

Nothing can be done about this, it's an harmless Apple problem.

### ERROR:gles2_cmd_decoder.cc

Errors that look like these are not fixed currently but do not directly affect the user experience:

	[52236:0405/125537.537945:ERROR:gles2_cmd_decoder.cc(18903)] [.DisplayCompositor]GL ERROR :GL_INVALID_OPERATION : DoCreateAndTexStorage2DSharedImageINTERNAL: invalid mailbox name
	[52236:0405/125537.537992:ERROR:gles2_cmd_decoder.cc(18924)] [.DisplayCompositor]GL ERROR :GL_INVALID_OPERATION : DoBeginSharedImageAccessCHROMIUM: bound texture is not a shared image
	[52236:0405/125537.538025:ERROR:gles2_cmd_decoder.cc(13814)] [.DisplayCompositor]GL ERROR :GL_INVALID_VALUE : glScheduleCALayerCHROMIUM: unsupported texture format
	[52236:0405/125537.548806:ERROR:gles2_cmd_decoder.cc(18953)] [.DisplayCompositor]GL ERROR :GL_INVALID_OPERATION : DoEndSharedImageAccessCHROMIUM: bound texture is not a shared image

## Development

I'm not a "professional" developer, I do this for fun and for my personal learning. Any suggestion is welcome, don't hesitate to open an issue if necessary.

### Milestones

* GUI
  + [x] Login
  + [x] Register
  + [x] Files explorer
  + [x] Files options
  + [x] Files uploader
  + [x] Files downloader
  + [x] Files drag&drop
  + [x] User settings
  + [x] Toolbar
  + [ ] Tooltip on hover
* Functionalities
  + [x] Upload file
  + [ ] Move file
  + [x] Download file
  + [x] Delete file
  + [x] Encrypt file
  + [x] Decrypt file
  + [ ] Create folder
  + [ ] Automatic files synchronization
  + [x] Get clouds disk space and usage
* API integrations
  + [x] Dropbox
  + [ ] Google Drive
  + [ ] One Drive
  + [ ] FTP

### TODO

	-optimize (replace data copy by reference, etc)
	-make the GUI look nicer
	-refactor
	-handle & recover error if possible
	-move setting button to an other place
	-logout button
	-default profile
	-autologin
	-add file moving
	-add a "select every file in directory" button
	-add loading screen
	-improve user security
	-improve sending of large files
	-etc.

## License

Do not share without the proper LICENSE file.
