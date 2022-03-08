import * as React from 'react';
import { LoginInfoContext } from "../context";
import { Modal, Button, Input} from '@douyinfe/semi-ui';
import { IconPlus, IconClose } from '@douyinfe/semi-icons';


export class RssSearch extends React.Component {

    static contextType = LoginInfoContext;

    constructor(props, context) {
        super(props, context);
        this.state = { visible: false}

    }

    // open or close the modal
    changeVisible = () => {
        this.setState({
            visible: !this.state.visible
        });
    }

    render() {
        const style = {
            modal: {
                width: '400px',
                height: '380px',
                WebkitAppRegion: 'no-drag'
            },
            button_add: {
                position: 'fixed',
                top: 30,
                left: 40,
                margin: 0
            }
        };
        let page;
        page = (
            <Input placeholder='feed url'></Input>
        );
        return(
            <>
                <Button 
                    type="primary" 
                    theme="borderless"
                    onClick={this.changeVisible} 
                    style={style.button_add} 
                    icon={<IconPlus />}
                >

                </Button>
                <Modal
                    //header={null}
                    visible={this.state.visible}
                    onOk={this.changePage}
                    footer={page}
                    maskClosable={false}
                    style={style.modal}
                    centered={true}
                    closable={true}
                    closeIcon={<IconClose />}
                    onCancel={this.changeVisible}
                >
                </Modal>
            </>
        )
    }
}