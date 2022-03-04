const {contextBridge} = require('electron');
// const config = require("./preload/utility.js");
const fs = require("fs");
const os = require("os");
// All of the Node.js APIs are available in the preload process.
// It has the same sandbox as a Chrome extension.
window.addEventListener('DOMContentLoaded', () => {
  const replaceText = (selector, text) => {
    const element = document.getElementById(selector)
    if (element) element.innerText = text
  }

  for (const type of ['chrome', 'node', 'electron']) {
    replaceText(`${type}-version`, process.versions[type])
  }
})

// contextBridge.exposeInMainWorld('utility', {
// 	config,
// })
contextBridge.exposeInMainWorld('preload', {
	fs: {
		readFileSync: fs.readFileSync,
		writeFileSync: fs.writeFileSync,
		existsSync: fs.existsSync,
		mkdirSync: fs.mkdirSync,

	},
	os: {
		homedir: os.homedir
	}
})