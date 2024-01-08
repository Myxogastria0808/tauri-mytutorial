import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const executeCommands = () => {
    invoke("simple_command");
  };

  return (
    <React.Fragment>
      <button onClick={executeCommands}>Click here!</button>
    </React.Fragment>
  );
}

export default App;
