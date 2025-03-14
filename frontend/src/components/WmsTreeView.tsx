import { useEffect, useState } from 'react';
import { WmsGroup } from '../types/wmsTypes.ts';
import WmsGroupNode from './WmsGroupNode.tsx'


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

  return (
    <div className="wmsTreeContainer">
      <ul>
        {wmsGroups.map((group) => (
          <WmsGroupNode key={group.id} group={group} />
        ))}
      </ul>
    </div>
  );
};


export default WmsTreeView;
