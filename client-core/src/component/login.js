import * as React from "react";
import "../css/login.css";
import { LoginInfoContext } from "../context";
import { SERVER_HOST } from "../variable";
import axios from 'axios';
import { Modal, Button, Form } from '@douyinfe/semi-ui';

export class Login extends React.Component {

    static contextType = LoginInfoContext;

    constructor(props, context) {
        super(props, context);
        this.state = {page: true, visible: true};

    }

    // control to show login or register page
    changePage = () => {
        this.setState({
            page: !this.state.page
        })
    }

    // close the modal
    closeModal = () => {
        this.setState({
            visible: false
        })
    }

    // register
    register = () => {
        
        const url = SERVER_HOST + "api/register";
        const formState = this.formApi.getFormState()['values'];
        let data = {
            username: formState['username'],
            passwd: formState['passwd']
        }
        axios({
            method: 'post',
            url: url,
            data: JSON.stringify(data),
            headers: {'Content-Type': 'application/json'}
        }).then(d => console.log(d.data))        
    }

    // login
    login = (setLoginInfo) => {
        const url = SERVER_HOST + "api/auth/login";
        const formState = this.formApi.getFormState()['values'];
        let data = {
            username: formState['username'],
            passwd: formState['passwd']
        }
        axios({
            method: 'post',
            url: url,
            data: JSON.stringify(data),
            headers: {'Content-Type': 'application/json'}
        }).then(response => {
            const code = response.data['status'];
            const token = response.data['token'];
            if (code == 201) {
                this.closeModal();
                let info = { is_login: true, username: data.username, token: token};
                setLoginInfo(info);
            }
        })     
    }

    getFormApi = (formApi) => {
        this.formApi = formApi;
    }
    
    render() {
        const { Input, TextArea } = Form;
        
        const style = {
            inputAccount: {
                width: '300px',
                height: '40px',
                paddingTop: 4,
                border: '1px solid #BDBDC3',
                backgroundColor: '#F1F1F2',
                position: 'fixed',
                top: '110px',
                left: '50px'
            },
            inputPasswd: {
                width: '300px',
                height: '40px',
                paddingTop: 4,
                border: '1px solid #BDBDC3',
                backgroundColor: '#F1F1F2',
                position: 'fixed',
                top: '190px',
                left: '50px'
            },
            buttonLogin: {
                width: '300px',
                height: '40px',
                position: 'fixed',
                top: '270px',
                left: '50px',
                margin: 0
            },
            modal: {
                width: '400px',
                height: '380px',
                WebkitAppRegion: 'no-drag'
            },
            text: {
                fontSize: 18,
                //fontWeight: 'bold',
                position: 'fixed',
                top: '50px',
                left: '110px',
            },
            inputRegPasswd: {
                width: '300px',
                height: '40px',
                paddingTop: 4,
                border: '1px solid #BDBDC3',
                backgroundColor: '#F1F1F2',
                position: 'fixed',
                top: '170px',
                left: '50px'
            },
            inputRegPasswdAgain: {
                width: '300px',
                height: '40px',
                paddingTop: 4,
                border: '1px solid #BDBDC3',
                backgroundColor: '#F1F1F2',
                position: 'fixed',
                top: '230px',
                left: '50px'
            },
            buttonRegister: {
                width: '300px',
                height: '40px',
                position: 'fixed',
                top: '290px',
                left: '50px',
                margin: 0
            },
            buttonReturn: {
                position: 'fixed',
                top: '340px',
                left: '45px',
                margin: 0
            },
            buttonChange: {
                position: 'fixed',
                top: '330px',
                left: '39px',
                margin: 0
            }
        }
        
        let page;
        if (this.state.page) {
            page = (
                <Form getFormApi={this.getFormApi}>
                    
                    <p style={style.text}>欢迎使用RSS-PUSH</p>
                    <Input
                        field="username"
                        placeholder="请输入账号"
                        style={style.inputAccount}
                        noLabel={true}
                    />
                    <Input
                        field="passwd"
                        placeholder="请输入密码"
                        type="password"
                        style={style.inputPasswd}
                        noLabel={true}
                    />
                    <LoginInfoContext.Consumer>
                        {({login_info, setLoginInfo}) => (
                            <Button
                                type="primary" 
                                theme="solid" 
                                onClick={() => {this.login(setLoginInfo)}} 
                                style={style.buttonLogin}>
                                登录
                            </Button>
                        )}
                    </LoginInfoContext.Consumer>                    
                    <Button type="primary" theme="borderless" onClick={this.changePage} style={style.buttonChange}>
                        没有账号？
                    </Button>
                </Form>
            )
        }else {
            page = (
                <Form>
                    <p style={style.text}>欢迎使用RSS-PUSH</p>
                    <Input 
                        field="username"
                        placeholder="请输入账号"
                        style={style.inputAccount}
                        noLabel={true}
                    />
                    <Input
                        field="passwd"
                        placeholder="请输入密码"
                        type="password"
                        style={style.inputRegPasswd}
                        noLabel={true}
                    />
                    <Input
                        field="passwd_again"
                        placeholder="请再次输入密码"
                        type="password"
                        style={style.inputRegPasswdAgain}
                        noLabel={true}
                    />

                    <Button
                        type="primary" 
                        theme="solid" 
                        onClick={this.register} 
                        style={style.buttonRegister}>
                        注册
                    </Button>
                    <Button type="primary" theme="borderless" onClick={this.changePage} style={style.buttonReturn}>
                        返回
                    </Button>
                </Form>
            )
        }
        
        return (
            <>
                {/* <ThemeContext.Consumer>
                {({theme, toggleTheme}) => (
                    <button
                    onClick={toggleTheme}
                    style={{backgroundColor: theme.background}}>
                    Toggle Theme
                    </button>
                )}
                </ThemeContext.Consumer>  */}
                
                <Modal
                    header={null}
                    visible={this.state.visible}
                    onOk={this.changePage}
                    footer={page}
                    maskClosable={false}
                    style={style.modal}
                    centered={true}
                >
                    
                </Modal>
            </>
        );
    }
}
