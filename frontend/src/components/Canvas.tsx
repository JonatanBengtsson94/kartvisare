import "./Canvas.css";

import { useEffect, useRef } from "react";
import { Wms } from "../types/wmsTypes.ts";
import { Map as OlMap, View } from "ol";
import TileLayer from "ol/layer/Tile";
import TileWMS from "ol/source/TileWMS";
import OSM from "ol/source/OSM";

interface CanvasProps {
  wms: Wms[];
}

const Canvas: React.FC<CanvasProps> = ({ wms }) => {
  const mapRef = useRef<HTMLDivElement>(null);
  const mapInstance = useRef<OlMap | null>(null);
  const layerInstances = useRef<Map<number, TileLayer>>(new Map());

  useEffect(() => {
    const currentLayerInstances = layerInstances.current;

    if (!mapInstance.current && mapRef.current) {
      mapInstance.current = new OlMap({
        controls: [],
        target: mapRef.current,
        layers: [],
        view: new View({
          center: [0, 0],
          zoom: 2,
        }),
      });
    }

    const currentMap = mapInstance.current;

    if (currentMap && currentMap.getLayers().getLength() === 0) {
      const baseLayer = new TileLayer({
        source: new OSM(),
      });
      currentMap.addLayer(baseLayer);
    }

    if (currentMap) {
      wms.forEach((wms) => {
        if (!layerInstances.current.has(wms.id)) {
          const tileLayer = new TileLayer({
            source: new TileWMS({
              url: wms.url,
              params: {
                LAYERS: wms.layers,
              },
            }),
          });

          currentMap.addLayer(tileLayer);
          currentLayerInstances.set(wms.id, tileLayer);
        }
      });

      layerInstances.current.forEach((layer, id) => {
        if (!wms.some((wms) => wms.id === id)) {
          currentMap.removeLayer(layer);
          currentLayerInstances.delete(id);
        }
      });
    }

    return () => {
      const currentMap = mapInstance.current;
      if (currentMap) {
        currentMap.getLayers().clear();
        currentLayerInstances.clear();
      }
    };
  }, [wms]);

  return <div className="canvas" ref={mapRef} />;
};

export default Canvas;
