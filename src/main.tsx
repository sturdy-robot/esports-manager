import "./styles/index.css";
import { QueryClient } from "@tanstack/react-query";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import { DEFAULT_LOCALE, negotiateLocales } from "./lib/i18n/locales";
import { createLocalization } from "./lib/i18n/localization";

async function start(root: HTMLElement): Promise<void> {
  if (import.meta.env.MODE === "e2e") {
    // Browser-only E2E runs have no Tauri backend; this branch is removed
    // from normal builds.
    const { installIpcMocks } = await import("./test/e2e-ipc");
    installIpcMocks();
  }

  const locales = negotiateLocales(navigator.languages);
  document.documentElement.lang = locales[0] ?? DEFAULT_LOCALE;

  createRoot(root).render(
    <StrictMode>
      <App l10n={createLocalization(locales)} queryClient={new QueryClient()} />
    </StrictMode>,
  );
}

const root = document.getElementById("root");
if (root === null) {
  throw new Error("index.html is missing the #root element");
}
void start(root);
