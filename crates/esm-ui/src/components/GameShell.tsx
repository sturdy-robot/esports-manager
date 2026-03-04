import { useState } from "react";
import { Sidebar } from "@/components/Sidebar";
import { TopBar } from "@/components/TopBar";
import { Dashboard } from "@/components/Dashboard";

const pageTitles: Record<string, string> = {
  dashboard: "Dashboard",
  roster: "Roster",
  schedule: "Schedule",
  standings: "Standings",
  results: "Results",
  finances: "Finances",
  staff: "Staff",
  scouting: "Scouting",
};

export function GameShell() {
  const [activePage, setActivePage] = useState("dashboard");

  return (
    <div
      className="flex w-full min-h-screen"
      style={{ backgroundColor: "var(--bg-base)" }}
    >
      <Sidebar activeItem={activePage} onNavigate={setActivePage} />
      <div className="flex flex-col flex-1 min-w-0">
        <TopBar title={pageTitles[activePage] || "Dashboard"} />
        <main className="flex-1 overflow-y-auto p-6">
          {activePage === "dashboard" && <Dashboard />}
          {activePage !== "dashboard" && (
            <div className="flex items-center justify-center h-64">
              <span
                className="text-lg"
                style={{ color: "var(--text-muted)" }}
              >
                {pageTitles[activePage]} — Coming soon
              </span>
            </div>
          )}
        </main>
      </div>
    </div>
  );
}
