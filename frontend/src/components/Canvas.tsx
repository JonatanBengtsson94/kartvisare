import { useRef } from 'react';

const Canvas: React.FC = () => {
  const mapRef = useRef<HTMLDivElement>(null);

  return <div className="canvas" ref={mapRef} />
}

export default Canvas;
