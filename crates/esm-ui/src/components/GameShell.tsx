import { useState, useEffect, useCallback } from "react";
import { Sidebar } from "@/components/Sidebar";
import { TopBar } from "@/components/TopBar";
import { Dashboard } from "@/components/Dashboard";
import { Roster } from "@/components/Roster";
import { Inbox } from "@/components/Inbox";
import { TeamScheduleView } from "@/components/TeamScheduleView";
import { Standings } from "@/components/Standings";
import { Finances } from "@/components/Finances";
import { Staff } from "@/components/Staff";
import { Scouting } from "@/components/Scouting";
import { Results } from "@/components/Results";
import { TournamentEnd } from "@/components/TournamentEnd";
import { MatchLobby } from "@/components/MatchLobby";
import { useRoster, useInbox, useStandings, useSchedule, useResolveMessage, usePlayMatchDelegate, useTeamSchedule } from "@/lib/use-api";
import type { Transaction } from "@/components/Finances";
import type { StaffMember } from "@/components/Staff";
import type { ScoutingTarget } from "@/components/Scouting";
import type { MatchMode } from "./PlayMatchButton";

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
  "match-lobby": "Match Day",
  draft: "Draft Phase",
  "tournament-end": "Season Results",
};

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



interface GameShellProps {
  teamName?: string;
  year?: number;
  month?: number;
  day?: number;
  phase?: string;
  isMatchDay?: boolean;
  onContinue?: () => void;
  onPlayMatch?: (mode: MatchMode) => void;
  onSave?: () => void;
  onExitToMenu?: () => void;
}

export function GameShell({
  teamName = "T1",
  year = 2025,
  month = 1,
  day = 1,
  phase = "Morning",
  isMatchDay = false,
  onContinue: onContinueProp,
  onPlayMatch: onPlayMatchProp,
  onSave,
  onExitToMenu,
}: GameShellProps) {
  const [activePage, setActivePage] = useState("dashboard");
  const { roster, fetchRoster } = useRoster();
  const { messages: inboxMessages, fetchInbox } = useInbox();
  const { standings, fetchStandings } = useStandings();
  const { schedule, fetchSchedule } = useSchedule();
  const { resolve } = useResolveMessage();
  const { playMatchDelegate, playing: simulating } = usePlayMatchDelegate();
  const {
    weekSchedule, scrims, refresh: refreshTeamSchedule,
    scheduleScrim, cancelScrim, scheduleSoloQueue, scheduleRest, clearSlot,
  } = useTeamSchedule();
  const [resolvingMsgId, setResolvingMsgId] = useState<string | null>(null);
  const hasUrgentUnread = inboxMessages.some((m) => m.priority === 'Urgent' && !m.read);

  // Fetch live data on mount
  useEffect(() => {
    fetchRoster();
    fetchInbox();
    fetchStandings();
    fetchSchedule();
    refreshTeamSchedule();
  }, [fetchRoster, fetchInbox, fetchStandings, fetchSchedule, refreshTeamSchedule]);

  // Wrap onContinue to also refresh data after advancing
  const handleContinue = useCallback(async () => {
    if (onContinueProp) {
      await onContinueProp();
      fetchRoster();
      fetchInbox();
      fetchStandings();
      fetchSchedule();
    }
  }, [onContinueProp, fetchRoster, fetchInbox, fetchStandings, fetchSchedule]);

  const handlePlayMatch = useCallback((mode: MatchMode) => {
    if (onPlayMatchProp) {
      onPlayMatchProp(mode);
    }
  }, [onPlayMatchProp]);

  const handleDelegate = useCallback(async () => {
    const info = await playMatchDelegate();
    if (info) {
      await fetchSchedule();
      await fetchStandings();
      setActivePage("results");
    }
  }, [playMatchDelegate, fetchSchedule, fetchStandings]);

  const handleResolveMessage = async (id: string, subject: string) => {
    setResolvingMsgId(id);
    const ok = await resolve(id);
    setResolvingMsgId(null);
    if (ok) {
      if (subject === "Tournament Concluded") {
        setActivePage("tournament-end");
        fetchInbox(); // refresh inbox so message is marked read
      } else {
        fetchInbox();
      }
    }
  };

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
      case "dashboard": {
        const nextMatch = schedule.find(
          (m) => (m.blue_team === teamName || m.red_team === teamName) && m.winner === null
        );
        const currentDayIndex = day % 7;
        const todaySlots = weekSchedule?.days[currentDayIndex]?.slots;
        return (
          <Dashboard
            inboxCount={inboxMapped.length}
            urgentCount={inboxMapped.filter((m) => m.priority === "Urgent").length}
            rosterPlayers={rosterPlayers.map((p) => ({
              name: p.nickname,
              role: p.role,
              stamina: p.stamina,
              morale: p.morale,
            }))}
            nextMatch={nextMatch ? {
              blueTeam: nextMatch.blue_team,
              redTeam: nextMatch.red_team,
              day: nextMatch.scheduled_day,
              playerSide: nextMatch.blue_team === teamName ? 'blue' : 'red',
            } : undefined}
            todaySlots={todaySlots}
          />
        );
      }
      case "roster":
        return <Roster players={rosterPlayers} />;
      case "schedule":
        return (
          <TeamScheduleView
            schedule={weekSchedule}
            scrims={scrims}
            rosterNames={rosterPlayers.map((p) => p.nickname)}
            teamNames={standings.map((s) => s.team_name)}
            onScheduleScrim={async (dayIndex, timeSlot, awayTeamIndex, gameCount, draftRules) => {
              await scheduleScrim({ away_team_index: awayTeamIndex, scheduled_day: dayIndex, time_slot: timeSlot, game_count: gameCount, draft_rules: draftRules });
            }}
            onScheduleSoloQueue={async (dayIndex, timeSlot, players, focus) => {
              await scheduleSoloQueue({ day_index: dayIndex, time_slot: timeSlot, players, focus });
            }}
            onScheduleRest={async (dayIndex, timeSlot) => {
              await scheduleRest({ day_index: dayIndex, time_slot: timeSlot });
            }}
            onClearSlot={async (dayIndex, timeSlot) => {
              await clearSlot(dayIndex, timeSlot);
            }}
            onCancelScrim={async (scrimId) => {
              await cancelScrim(scrimId);
            }}
          />
        );
      case "standings":
        return (
          <Standings
            entries={standings.map((s) => ({
              rank: s.rank,
              teamName: s.team_name,
              wins: s.wins,
              losses: s.losses,
              mapWins: s.wins,
              mapLosses: s.losses,
              streak: "",
            }))}
          />
        );
      case "inbox":
        return <Inbox messages={inboxMapped} onResolveMessage={handleResolveMessage} resolvingMsgId={resolvingMsgId} />;
      case "finances":
        return <Finances balance={1200000} income={75000} expenses={53000} transactions={PLACEHOLDER_TRANSACTIONS} />;
      case "staff":
        return <Staff members={PLACEHOLDER_STAFF} />;
      case "scouting":
        return <Scouting targets={PLACEHOLDER_SCOUTING} />;
      case "results":
        return (
          <Results
            results={schedule
              .filter((m) => m.winner !== null)
              .map((m) => ({
                id: String(m.id),
                homeTeam: m.blue_team,
                awayTeam: m.red_team,
                homeWins: m.winner === m.blue_team ? 1 : 0,
                awayWins: m.winner === m.red_team ? 1 : 0,
                day: m.scheduled_day,
                month: month ?? 1,
                year: year ?? 2025,
                bestOf: 1,
                playerTeamWon: m.winner === teamName,
              }))}
          />
        );
      case "tournament-end":
        return (
          <TournamentEnd
            seasonName={`${year} Season`}
            standings={standings}
            teamName={teamName}
            onFinish={() => setActivePage("dashboard")}
          />
        );
      case "match-lobby": {
        const pMatch = schedule.find(
          (m) => (m.blue_team === teamName || m.red_team === teamName) && m.winner === null
        );
        return (
          <MatchLobby
            match={pMatch ? { homeTeam: pMatch.blue_team, awayTeam: pMatch.red_team, day: pMatch.scheduled_day } : undefined}
            teamName={teamName}
            simulating={simulating}
            onDelegate={handleDelegate}
            onDraft={() => setActivePage("draft")}
          />
        );
      }
      case "draft":
        return (
          <div className="flex items-center justify-center h-64">
            <span className="text-lg" style={{ color: "var(--text-muted)" }}>
              Draft is available during match day — use Play Match.
            </span>
          </div>
        );
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
      className="flex w-full h-screen"
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
          isMatchDay={isMatchDay}
          onSave={onSave}
          onContinue={handleContinue}
          onPlayMatch={onPlayMatchProp ? handlePlayMatch : undefined}
          continueDisabled={hasUrgentUnread}
        />
        <main className="flex-1 overflow-y-auto p-6">
          {renderPage()}
        </main>
      </div>
    </div>
  );
}
