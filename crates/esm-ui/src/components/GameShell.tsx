import { useState, useEffect, useCallback } from "react";
import { Sidebar } from "@/components/Sidebar";
import { TopBar } from "@/components/TopBar";
import { Dashboard } from "@/components/Dashboard";
import { Roster } from "@/components/Roster";
import { Inbox } from "@/components/Inbox";
import { Schedule } from "@/components/Schedule";
import { Standings } from "@/components/Standings";
import { Finances } from "@/components/Finances";
import { Staff } from "@/components/Staff";
import { Scouting } from "@/components/Scouting";
import { Results } from "@/components/Results";
import { useRoster, useInbox } from "@/lib/use-api";
import type { ScheduleMatch } from "@/components/Schedule";
import type { StandingsEntry } from "@/components/Standings";
import type { Transaction } from "@/components/Finances";
import type { StaffMember } from "@/components/Staff";
import type { ScoutingTarget } from "@/components/Scouting";
import type { MatchResult } from "@/components/Results";

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

// Placeholder results until wired to Tauri backend
const PLACEHOLDER_RESULTS: MatchResult[] = [
  { id: "r1", homeTeam: "T1", awayTeam: "DRX", homeWins: 2, awayWins: 0, day: 3, month: 1, year: 2025, bestOf: 3, playerTeamWon: true },
  { id: "r2", homeTeam: "Gen.G", awayTeam: "T1", homeWins: 2, awayWins: 1, day: 5, month: 1, year: 2025, bestOf: 3, playerTeamWon: false },
  { id: "r3", homeTeam: "T1", awayTeam: "KT Rolster", homeWins: 2, awayWins: 1, day: 8, month: 1, year: 2025, bestOf: 3, playerTeamWon: true },
];

// Placeholder scouting until wired to Tauri backend
const PLACEHOLDER_SCOUTING: ScoutingTarget[] = [
  { id: "p1", nickname: "Chovy", role: "Mid", team: "Gen.G", mechanics: 95, vision: 85, teamfighting: 88, estimatedValue: 800000, scoutingAccuracy: 72 },
  { id: "p2", nickname: "Peyz", role: "Bot", team: "Gen.G", mechanics: 82, vision: 74, teamfighting: 80, estimatedValue: 350000, scoutingAccuracy: 55 },
  { id: "p3", nickname: "Doran", role: "Top", team: "Hanwha Life", mechanics: 78, vision: 80, teamfighting: 82, estimatedValue: 300000, scoutingAccuracy: 90 },
];

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


interface GameShellProps {
  teamName?: string;
  year?: number;
  month?: number;
  day?: number;
  phase?: string;
  onContinue?: () => void;
  onSave?: () => void;
  onExitToMenu?: () => void;
}

export function GameShell({
  teamName = "T1",
  year = 2025,
  month = 1,
  day = 1,
  phase = "Morning",
  onContinue: onContinueProp,
  onSave,
  onExitToMenu,
}: GameShellProps) {
  const [activePage, setActivePage] = useState("dashboard");
  const { roster, fetchRoster } = useRoster();
  const { messages: inboxMessages, fetchInbox } = useInbox();

  // Fetch live data on mount
  useEffect(() => {
    fetchRoster();
    fetchInbox();
  }, [fetchRoster, fetchInbox]);

  // Wrap onContinue to also refresh data after advancing
  const handleContinue = useCallback(async () => {
    if (onContinueProp) {
      await onContinueProp();
      fetchRoster();
      fetchInbox();
    }
  }, [onContinueProp, fetchRoster, fetchInbox]);

  // Map API PlayerInfo → component RosterPlayer
  const rosterPlayers = roster.map((p) => ({
    nickname: p.nickname,
    firstName: p.first_name,
    lastName: p.last_name,
    role: p.role,
    stamina: p.stamina,
    morale: p.morale,
    mechanics: p.mechanics,
    vision: p.vision,
    teamfighting: p.teamfighting,
  }));

  // Map API InboxMessageInfo → component InboxMessage
  const inboxMapped = inboxMessages.map((m) => ({
    id: m.id,
    subject: m.subject,
    category: m.category,
    priority: m.priority as "Urgent" | "Action" | "Info",
    day: m.day,
    read: m.read,
  }));

  const renderPage = () => {
    switch (activePage) {
      case "dashboard":
        return (
          <Dashboard
            inboxCount={inboxMapped.length}
            urgentCount={inboxMapped.filter((m) => m.priority === "Urgent").length}
            rosterPlayers={rosterPlayers.map((p) => ({
              name: p.nickname,
              role: p.role,
            }))}
          />
        );
      case "roster":
        return <Roster players={rosterPlayers} />;
      case "schedule":
        return <Schedule matches={PLACEHOLDER_SCHEDULE} />;
      case "standings":
        return <Standings entries={PLACEHOLDER_STANDINGS} />;
      case "inbox":
        return <Inbox messages={inboxMapped} />;
      case "finances":
        return <Finances balance={1200000} income={75000} expenses={53000} transactions={PLACEHOLDER_TRANSACTIONS} />;
      case "staff":
        return <Staff members={PLACEHOLDER_STAFF} />;
      case "scouting":
        return <Scouting targets={PLACEHOLDER_SCOUTING} />;
      case "results":
        return <Results results={PLACEHOLDER_RESULTS} />;
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
      <Sidebar activeItem={activePage} onNavigate={setActivePage} onExitToMenu={onExitToMenu} />
      <div className="flex flex-col flex-1 min-w-0">
        <TopBar
          title={pageTitles[activePage] || "Dashboard"}
          teamName={teamName}
          year={year}
          month={month}
          day={day}
          phase={phase}
          onSave={onSave}
          onContinue={handleContinue}
        />
        <main className="flex-1 overflow-y-auto p-6">
          {renderPage()}
        </main>
      </div>
    </div>
  );
}
