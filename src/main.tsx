import ReactDOM from "react-dom/client";
import { StrictMode } from "react";

import App from "./App";
import Titlebar from "./components/ui/Titlebar";

import "./styles/globals.css";
ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <Titlebar />
    <App />
  </StrictMode>
);