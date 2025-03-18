import { useEffect, useState } from "react";
import { WmsGroup, Wms } from "../types/wmsTypes.ts";
import WmsTreeView from "./WmsTreeView.tsx";
import Canvas from "./Canvas.tsx";

function MapViewer(): React.FC {
  const [selectedWmsIds, setSelectedWmsIds] = useState<number[]>([]);
  const [selectedWms, setSelectedWms] = useState<Wms[]>([]);
  const [wmsGroups, setWmsGroups] = useState<WmsGroup[]>([]);
  const [expandedGroupIds, setExpandedGroupIds] = useState<number[]>([]);

  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const wmsGroupApiUrl = import.meta.env.VITE_API_BASEURL + "/wms_groups";
  const wmsApiUrl = import.meta.env.VITE_API_BASEURL + "/wms";

  useEffect(() => {
    fetch(wmsGroupApiUrl)
      .then((response) => {
        if (!response.ok) {
          throw new Error("Failed to fetch WMS list");
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

  useEffect(() => {
    const idsToFetch = selectedWmsIds.filter(
      (id) => !selectedWms.some((wms) => wms.id === id),
    );

    if (idsToFetch.length > 0) {
      setLoading(true);
      setError(null);
      Promise.all(
        idsToFetch.map((id) =>
          fetch(`${wmsApiUrl}/${id}`)
            .then((response) => response.json())
            .catch((error) => {
              throw new Error("Failed to fetch wms with ID ${id}");
            }),
        ),
      )
        .then((newWmsLayers: Wms[]) => {
          setSelectedWms((prevSelectedWms) => [
            ...prevSelectedWms,
            ...newWmsLayers,
          ]);
          setLoading(false);
        })
        .catch((error) => {
          setError(error.message);
          setLoading(false);
        });
    } else {
      setSelectedWms(
        selectedWmsIds.map(
          (id) => selectedWms.find((wms) => wms.id === id) as Wms,
        ),
      );
      setLoading(false);
    }
  }, [selectedWmsIds]);

  const handleWmsChange = (checked: boolean, wmsId: number) => {
    if (checked) {
      setSelectedWmsIds((prevSelected) => [...prevSelected, wmsId]);
    } else {
      setSelectedWmsIds((prevSelected) =>
        prevSelected.filter((id) => id !== wmsId),
      );
    }
  };

  const toggleGroupExpansion = (groupId: number) => {
    setExpandedGroupIds((prevExpanded) => {
      if (prevExpanded.includes(groupId)) {
        return prevExpanded.filter((id) => id !== groupId);
      } else {
        return [...prevExpanded, groupId];
      }
    });
  };

  if (loading) {
    return <p>Loading available WMS...</p>;
  }

  if (error) {
    return <p>Error: {error}</p>;
  }

  return (
    <>
      <WmsTreeView
        groups={wmsGroups}
        onWmsChange={handleWmsChange}
        checkedWms={selectedWmsIds}
        expandedGroups={expandedGroupIds}
        toggleGroupExpansion={toggleGroupExpansion}
      />
      <Canvas wms={selectedWms} />
    </>
  );
}

export default MapViewer;
