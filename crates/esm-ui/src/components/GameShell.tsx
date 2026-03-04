import { useState } from "react";
import { Sidebar } from "@/components/Sidebar";
import { TopBar } from "@/components/TopBar";
import { Dashboard } from "@/components/Dashboard";
import { Roster } from "@/components/Roster";
import { Inbox } from "@/components/Inbox";
import { Schedule } from "@/components/Schedule";
import { Standings } from "@/components/Standings";
import { Finances } from "@/components/Finances";
import { Staff } from "@/components/Staff";
import type { RosterPlayer } from "@/components/Roster";
import type { InboxMessage } from "@/components/Inbox";
import type { ScheduleMatch } from "@/components/Schedule";
import type { StandingsEntry } from "@/components/Standings";
import type { Transaction } from "@/components/Finances";
import type { StaffMember } from "@/components/Staff";

const pageTitles: Record<string, string> = {
  dashboard: "Dashboard",
  inbox: "Inbox",
  roster: "Roster",
  schedule: "Schedule",
  standings: "Standings",
  results: "Results",
  finances: "Finances",
  staff: "Staff",
  scouting: "Scouting",
};

// Placeholder staff until wired to Tauri backend
const PLACEHOLDER_STAFF: StaffMember[] = [
  { id: "s1", name: "Park Ji-sung", role: "Head Coach", skill: 88 },
  { id: "s2", name: "Kim Dae-ho", role: "Assistant Coach", skill: 75 },
  { id: "s3", name: "Lee Min-ji", role: "Positional Coach", skill: 70 },
  { id: "s4", name: "Choi Yeon-su", role: "Psychologist", skill: 82 },
];

// Placeholder finances until wired to Tauri backend
const PLACEHOLDER_TRANSACTIONS: Transaction[] = [
  { id: "t1", description: "Player salary — Faker", amount: -45000, date: "Jan 1", category: "Salary" },
  { id: "t2", description: "Sponsor payment — TechCorp", amount: 50000, date: "Jan 1", category: "Sponsor" },
  { id: "t3", description: "Scrim facility rental", amount: -8000, date: "Jan 2", category: "Operations" },
  { id: "t4", description: "Prize money — LCK Week 1", amount: 25000, date: "Jan 3", category: "Prize" },
];

// Placeholder standings until wired to Tauri backend
const PLACEHOLDER_STANDINGS: StandingsEntry[] = [
  { rank: 1, teamName: "T1", wins: 8, losses: 2, mapWins: 18, mapLosses: 7, streak: "W3" },
  { rank: 2, teamName: "Gen.G", wins: 7, losses: 3, mapWins: 16, mapLosses: 9, streak: "L1" },
  { rank: 3, teamName: "Dplus KIA", wins: 6, losses: 4, mapWins: 14, mapLosses: 11, streak: "W1" },
  { rank: 4, teamName: "Hanwha Life", wins: 5, losses: 5, mapWins: 13, mapLosses: 12, streak: "L2" },
  { rank: 5, teamName: "KT Rolster", wins: 3, losses: 7, mapWins: 9, mapLosses: 16, streak: "L3" },
  { rank: 6, teamName: "DRX", wins: 1, losses: 9, mapWins: 5, mapLosses: 20, streak: "L5" },
];

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
      case "standings":
        return <Standings entries={PLACEHOLDER_STANDINGS} />;
      case "inbox":
        return <Inbox messages={PLACEHOLDER_INBOX} />;
      case "finances":
        return <Finances balance={1200000} income={75000} expenses={53000} transactions={PLACEHOLDER_TRANSACTIONS} />;
      case "staff":
        return <Staff members={PLACEHOLDER_STAFF} />;
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
