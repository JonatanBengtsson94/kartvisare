import { useEffect, useRef } from 'react';
import { Map, View } from 'ol/';
import { Tile } from 'ol/layer'
import { OSM } from 'ol/source';
import './MapViewer.css'

function MapViewer(): React.FC {
  const mapElement = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (mapElement.current) {
      const osmLayer = new Tile({
        source: new OSM(),
      });

    new Map({
      target: mapElement.current,
      layers: [osmLayer],
      view: new View({
        center: [0, 0],
        zoom: 2,
      }),
    });
  }
  
  return () => {
    mapElement.current = null;
  };
}, []);

  return (
   <div className="mapElement" ref={mapElement} /> 
  )
};

export default MapViewer;
