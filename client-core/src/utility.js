import {CFG_DIR, CFG_FILE} from "./variable";
const fs = window.preload.fs;
const os = window.preload.os;

export class Config{
    // init the config file
    static init() {
        if (!fs.existsSync(CFG_DIR)){
            fs.mkdirSync(CFG_DIR);
        }
        if (!fs.existsSync(CFG_FILE)){
            fs.writeFileSync(CFG_FILE, JSON.stringify(cfg_ctx_default));
        }
    }
    // read config content
    static readAll() {
        return fs.readFileSync(CFG_FILE, 'utf8');
    }
}

// default config context
const cfg_ctx_default = {
    is_login: false,
    username: null,

}