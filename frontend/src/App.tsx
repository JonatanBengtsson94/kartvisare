import './App.css'
import MapViewer from './components/MapViewer'
import WmsListComponent from './components/WmsListComponent.tsx'

function App(): React.FC {

  return (
    <>
      <WmsListComponent />
      <MapViewer />
    </>
  )
}

export default App
