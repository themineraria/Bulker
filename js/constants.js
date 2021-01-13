const { shell, remote, ipcRenderer } = require('electron');
const BrowserWindow = remote.BrowserWindow;
const remoteapp = remote.app;
const TOML = require('@iarna/toml');
const fs = require("fs");
var AnyPromise = require('promise.any'); //AnyPromise is a const implementing Promise.any()
