import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import dayjs from "dayjs";
import utc from "dayjs/plugin/utc";

dayjs.extend(utc);

import Providers from "@/lib/providers";

import "./index.css";

const root = document.getElementById("root")!;

if (!root) {
  throw new Error("Root element not found");
}

const loadingContainer = document.getElementById("loading-container");
const loadingContent = document.getElementById("loading-content");

if (loadingContent) {
  loadingContent.classList.add("scale-150");
  loadingContent.classList.add("opacity-100");
}

if (loadingContainer) {
  loadingContainer.style.opacity = "0";
  setTimeout(() => {
    loadingContainer.remove();
  }, 300);
}

createRoot(root).render(
  <StrictMode>
    <Providers />
  </StrictMode>,
);
