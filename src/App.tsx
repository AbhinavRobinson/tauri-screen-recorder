import React from 'react'
import './App.css'

import { invoke } from '@tauri-apps/api/tauri'

function App() {
  const [recording, setRecording] = React.useState(false)

  const startRecording = () => {
    invoke('start', { capture: 'fullscreen' }).then((message) => {
      if (message == '200') {
        setRecording(true)
      } else if (message == '400') {
        alert('Error: Already recording')
      } else {
        alert("Error: Couldn't starting recording")
      }
    })
  }

  const stopRecording = () => {
    invoke('stop').then((message) => {
      if (message == '200') {
        setRecording(false)
      } else {
        alert('Error: Unknown Error while Stopping recording')
      }
    })
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
        {/* <div className='group'> */}
        <button
          id='startBtn'
          className='button primary'
          onClick={() => startRecording()}
        >
          ⏺ Start
        </button>
        <button
          id='stopBtn'
          className='button warning'
          onClick={() => stopRecording()}
        >
          ⏸ Stop
        </button>
        {/* </div> */}
        {/* LETS TRY WITH JUST FULLSCREEN BEFORE THIS */}
        {/* <button id='videoSelectBtn' className='button text'>
          Choose a Video Source
        </button> */}
      </div>
    </div>
  )
}

export default App
