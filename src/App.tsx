import React from 'react'
import './App.css'

function App() {
  const [recording, setRecording] = React.useState(false)

  const startRecording = () => {
    setRecording(true)
  }

  const stopRecording = () => {
    setRecording(false)
  }

  return (
    <div className='content'>
      <h1>
        <span className={recording ? 'recording' : ''}>
          {recording ? '🔴' : '⚫️'}{' '}
        </span>
        Tauri Screen Recorder
      </h1>

      <video></video>

      <hr />

      <div className='actions'>
        <div className='group'>
          <button
            id='startBtn'
            className='button primary'
            onClick={() => startRecording()}
          >
            Start
          </button>
          <button
            id='stopBtn'
            className='button warning'
            onClick={() => stopRecording()}
          >
            Stop
          </button>
        </div>
        <button id='videoSelectBtn' className='button text'>
          Choose a Video Source
        </button>
      </div>
    </div>
  )
}

export default App
