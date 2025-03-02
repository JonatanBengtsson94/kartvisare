import { useEffect, useRef, useState } from 'react';
import { Map, View } from 'ol/';
import { Tile } from 'ol/layer'
import { TileWMS } from 'ol/source';
import { WmsLayer } from '../types/MapTypes'
import './MapViewer.css'

function MapViewer(): React.FC {
  const mapElement = useRef<HTMLDivElement | null>(null);
  const [wmsLayer, setWmsLayer] = useState<wmsLayer | null>(null);

  useEffect(() => {
    const fetchLayers = async () => {
      try {
        const response = await fetch('http://localhost:8080/layers');
        const data = await response.json();

        if (Array.isArray(data) && data.length > 0) {
          setWmsLayer(data[0])
        }
        } catch (error) {
        } 
  };

    fetchLayers();
  }, []);

  useEffect(() => {
    if (mapElement.current && wmsLayer) {
      const { url, params } = wmsLayer;

      const wmsSource = new TileWMS({
        url: url,
        params: {
          LAYERS: params.LAYERS,
          VERSION: params.VERSION,
          FORMAT: params.FORMAT,
          SRS: params.SRS,
        }
      });

      const wmsLayerObj = new Tile({
        source: wmsSource,
      });

      const map = new Map({
        target: mapElement.current,
        layers: [wmsLayerObj],
        view: new View({
          center: [0, 0],
          zoom: 2
        }),
      });

      return () => {
        map.setTarget(null);
      };
    }
  }, [wmsLayer]);;



  
  return (
   <div className="mapElement" ref={mapElement} /> 
  )
};

export default MapViewer;
