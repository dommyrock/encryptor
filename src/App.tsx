import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [pwd, setPwd] = useState("");
  const [name, setName] = useState("");

  async function get_path() {
    setGreetMsg(await invoke("get_path", { name }));
  }

  async function get_pwd() {
    setPwd(await invoke("get_password", { name }));
  }

  return (
    <div className="container">
      <form
        className="w-[100vw]"
        onSubmit={(e) => {
          e.preventDefault();
          get_path();
          get_pwd();
        }}
      >
        <div className="row">
          <input
            id="path-input"
            type="text"
            className="w-[600px] h-[40px] bg-[#222] text-xl text-blue-400 px-6"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter File or Folder path..."
          />
        </div>
        <div className="row">
          <input
            id="pwd-input"
            type="password"
            className="w-[600px] h-[40px] bg-[#222] text-xl text-blue-400 px-6"
            onChange={(e) => setPwd(e.currentTarget.value)}
            placeholder="Enter encryption password..."
          />
        </div>

        <button type="submit">Encrypt data</button>
      </form>
      <p>{greetMsg}</p>

      <div
        id="grid-icon"
        className="flex justify-center absolute bottom-0 right-0 m-8 rounded text-black"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          strokeWidth={1.5}
          stroke="gray"
          className="w-10 h-10"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M10.5 19.5h3m-6.75 2.25h10.5a2.25 2.25 0 002.25-2.25v-15a2.25 2.25 0 00-2.25-2.25H6.75A2.25 2.25 0 004.5 4.5v15a2.25 2.25 0 002.25 2.25z"
          />
        </svg>
      </div>
    </div>
  );
}

export default App;
