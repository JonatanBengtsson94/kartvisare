import './App.css'
import MapViewer from './components/MapViewer'
import WmsTreeView from './components/WmsTreeView.tsx'

function App(): React.FC {

  return (
    <>
      <WmsTreeView />
      <MapViewer />
    </>
  )
}

export default App
