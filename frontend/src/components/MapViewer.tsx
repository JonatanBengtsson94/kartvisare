import { useEffect, useRef, useState } from 'react';
import { Map, View } from 'ol/';
import { Tile } from 'ol/layer'
import { TileWMS } from 'ol/source';
import './MapViewer.css'

function MapViewer(): React.FC {
  const mapElement = useRef<HTMLDivElement | null>(null);
  const [mapUrl, setMapUrl] = useState<string | null>(null);

  useEffect(() => {
    const fetchLayers = async () => {
      try {
        const response = await fetch('http://localhost:8080/layers');
        const data = await response.json();

        if (Array.isArray(data) && data.length > 0) {
          setMapUrl(data[0].url)
        }
        } catch (error) {
        } 
  };

    fetchLayers();
  }, []);

  useEffect(() => {
    if (mapElement.current && mapUrl) {
      const wmsSource = new TileWMS({
        url: mapUrl,
        params: {
          LAYERS: "topp:states",
          VERSION: "1.1.1",
          FORMAT: "image/png",
          SRS: "EPSG:4326",
        }
      });

      const wmsLayer = new Tile({
        source: wmsSource,
      });

      const map = new Map({
        target: mapElement.current,
        layers: [wmsLayer],
        view: new View({
          center: [0, 0],
          zoom: 2
        }),
      });

      return () => {
        map.setTarget(null);
      };
    }
  }, [mapUrl]);;



  
  return (
   <div className="mapElement" ref={mapElement} /> 
  )
};

export default MapViewer;
