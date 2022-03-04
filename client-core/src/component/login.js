import * as React from "react";
import "../css/login.css";
import { LoginInfoContext } from "../context";

import { Modal, Button, Form } from '@douyinfe/semi-ui';
import { IconVigoLogo, IconSemiLogo } from '@douyinfe/semi-icons';

export class Login extends React.Component {

    static contextType = LoginInfoContext;

    constructor(props, context) {
        super(props, context);
        this.state = {page: true};

    }

    // control to show login or register page
    changePage = () => {
        console.log("change");
        this.setState({
            page: !this.state.page
        })
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
                <Form>
                    <p style={style.text}>欢迎使用RSS-PUSH</p>
                    <Input 
                        placeholder="请输入账号"
                        style={style.inputAccount}
                        noLabel={true}
                    />
                    <Input
                        placeholder="请输入密码"
                        type="password"
                        style={style.inputPasswd}
                    />
                    <Button
                        type="primary" 
                        theme="solid" 
                        onClick={this.changePage} 
                        style={style.buttonLogin}>
                        登录
                    </Button>
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
                        placeholder="请输入账号"
                        style={style.inputAccount}
                        noLabel={true}
                    />
                    <Input
                        placeholder="请输入密码"
                        type="password"
                        style={style.inputRegPasswd}
                    />
                    <Input
                        placeholder="请再次输入密码"
                        type="password"
                        style={style.inputRegPasswdAgain}
                    />
                    <Button
                        type="primary" 
                        theme="solid" 
                        onClick={this.changePage} 
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
                
                <Modal
                    header={null}
                    visible={true}
                    onOk={this.changePage}
                    onCancel={this.handleCancel}
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
