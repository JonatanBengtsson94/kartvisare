import './WmsTreeView.css'

import { WmsGroup } from '../types/wmsTypes.ts';
import WmsGroupNode from './WmsGroupNode.tsx'

interface WmsTreeViewProps {
  group: WmsGroup;
  onWmsChange: (checked: boolean, wmsId: number) => void;
}

const WmsTreeView: React.FC<WmsTreeViewProps> = ({ group, onWmsChange}) => {
  return (
    <div className="wmsTreeContainer">
      <ul>
        {group.map((group) => (
          <WmsGroupNode
            key={group.id}
            group={group}
            level={0}
            onWmsChange={onWmsChange}
          />
        ))}
      </ul>
    </div>
  );
};


export default WmsTreeView;
