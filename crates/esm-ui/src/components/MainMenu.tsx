import { Gamepad2, FolderOpen, Settings, LogOut } from "lucide-react";

export type MenuTarget = "new-game" | "load-game" | "settings" | "exit";

interface MainMenuProps {
  onNavigate: (target: MenuTarget) => void;
}

interface MenuButtonProps {
  icon: React.ReactNode;
  label: string;
  sublabel: string;
  onClick: () => void;
}

function MenuButton({ icon, label, sublabel, onClick }: MenuButtonProps) {
  return (
    <button
      onClick={onClick}
      className="app-panel flex items-center gap-4 w-full px-6 py-4 rounded-2xl text-left cursor-pointer transition-all glow-hover"
      style={{
        color: "var(--text-primary)",
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.backgroundColor = "var(--bg-elevated)";
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.backgroundColor = "var(--bg-surface)";
      }}
    >
      <span
        className="flex items-center justify-center w-11 h-11 rounded-xl"
        style={{
          color: "var(--color-accent-cyan)",
          background: "var(--accent-gradient-soft)",
          border: "1px solid var(--border-strong)",
        }}
      >
        {icon}
      </span>
      <div className="flex flex-col">
        <span className="text-base font-semibold">{label}</span>
        <span className="text-xs" style={{ color: "var(--text-secondary)" }}>
          {sublabel}
        </span>
      </div>
    </button>
  );
}

export function MainMenu({ onNavigate }: MainMenuProps) {
  return (
    <div
      className="app-shell flex flex-col items-center justify-center min-h-screen w-full animate-fade-in-up relative px-6"
      style={{ backgroundColor: "var(--bg-base)" }}
    >
      <div className="app-panel-strong w-full max-w-lg rounded-lg px-8 py-10">
        <div className="flex flex-col gap-3 mb-10">
          <h1 className="app-page-title max-w-2xl">eSports Manager</h1>

          <div
            className="flex items-center gap-3 text-xs"
            style={{ color: "var(--text-muted)" }}
          >
            <span
              className="px-2.5 py-1 rounded-full"
              style={{
                background: "var(--accent-gradient-soft)",
                border: "1px solid var(--border-strong)",
                color: "var(--text-primary)",
              }}
            >
              v0.1.0-alpha
            </span>
          </div>
        </div>

        <div className="flex justify-center">
          <div className="flex w-full max-w-md flex-col gap-3">
            <MenuButton
              icon={<Gamepad2 size={22} />}
              label="New Game"
              sublabel="Start a new career"
              onClick={() => onNavigate("new-game")}
            />
            <MenuButton
              icon={<FolderOpen size={22} />}
              label="Load Game"
              sublabel="Continue a saved career"
              onClick={() => onNavigate("load-game")}
            />
            <MenuButton
              icon={<Settings size={22} />}
              label="Settings"
              sublabel="Configure game options"
              onClick={() => onNavigate("settings")}
            />
            <MenuButton
              icon={<LogOut size={22} />}
              label="Exit"
              sublabel="Quit the game"
              onClick={() => onNavigate("exit")}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
