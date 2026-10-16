import { createRootRoute, createRoute, createRouter } from "@tanstack/react-router";
import { LoadGame } from "./routes/LoadGame";
import { MainMenu } from "./routes/MainMenu";
import { NewGame } from "./routes/NewGame";
import { Player } from "./routes/Player";
import { Roster } from "./routes/Roster";
import { Shell } from "./routes/Shell";

const rootRoute = createRootRoute({ component: Shell });

const menuRoute = createRoute({ getParentRoute: () => rootRoute, path: "/", component: MainMenu });

const newGameRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/new",
  component: NewGame,
});

const loadGameRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/load",
  component: LoadGame,
});

function text(value: unknown): string | undefined {
  return typeof value === "string" && value !== "" ? value : undefined;
}

// Ids hold `:` and `.`, so they travel as search parameters, not path segments.
const rosterRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/roster",
  validateSearch: (search: Record<string, unknown>): { team?: string } => {
    const team = text(search.team);
    return team === undefined ? {} : { team };
  },
  component: function RosterRoute() {
    const { team } = rosterRoute.useSearch();
    return <Roster team={team} />;
  },
});

const playerRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/player",
  validateSearch: (search: Record<string, unknown>): { id: string } => ({
    id: text(search.id) ?? "",
  }),
  component: function PlayerRoute() {
    const { id } = playerRoute.useSearch();
    return <Player id={id} />;
  },
});

export const router = createRouter({
  routeTree: rootRoute.addChildren([
    menuRoute,
    newGameRoute,
    loadGameRoute,
    rosterRoute,
    playerRoute,
  ]),
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
