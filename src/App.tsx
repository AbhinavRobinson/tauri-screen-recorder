import React from "react";
import "./App.css";

import { invoke } from "@tauri-apps/api/core";

function App() {
  const [isBusy, setIsBusy] = React.useState(false);

  const screenshot = () => {
    setIsBusy(true);
    invoke("screenshot", { capture: "fullscreen" }).then((message) => {
      if (message == "200") {
        free();
      } else if (message == "400") {
        alert("Error: Already captured");
      } else {
        alert("Error: Couldn't starting capture");
      }
    });
  };

  const free = () => {
    setIsBusy(false);
  };

  return (
    <div className="content">
      <h1>
        <span className={isBusy ? "capturing" : ""}>
          {isBusy ? "🔴" : "⚫️"}{" "}
        </span>
        Tauri Screen Recorder
      </h1>
      <div className="actions">
        <button
          id="startBtn"
          className="button primary"
          onClick={() => screenshot()}
        >
          ⏺ Screenshot
        </button>
      </div>
    </div>
  );
}

export default App;
