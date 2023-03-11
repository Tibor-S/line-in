import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { listen } from '@tauri-apps/api/event'

type trackDataT = {
  track: string
  artist: string
  coverart: string
}

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");
  const [trackData, setTrackData] = createSignal<trackDataT>({
    track: "n/a",
    artist: "n/a",
    coverart: "n/a"
  })
  invoke("initialize_audio", {})

  listen('get-data', (event) => {
    console.log("window_event (get-data):", event.payload)
    setGreetMsg((event.payload as {data: number[]}).data.toString())
  })

  listen('get-track', (event) => {
    console.log("window_event (get-track):", event.payload)
    setTrackData(event.payload as {artist: string, track: string, coverart: string})
  })
  
  setInterval(() => invoke("emit_track", {}), 30000)
  async function getTrack() {
  }

  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>

      <div class="row" style={{display: 'flex', "flex-direction": "column", "align-items": "center"}}>
        <span>
        {trackData().track}
        </span>
        <span>
        {trackData().artist}
        </span>
        
        <img src={trackData().coverart} width={400} />
      </div>

      <p>Click on the Tauri, Vite, and Solid logos to learn more.</p>

      <div class="row">
        <div>
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="button" onClick={() => getTrack()}>
            Get Track
          </button>
        </div>
      </div>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
