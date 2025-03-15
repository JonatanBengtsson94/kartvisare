import './WmsLayerControl.css'

import { useState } from 'react';
import { Wms } from '../types/wmsTypes.ts';

interface WmsLayerControlProps {
  wms: Wms;
  onChange: (checked: boolean, wmsId: number) => void;
}

const WmsLayerControl: React.FC<WmsLayerControlProps> = ({ wms, onChange }) => {
  const [isChecked, setIsChecked] = useState<boolean>(false);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const checked = e.target.checked;
    setIsChecked(checked);
    onChange(checked, wms.id)
  }

  return (
  <div className="wmsLayerControl">
    <input
      type="checkbox"
      checked={isChecked}
      onChange={handleChange}
      id={wms.id}
    />
    <label htmlFor={wms.id}>{wms.name}</label>
  </div>
  )
}

export default WmsLayerControl;
