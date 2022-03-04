var os = window.preload.os;

// config directary path
const CFG_DIR = os.homedir() + "/.config/rss-push/";
// confiig file path
const CFG_FILE = CFG_DIR + "config";

export {
    CFG_DIR,
    CFG_FILE
}