import { useState } from 'react';
import { WmsGroup } from '../types/wmsTypes.ts';

interface WmsGroupNodeProps {
  group: WmsGroup;
}

const WmsGroupNode: React.FC<WmsGroupNodeProps> = ( { group }) => {
  const [isExpanded, setIsExpanded] = useState<boolean>(false);

  const toggleExpand = () => {
    setIsExpanded(!isExpanded);
  };

  return (
    <li>
      <button onClick={toggleExpand}>{group.name}</button>
      {isExpanded && (
        <ul>
          {group.sub_groups && group.sub_groups.length > 0 && (
            <ul>
              {group.sub_groups.map((subGroup) => (
                <WmsGroupNode key={subGroup.id} group={subGroup} />
              ))}
            </ul>
          )}
          {group.wms && group.wms.length > 0 && (
            <ul>
              {group.wms.map((wms) => {
                <li key={wms.id}>{wms.name}</li>
              })}
            </ul>
          )}
        </ul>
      )}
    </li>
  );
}

export default WmsGroupNode;
