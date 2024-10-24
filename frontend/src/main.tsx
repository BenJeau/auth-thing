import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import "./index.css";
import App from "./routes";
import Providers from "./lib/providers.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Providers>
      <App />
    </Providers>
  </StrictMode>
);
