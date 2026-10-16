import { createRootRoute, createRoute, createRouter } from "@tanstack/react-router";
import { Home } from "./routes/Home";
import { Shell } from "./routes/Shell";

const rootRoute = createRootRoute({ component: Shell });

const homeRoute = createRoute({ getParentRoute: () => rootRoute, path: "/", component: Home });

export const router = createRouter({ routeTree: rootRoute.addChildren([homeRoute]) });

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
