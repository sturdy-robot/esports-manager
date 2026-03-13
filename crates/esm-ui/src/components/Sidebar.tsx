import {
  LayoutDashboard,
  Users,
  Calendar,
  Trophy,
  FileText,
  DollarSign,
  UserCog,
  Search,
  Inbox,
  LogOut,
  ChevronLeft,
  ChevronRight,
} from "lucide-react";
import { useState } from "react";

interface NavItem {
  icon: React.ReactNode;
  label: string;
  id: string;
}

interface NavGroup {
  title: string;
  items: NavItem[];
}

const navGroups: NavGroup[] = [
  {
    title: "Team",
    items: [
      {
        icon: <LayoutDashboard size={20} />,
        label: "Dashboard",
        id: "dashboard",
      },
      { icon: <Inbox size={20} />, label: "Inbox", id: "inbox" },
      { icon: <Users size={20} />, label: "Roster", id: "roster" },
      { icon: <Calendar size={20} />, label: "Schedule", id: "schedule" },
    ],
  },
  {
    title: "League",
    items: [
      { icon: <Trophy size={20} />, label: "Standings", id: "standings" },
      { icon: <FileText size={20} />, label: "Results", id: "results" },
    ],
  },
  {
    title: "Management",
    items: [
      { icon: <DollarSign size={20} />, label: "Finances", id: "finances" },
      { icon: <UserCog size={20} />, label: "Staff", id: "staff" },
      { icon: <Search size={20} />, label: "Scouting", id: "scouting" },
    ],
  },
];

interface SidebarProps {
  activeItem: string;
  onNavigate: (id: string) => void;
  onExitToMenu?: () => void;
}

export function Sidebar({
  activeItem,
  onNavigate,
  onExitToMenu,
}: SidebarProps) {
  const [collapsed, setCollapsed] = useState(false);

  return (
    <aside
      className={`app-panel flex flex-col border-r transition-all m-3 mr-0 rounded-2xl overflow-hidden ${collapsed ? "w-16" : "w-64"}`}
      style={{
        borderRightColor: "var(--border-subtle)",
      }}
    >
      {/* Logo + collapse toggle */}
      <div
        className="flex items-center justify-between px-4 h-18 border-b"
        style={{ borderColor: "var(--border-subtle)" }}
      >
        {!collapsed && (
          <div className="flex flex-col gap-1">
            <span className="app-eyebrow">Control Room</span>
            <div className="flex items-center gap-2">
              <span className="text-lg font-bold tracking-tight accent-gradient-text font-display">
                ESM
              </span>
              <span
                className="text-xs font-medium"
                style={{ color: "var(--text-secondary)" }}
              >
                eSports Manager
              </span>
            </div>
          </div>
        )}
        <button
          onClick={() => setCollapsed(!collapsed)}
          className="app-icon-button flex items-center justify-center w-8 h-8 rounded-xl cursor-pointer border transition-colors"
          style={{
            color: "var(--text-muted)",
          }}
        >
          {collapsed ? <ChevronRight size={16} /> : <ChevronLeft size={16} />}
        </button>
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto py-4">
        {navGroups.map((group) => (
          <div key={group.title} className="mb-4 px-2">
            {!collapsed && (
              <div
                className="px-3 py-2 text-[11px] font-semibold uppercase tracking-[0.22em]"
                style={{ color: "var(--text-muted)" }}
              >
                {group.title}
              </div>
            )}
            {group.items.map((item) => {
              const isActive = activeItem === item.id;
              return (
                <button
                  key={item.id}
                  onClick={() => onNavigate(item.id)}
                  className={`flex items-center gap-3 w-full px-3 py-2.5 rounded-xl text-sm font-medium transition-colors cursor-pointer border outline-none ${collapsed ? "justify-center" : ""}`}
                  style={{
                    background: isActive
                      ? "var(--accent-gradient-soft)"
                      : "transparent",
                    color: isActive
                      ? "var(--text-primary)"
                      : "var(--text-secondary)",
                    borderColor: isActive
                      ? "var(--border-strong)"
                      : "transparent",
                    boxShadow: isActive ? "var(--accent-glow)" : "none",
                  }}
                  onMouseEnter={(e) => {
                    if (!isActive) {
                      e.currentTarget.style.backgroundColor =
                        "rgba(34, 211, 238, 0.06)";
                      e.currentTarget.style.borderColor =
                        "rgba(34, 211, 238, 0.1)";
                      e.currentTarget.style.color = "var(--text-primary)";
                    }
                  }}
                  onMouseLeave={(e) => {
                    if (!isActive) {
                      e.currentTarget.style.backgroundColor = "transparent";
                      e.currentTarget.style.borderColor = "transparent";
                      e.currentTarget.style.color = "var(--text-secondary)";
                    }
                  }}
                >
                  {item.icon}
                  {!collapsed && <span>{item.label}</span>}
                </button>
              );
            })}
          </div>
        ))}
      </nav>

      {/* Exit to Menu */}
      {onExitToMenu && (
        <button
          onClick={onExitToMenu}
          className={`flex items-center gap-3 w-full px-4 py-3 text-sm font-medium cursor-pointer border-none outline-none transition-colors ${collapsed ? "justify-center" : ""}`}
          style={{
            borderTop: "1px solid var(--border-subtle)",
            color: "var(--text-secondary)",
            backgroundColor: "transparent",
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = "var(--bg-elevated)";
            e.currentTarget.style.color = "var(--color-loss)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = "transparent";
            e.currentTarget.style.color = "var(--text-secondary)";
          }}
        >
          <LogOut size={18} />
          {!collapsed && <span>Exit to Menu</span>}
        </button>
      )}
    </aside>
  );
}
