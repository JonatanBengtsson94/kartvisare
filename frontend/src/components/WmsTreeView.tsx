import { useEffect, useState } from 'react';
import { WmsGroup } from '../types/wmsTypes.ts';


const WmsTreeView: React.FC = () => {
  const wmsGroupApiUrl = import.meta.env.VITE_API_BASEURL + "/wms_groups";
  const [wmsGroups, setWmsGroups] = useState<Wms[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch(wmsGroupApiUrl)
      .then((response) => {
        if (!response.ok) {
          throw new Error('Failed to fetch WMS list');
        }
        return response.json();
      })
      .then((data: Wms[]) => {
        setWmsGroups(data);
        setLoading(false);
      })
      .catch((error) => {
        setError(error.message);
        setLoading(false);
      });
  }, []);

  if (loading) {
    return <p>Loading available WMS...</p>
  }

  if (error) {
    return <p>Error: {error}</p>
  }

  const renderWmsGroup = (group: WmsGroup) => {
    return (
      <li key={group.id}>
          {group.name}
          {group.sub_groups && group.sub_groups.length > 0 && (
            <ul>
              {group.sub_groups.map((subGroup) => renderWmsGroup(subGroup))}
            </ul>
          )}
          {group.wms && group.wms.length > 0 && (
            <ul>
              {group.wms.map((wms) => (
              <li key={wms.id}>{wms.name}</li>
              ))}
            </ul>
          )}
      </li>
    );
  };

  return <ul>{wmsGroups.map((group) => renderWmsGroup(group))}</ul>
};


export default WmsTreeView;
