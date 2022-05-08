import './App.css'

function App() {
  return (
    <div className='content'>
      <h1>⚡ Tauri Screen Recorder</h1>

      <video></video>

      <button id='startBtn' className='button is-primary'>
        Start
      </button>
      <button id='stopBtn' className='button is-warning'>
        Stop
      </button>

      <hr />

      <button id='videoSelectBtn' className='button is-text'>
        Choose a Video Source
      </button>
    </div>
  )
}

export default App