import { useState } from "react";
import { Sidebar } from "@/components/Sidebar";
import { TopBar } from "@/components/TopBar";
import { Dashboard } from "@/components/Dashboard";
import { Roster } from "@/components/Roster";
import { Inbox } from "@/components/Inbox";
import { Schedule } from "@/components/Schedule";
import type { RosterPlayer } from "@/components/Roster";
import type { InboxMessage } from "@/components/Inbox";
import type { ScheduleMatch } from "@/components/Schedule";

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

// Placeholder schedule until wired to Tauri backend
const PLACEHOLDER_SCHEDULE: ScheduleMatch[] = [
  { id: "s1", homeTeam: "T1", awayTeam: "DRX", day: 3, month: 1, year: 2025, bestOf: 3, result: { homeWins: 2, awayWins: 1 } },
  { id: "s2", homeTeam: "T1", awayTeam: "Gen.G", day: 5, month: 1, year: 2025, bestOf: 3, result: null },
  { id: "s3", homeTeam: "KT Rolster", awayTeam: "T1", day: 8, month: 1, year: 2025, bestOf: 3, result: null },
];

// Placeholder inbox until wired to Tauri backend
const PLACEHOLDER_INBOX: InboxMessage[] = [
  { id: "1", subject: "Welcome to your new team!", category: "System", priority: "Info", day: 1, read: false },
  { id: "2", subject: "Pre-season roster review required", category: "Staff", priority: "Action", day: 1, read: false },
  { id: "3", subject: "Sponsor offer: TechCorp $50K/season", category: "Finance", priority: "Action", day: 1, read: false },
];

// Placeholder roster until wired to Tauri backend
const PLACEHOLDER_ROSTER: RosterPlayer[] = [
  { nickname: "Zeus", firstName: "Woo-je", lastName: "Choi", role: "Top", stamina: 75, morale: 85, mechanics: 88, vision: 80, teamfighting: 85 },
  { nickname: "Oner", firstName: "Hyeon-jun", lastName: "Moon", role: "Jungle", stamina: 90, morale: 78, mechanics: 85, vision: 91, teamfighting: 88 },
  { nickname: "Faker", firstName: "Sang-hyeok", lastName: "Lee", role: "Mid", stamina: 82, morale: 90, mechanics: 97, vision: 88, teamfighting: 92 },
  { nickname: "Gumayusi", firstName: "Min-hyeok", lastName: "Lee", role: "Bot", stamina: 88, morale: 82, mechanics: 90, vision: 78, teamfighting: 86 },
  { nickname: "Keria", firstName: "Min-seok", lastName: "Ryu", role: "Support", stamina: 85, morale: 88, mechanics: 86, vision: 94, teamfighting: 91 },
];

interface GameShellProps {
  teamName?: string;
  year?: number;
  month?: number;
  day?: number;
  phase?: string;
  onSaveAndExit?: () => void;
}

export function GameShell({
  teamName = "T1",
  year = 2025,
  month = 1,
  day = 1,
  phase = "Morning",
  onSaveAndExit,
}: GameShellProps) {
  const [activePage, setActivePage] = useState("dashboard");

  const handleContinue = () => {
    // Placeholder: will invoke Tauri advance_turn command
  };

  const renderPage = () => {
    switch (activePage) {
      case "dashboard":
        return <Dashboard />;
      case "roster":
        return <Roster players={PLACEHOLDER_ROSTER} />;
      case "schedule":
        return <Schedule matches={PLACEHOLDER_SCHEDULE} />;
      case "results":
        return <Inbox messages={PLACEHOLDER_INBOX} />;
      default:
        return (
          <div className="flex items-center justify-center h-64">
            <span
              className="text-lg"
              style={{ color: "var(--text-muted)" }}
            >
              {pageTitles[activePage]} — Coming soon
            </span>
          </div>
        );
    }
  };

  return (
    <div
      className="flex w-full min-h-screen"
      style={{ backgroundColor: "var(--bg-base)" }}
    >
      <Sidebar activeItem={activePage} onNavigate={setActivePage} />
      <div className="flex flex-col flex-1 min-w-0">
        <TopBar
          title={pageTitles[activePage] || "Dashboard"}
          teamName={teamName}
          year={year}
          month={month}
          day={day}
          phase={phase}
          onContinue={handleContinue}
          onSaveAndExit={onSaveAndExit}
        />
        <main className="flex-1 overflow-y-auto p-6">
          {renderPage()}
        </main>
      </div>
    </div>
  );
}
