var os = window.preload.os;

// config directary path
const CFG_DIR = os.homedir() + "/.config/rss-push/";
// confiig file path
const CFG_FILE = CFG_DIR + "config";

const SERVER_HOST = "http://127.0.0.1:8000/"

export {
    CFG_DIR,
    CFG_FILE,
    SERVER_HOST
}