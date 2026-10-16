import { useEffect, useState } from "react";
import { type About, commands } from "./bindings";

/** Placeholder shell until the first screens land. */
export function App() {
  const [about, setAbout] = useState<About | null>(null);

  useEffect(() => {
    void commands.getAbout().then(setAbout);
  }, []);

  return (
    <main>
      <h1>eSports Manager</h1>
      {about !== null && <p data-testid="version">v{about.version}</p>}
    </main>
  );
}
