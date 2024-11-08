import React from "react";
import "./App.css";

import { invoke } from "@tauri-apps/api/core";

function App() {
  const [recording, setRecording] = React.useState(false);

  const startRecording = () => {
    setRecording(true);
    invoke("start", { capture: "fullscreen" }).then((message) => {
      if (message == "200") {
        stopRecording();
      } else if (message == "400") {
        alert("Error: Already recording");
      } else {
        alert("Error: Couldn't starting recording");
      }
    });
  };

  const stopRecording = () => {
    setRecording(false);
  };

  return (
    <div className="content">
      <h1>
        <span className={recording ? "recording" : ""}>
          {recording ? "🔴" : "⚫️"}{" "}
        </span>
        Tauri Screen Recorder
      </h1>

      <video></video>

      <hr />

      <div className="actions">
        <button
          id="startBtn"
          className="button primary"
          onClick={() => startRecording()}
        >
          ⏺ Start
        </button>
        <button
          id="stopBtn"
          className="button warning"
          onClick={() => stopRecording()}
        >
          ⏸ Stop
        </button>
      </div>
    </div>
  );
}

export default App;
