import { useEffect, useState } from 'react';
import { WmsGroup } from '../types/wmsTypes.ts';
import WmsTreeView from './WmsTreeView.tsx';
import Canvas from './Canvas.tsx';

function MapViewer(): React.FC {
  const [selectedWms, setSelectedWms] = useState<number[]>([]);
  const wmsGroupApiUrl = import.meta.env.VITE_API_BASEURL + "/wms_groups";
  const [wmsGroups, setWmsGroups] = useState<WmsGroup[]>([]);
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
      .then((data: WmsGroup[]) => {
        setWmsGroups(data);
        setLoading(false);
      })
      .catch((error) => {
        setError(error.message);
        setLoading(false);
      });
  }, []);

  const handleWmsChange = (checked: boolean, wmsId: number) => {
    if (checked) {
      setSelectedWms((prevSelected) => [...prevSelected, wmsId]);
    } else {
      setSelectedWms((prevSelected) => prevSelected.filter(id => id !== wmsId));
    }
    console.log('Selected layers:', selectedWms);
  };

  if (loading) {
    return <p>Loading available WMS...</p>
  }

  if (error) {
    return <p>Error: {error}</p>
  }

  return (
    <>
    <WmsTreeView
      group={wmsGroups}
      onWmsChange={handleWmsChange}
    />
    <Canvas
      wms={selectedWms}
    />
    </>
  );
};

export default MapViewer;
