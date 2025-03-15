import './WmsGroupNode.css'

import { useState } from 'react';
import { WmsGroup } from '../types/wmsTypes.ts';
import WmsLayerControl from './WmsLayerControl.tsx';

interface WmsGroupNodeProps {
  group: WmsGroup;
  level: number;
  onWmsChange: (checked: boolean, wmsId: number) => void;
}

const WmsGroupNode: React.FC<WmsGroupNodeProps> = ({ group, level, onWmsChange }) => {
  const [isExpanded, setIsExpanded] = useState<boolean>(false);

  const toggleExpand = () => {
    setIsExpanded(!isExpanded);
  };

  const marginLeft = `${level * 20}px`;

  return (
    <li>
      <button className="groupNodeButton" onClick={toggleExpand}>
        <span
          className={`arrow ${isExpanded ? 'expanded' : ''}`}
          style={{ marginLeft }}>
        </span>
        <span
          className="groupName" 
          style={{ marginLeft }}>
          {group.name}
        </span>
      </button>
      {isExpanded && (
        <ul>
          {group.sub_groups && group.sub_groups.length > 0 && (
            <ul>
              {group.sub_groups.map((subGroup) => (
                <WmsGroupNode
                  key={subGroup.id}
                  group={subGroup}
                  level={level + 1}
                  onWmsChange={onWmsChange}
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
