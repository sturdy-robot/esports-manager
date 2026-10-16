import "./styles/index.css";
import { QueryClient } from "@tanstack/react-query";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import { createLocalization } from "./lib/i18n/localization";

const root = document.getElementById("root");
if (root === null) {
  throw new Error("index.html is missing the #root element");
}

createRoot(root).render(
  <StrictMode>
    <App l10n={createLocalization(navigator.languages)} queryClient={new QueryClient()} />
  </StrictMode>,
);
