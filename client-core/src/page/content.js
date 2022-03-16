
import * as React from "react";
import ReactHtmlParser from 'react-html-parser';
import "../css/content.css"
 
export class Content extends React.Component {
    render() {
        const html = '<p>Example HTML string</p>';
        const style = {
            outer: {
                position: 'fixed',
                top: 0,
                left: 261,
                height: 560,
                width: 580,
                padding: '80px 30px 80px'

            }
        }
        return(
            <div className="outer" style={style.outer}>
                { ReactHtmlParser(html) }

                
                

            </div>
        );
    }
}