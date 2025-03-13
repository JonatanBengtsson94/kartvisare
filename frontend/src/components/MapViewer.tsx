import { useEffect, useRef, useState } from 'react';
import { Map, View } from 'ol/';
import { Tile } from 'ol/layer'
import { TileWMS } from 'ol/source';
import './MapViewer.css'

function MapViewer(): React.FC {
  const mapElement = useRef<HTMLDivElement | null>(null);

  
  return (
   <div className="mapElement" ref={mapElement} /> 
  )
};

export default MapViewer;
