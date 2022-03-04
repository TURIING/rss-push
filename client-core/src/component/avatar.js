import * as React from "react";
import {LoginInfoContext} from "../context"

export class Avatar extends React.Component {
    render() {
      return (
        <LoginInfoContext.Consumer>
          {({ login_info, setLoginInfo }) => (
			  <button onClick={() => setLoginInfo({
				  is_login: true,
				  username: "turiing"
			  })}>
              	Switch Language (Current: {login_info.username})
              </button>
            
          )}
        </LoginInfoContext.Consumer>
      );
    }
  }