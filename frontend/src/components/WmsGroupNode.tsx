import './WmsGroupNode.css'

import { WmsGroup } from '../types/wmsTypes.ts';
import WmsLayerControl from './WmsLayerControl.tsx';

interface WmsGroupNodeProps {
  group: WmsGroup;
  level: number;
  toggleExpand: (groupId: number) => void;
  onWmsChange: (checked: boolean, wmsId: number) => void;
  checkedWms: number[];
  expandedGroups: number[];
}

const WmsGroupNode: React.FC<WmsGroupNodeProps> = ({ 
  group,
  level,
  toggleExpand,
  onWmsChange,
  checkedWms,
  expandedGroups,
}) => {
  const marginLeft = `${level * 20}px`;

  return (
    <li>
      <button className="groupNodeButton" onClick={() => toggleExpand(group.id)}>
        <span
          className={`arrow ${expandedGroups.includes(group.id) ? 'expanded' : ''}`}
          style={{ marginLeft }}>
        </span>
        <span
          className="groupName" 
          style={{ marginLeft }}>
          {group.name}
        </span>
      </button>
      {expandedGroups.includes(group.id) && (
        <ul>
          {group.sub_groups && group.sub_groups.length > 0 && (
            <ul>
              {group.sub_groups.map((subGroup) => (
                <WmsGroupNode
                  key={subGroup.id}
                  group={subGroup}
                  level={level + 1}
                  toggleExpand={toggleExpand}
                  onWmsChange={onWmsChange}
                  checkedWms={checkedWms}
                  expandedGroups={expandedGroups}
                />
              ))}
            </ul>
          )}
          {group.wms && group.wms.length > 0 && (
            <ul className="wmsList">
              {group.wms.map((wms) => {
                return <li key={wms.id} style={{ marginLeft }}>
                  <WmsLayerControl
                    wms={wms} 
                    onChange={onWmsChange}
                    checked={checkedWms.includes(wms.id)}
                  />
                </li>
              })}
            </ul>
          )}
        </ul>
      )}
    </li>
  );
}

export default WmsGroupNode;
