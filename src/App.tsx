import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { listen } from '@tauri-apps/api/event'
function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");
  invoke("initialize_audio", {})

  listen('get-data', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    console.log(event.payload)
    setGreetMsg((event.payload as {data: number[]}).data.toString())
    console.log(Date.now())
  })
  

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // (invoke("get_audio_data", {/* name: name() */}))
    // .then(d => console.log(d));
    // setTimeout(() => {
    //   const start = Date.now()
    //   invoke("get_audio_data", {/* name: name() */})
    //     .then(d => {
    //       console.log(Date.now() - start)
    //       console.log(d)
    //     })
    // }, 5000);
    console.log(Date.now())
    invoke("test", {})
  }

  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>

      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://solidjs.com" target="_blank">
          <img src={logo} class="logo solid" alt="Solid logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and Solid logos to learn more.</p>

      <div class="row">
        <div>
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="button" onClick={() => greet()}>
            Greet
          </button>
        </div>
      </div>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
