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
      className="flex items-center gap-4 w-full px-6 py-4 rounded-lg border text-left cursor-pointer transition-all glow-hover"
      style={{
        backgroundColor: "var(--bg-surface)",
        borderColor: "var(--border-subtle)",
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
        className="flex items-center justify-center w-10 h-10 rounded-md"
        style={{ color: "var(--color-accent-cyan)" }}
      >
        {icon}
      </span>
      <div className="flex flex-col">
        <span className="text-base font-semibold">{label}</span>
        <span
          className="text-xs"
          style={{ color: "var(--text-secondary)" }}
        >
          {sublabel}
        </span>
      </div>
    </button>
  );
}

export function MainMenu({ onNavigate }: MainMenuProps) {
  return (
    <div
      className="flex flex-col items-center justify-center min-h-screen w-full animate-fade-in-up"
      style={{ backgroundColor: "var(--bg-base)" }}
    >
      {/* Logo + Title */}
      <div className="flex flex-col items-center gap-2 mb-12">
        <h1 className="text-4xl font-bold tracking-tight accent-gradient-text">
          eSports Manager
        </h1>
        <p
          className="text-sm"
          style={{ color: "var(--text-secondary)" }}
        >
          v0.1.0-alpha
        </p>
      </div>

      {/* Menu buttons */}
      <div className="flex flex-col gap-3 w-full max-w-sm">
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
  );
}
