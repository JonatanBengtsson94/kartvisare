import { useEffect, useState } from 'react';

interface Wms {
  id: number,
  name: string,
}

const WmsListComponent: React.FC = () => {
  const [wmsList, setWmsList] = useState<Wms[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetch('http://localhost:3000/wms')
      .then((response) => {
        if (!response.ok) {
          throw new Error('Failed to fetch WMS list');
        }
        return response.json();
      })
      .then((data: Wms[]) => {
        setWmsList(data);
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
    <ul>
      {wmsList.map((wms) => (
      <li key={wms.id}>{wms.name}</li>
      ))}
    </ul>
  )
}


export default WmsListComponent;
