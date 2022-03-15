import * as React from "react";

export const LoginInfoContext = React.createContext({
    login_info: {
        is_login: false,
        username: null,
        token: null
    },
    setLoginInfo: ()=>{}
})