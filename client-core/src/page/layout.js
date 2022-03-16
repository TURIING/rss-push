import { SideBar } from "./sidebar";
import { Login } from "../component/login";
import { LoginInfoContext } from "../context";
import { useContext } from "react";
import { Content } from "./content"

export function Layout() {
    const login_info = useContext(LoginInfoContext);
    const is_login = login_info.login_info["is_login"];
    
    return(
        <div>
            <SideBar />
            { !is_login && <Login />}
            <Content />
        </div>
    );
}
