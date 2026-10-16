import type { ReactLocalization } from "@fluent/react";
import { LocalizationProvider } from "@fluent/react";
import { type QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { RouterProvider } from "@tanstack/react-router";
import { router } from "./router";

interface AppProps {
  l10n: ReactLocalization;
  queryClient: QueryClient;
}

export function App({ l10n, queryClient }: AppProps) {
  return (
    <LocalizationProvider l10n={l10n}>
      <QueryClientProvider client={queryClient}>
        <RouterProvider router={router} />
      </QueryClientProvider>
    </LocalizationProvider>
  );
}
