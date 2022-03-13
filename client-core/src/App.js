import * as React from "react";
import { Routes, Route, Outlet, Link } from "react-router-dom";
import { Button } from '@douyinfe/semi-ui';
import { Config } from "./utility";
import { CFG_DIR } from "./variable";
import { LoginInfoContext } from "./context";
import { Layout } from "./page/layout";


export default class App extends React.Component {
	constructor(props) {
		super(props);
		Config.init();
		const cfg_ctx = JSON.parse(Config.readAll());
		this.state = {
			login_info:{
				login_info: {
        			is_login: cfg_ctx["is_login"],
        			username: cfg_ctx["username"],
				},
				setLoginInfo: this.setLoginInfo
			}
		}
	
	}
	render() {
		return (
			<div>
				<LoginInfoContext.Provider value={this.state.login_info}>
					<Routes>
						<Route path="/" element={<Layout/>}>

						</Route>
					</Routes>
				</LoginInfoContext.Provider>
			</div>
		)
	}

	// update the LoginInfo context
	setLoginInfo = (info) => {
		this.setState({
			login_info: {
				login_info: info,
				setLoginInfo: this.setLoginInfo
			}
		})

		let content = {
			is_login: info.is_login,
			username: info.username,
		};
		Config.writeAll(content);
	}
}

// export default function App() {
//   return (
//     <div>
//       <Routes>
//         <Route path="/" element={<Layout />}>
//           <Route index element={<Home />} />
//           <Route path="about" element={<About />} />
//           <Route path="dashboard" element={<Dashboard />} />

//           <Route path="*" element={<NoMatch />} />
//         </Route>
//       </Routes>
//     </div>
//   );
// }

// function Layout() {
//   return (
//     <div>
//       {/* A "layout route" is a good place to put markup you want to
//           share across all the pages on your site, like navigation. */}
//       <nav>
//         <ul>
//           <li>
//             <Link to="/">Home</Link>
//           </li>
//           <li>
//             <Link to="/about">About</Link>
//           </li>
//           <li>
//             <Link to="/dashboard">Dashboard</Link>
//           </li>
//           <li>
//             <Link to="/nothing-here">Nothing Here</Link>
//           </li>
//         </ul>
//       </nav>
//       <Button theme='solid' type='primary' style={{ marginRight: 8 }}>深色主要</Button>
//       <hr />

//       {/* An <Outlet> renders whatever child route is currently active,
//           so you can think about this <Outlet> as a placeholder for
//           the child routes we defined above. */}
//       <Outlet />
//     </div>
//   );
// }

// function Home() {
//   return (
//     <div>
//       <h2>Home</h2>
//     </div>
//   );
// }

// function About() {
//   return (
//     <div>
//       <h2>About</h2>
//     </div>
//   );
// }

// function Dashboard() {
//   return (
//     <div>
//       <h2>Dashboard</h2>
//     </div>
//   );
// }

// function NoMatch() {
//   return (
//     <div>
//       <h2>Nothing to see here!</h2>
//       <p>
//         <Link to="/">Go to the home page</Link>
//       </p>
//     </div>
//   );
// }