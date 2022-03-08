import "../css/side_bar.css";
import { Button } from '@douyinfe/semi-ui';
import { RssSearch } from "../component/rss_search";

export function SideBar() {
    return (
        <div className="side_bar">
            <RssSearch />
        </div>
    );
}