import "./WmsTreeView.css";

import { WmsGroup } from "../types/wmsTypes.ts";
import WmsGroupNode from "./WmsGroupNode.tsx";

interface WmsTreeViewProps {
  groups: WmsGroup[];
  onWmsChange: (checked: boolean, wmsId: number) => void;
  checkedWms: number[];
  expandedGroups: number[];
  toggleGroupExpansion: (groupId: number) => void;
}

const WmsTreeView: React.FC<WmsTreeViewProps> = ({
  groups,
  onWmsChange,
  checkedWms,
  expandedGroups,
  toggleGroupExpansion,
}) => {
  return (
    <div className="wmsTreeContainer">
      <ul>
        {groups.map((group) => (
          <WmsGroupNode
            key={group.id}
            group={group}
            level={0}
            toggleExpand={toggleGroupExpansion}
            onWmsChange={onWmsChange}
            checkedWms={checkedWms}
            expandedGroups={expandedGroups}
          />
        ))}
      </ul>
    </div>
  );
};

export default WmsTreeView;
