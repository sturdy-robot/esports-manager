import { useQuery } from "@tanstack/react-query";
import { commands } from "../bindings";

/** What the game reports about itself (`app::about`). */
export function useAbout() {
  return useQuery({ queryKey: ["about"], queryFn: () => commands.getAbout() });
}
