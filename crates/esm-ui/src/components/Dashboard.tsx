import { Swords, Trophy, TrendingUp, Inbox, Gamepad2, Moon } from 'lucide-react'
import type { ScheduleSlotInfo } from '@/lib/api'

interface StatCardProps {
  icon: React.ReactNode
  label: string
  value: string
  subtext?: string
  accentColor?: string
}

function StatCard({ icon, label, value, subtext, accentColor }: StatCardProps) {
  return (
    <div
      className="flex flex-col gap-2 p-4 rounded-lg border glow-hover transition-all"
      style={{
        backgroundColor: 'var(--bg-surface)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      <div className="flex items-center justify-between">
        <span
          className="text-xs font-semibold uppercase tracking-wider"
          style={{ color: 'var(--text-muted)' }}
        >
          {label}
        </span>
        <span style={{ color: accentColor || 'var(--text-muted)' }}>{icon}</span>
      </div>
      <span
        className="text-2xl font-bold tracking-tight font-display tabular-nums"
        style={{ color: 'var(--text-primary)' }}
      >
        {value}
      </span>
      {subtext && (
        <span className="text-xs" style={{ color: 'var(--text-secondary)' }}>
          {subtext}
        </span>
      )}
    </div>
  )
}

interface UpcomingMatchProps {
  nextMatch?: { blueTeam: string; redTeam: string; day: number; playerSide: 'blue' | 'red' }
}

function UpcomingMatch({ nextMatch }: UpcomingMatchProps) {
  return (
    <div
      className="p-4 rounded-lg border glow-hover"
      style={{
        backgroundColor: 'var(--bg-surface)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      <div
        className="text-xs font-semibold uppercase tracking-wider mb-3"
        style={{ color: 'var(--text-muted)' }}
      >
        Next Match
      </div>
      {!nextMatch ? (
        <div
          className="text-sm text-center py-4"
          style={{ color: 'var(--text-muted)' }}
        >
          No upcoming matches
        </div>
      ) : (
        <div className="flex items-center justify-between">
          <div className="flex flex-col items-center gap-1">
            <div
              className="w-12 h-12 rounded-full flex items-center justify-center text-sm font-bold"
              style={
                nextMatch.playerSide === 'blue'
                  ? { background: 'linear-gradient(135deg, var(--color-accent-emerald), var(--color-accent-cyan))', color: '#fff' }
                  : { backgroundColor: 'var(--bg-elevated)', color: 'var(--text-primary)', border: '1px solid var(--border-subtle)' }
              }
            >
              {nextMatch.blueTeam.slice(0, 3).toUpperCase()}
            </div>
            <span className="text-xs font-medium" style={{ color: 'var(--text-primary)' }}>
              {nextMatch.blueTeam}
            </span>
          </div>
          <div className="flex flex-col items-center gap-1">
            <span className="text-lg font-bold" style={{ color: 'var(--text-muted)' }}>
              VS
            </span>
            <span className="text-xs" style={{ color: 'var(--text-secondary)' }}>
              Day {nextMatch.day}
            </span>
          </div>
          <div className="flex flex-col items-center gap-1">
            <div
              className="w-12 h-12 rounded-full flex items-center justify-center text-sm font-bold"
              style={
                nextMatch.playerSide === 'red'
                  ? { background: 'linear-gradient(135deg, var(--color-accent-emerald), var(--color-accent-cyan))', color: '#fff' }
                  : { backgroundColor: 'var(--bg-elevated)', color: 'var(--text-primary)', border: '1px solid var(--border-subtle)' }
              }
            >
              {nextMatch.redTeam.slice(0, 3).toUpperCase()}
            </div>
            <span className="text-xs font-medium" style={{ color: 'var(--text-primary)' }}>
              {nextMatch.redTeam}
            </span>
          </div>
        </div>
      )}
    </div>
  )
}

function MiniBar({ value, color }: { value: number; color: string }) {
  const pct = Math.min(100, Math.max(0, value))
  return (
    <div className="flex items-center gap-1.5">
      <div
        className="flex-1 h-1.5 rounded-full overflow-hidden"
        style={{ backgroundColor: 'var(--bg-elevated)' }}
      >
        <div
          className="h-full rounded-full transition-all"
          style={{ width: `${pct}%`, backgroundColor: color }}
        />
      </div>
      <span
        className="text-xs tabular-nums w-6 text-right shrink-0"
        style={{ color: 'var(--text-secondary)' }}
      >
        {value}
      </span>
    </div>
  )
}

interface RosterPreviewProps {
  players?: { name: string; role: string; stamina?: number; morale?: number }[]
}

function RosterPreview({ players: playersProp }: RosterPreviewProps) {
  const players = playersProp ?? [
    { name: 'Zeus', role: 'Top' },
    { name: 'Oner', role: 'Jungle' },
    { name: 'Faker', role: 'Mid' },
    { name: 'Gumayusi', role: 'Bot' },
    { name: 'Keria', role: 'Support' },
  ]

  return (
    <div
      className="p-4 rounded-lg border glow-hover"
      style={{
        backgroundColor: 'var(--bg-surface)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      <div
        className="text-xs font-semibold uppercase tracking-wider mb-3"
        style={{ color: 'var(--text-muted)' }}
      >
        Roster Overview
      </div>
      <table className="w-full">
        <thead>
          <tr>
            <th
              className="text-left text-xs font-semibold uppercase tracking-wider py-2 px-3"
              style={{ color: 'var(--text-muted)' }}
            >
              Player
            </th>
            <th
              className="text-left text-xs font-semibold uppercase tracking-wider py-2 px-3"
              style={{ color: 'var(--text-muted)' }}
            >
              Role
            </th>
            <th
              className="text-left text-xs font-semibold uppercase tracking-wider py-2 px-3"
              style={{ color: 'var(--text-muted)' }}
            >
              Stamina
            </th>
            <th
              className="text-left text-xs font-semibold uppercase tracking-wider py-2 px-3"
              style={{ color: 'var(--text-muted)' }}
            >
              Morale
            </th>
          </tr>
        </thead>
        <tbody>
          {players.map((p) => {
            const moraleColor =
              (p.morale ?? 50) >= 70
                ? 'var(--color-win)'
                : (p.morale ?? 50) >= 40
                  ? 'var(--color-warning)'
                  : 'var(--color-loss)'
            return (
              <tr
                key={p.name}
                className="transition-colors"
                style={{ borderBottom: '1px solid var(--border-subtle)' }}
                onMouseEnter={(e) => { e.currentTarget.style.backgroundColor = 'var(--bg-elevated)' }}
                onMouseLeave={(e) => { e.currentTarget.style.backgroundColor = 'transparent' }}
              >
                <td className="py-2 px-3 text-sm font-medium" style={{ color: 'var(--text-primary)' }}>
                  {p.name}
                </td>
                <td className="py-2 px-3 text-xs" style={{ color: 'var(--text-secondary)' }}>
                  <span
                    className="px-2 py-0.5 rounded"
                    style={{
                      backgroundColor: 'rgba(6, 182, 212, 0.1)',
                      color: 'var(--color-accent-cyan)',
                    }}
                  >
                    {p.role}
                  </span>
                </td>
                <td className="py-2 px-3" style={{ minWidth: '80px' }}>
                  <MiniBar value={p.stamina ?? 0} color="var(--color-accent-cyan)" />
                </td>
                <td className="py-2 px-3" style={{ minWidth: '80px' }}>
                  <MiniBar value={p.morale ?? 0} color={moraleColor} />
                </td>
              </tr>
            )
          })}
        </tbody>
      </table>
    </div>
  )
}

function TodaySchedule({ slots }: { slots?: ScheduleSlotInfo[] }) {
  const defaultSlots: ScheduleSlotInfo[] = [
    { time_slot: 'Morning', entry_type: 'free', scrim_id: null, opponent: null, players: null, focus: null },
    { time_slot: 'Afternoon', entry_type: 'free', scrim_id: null, opponent: null, players: null, focus: null },
    { time_slot: 'Evening', entry_type: 'free', scrim_id: null, opponent: null, players: null, focus: null },
  ]
  const displaySlots = slots ?? defaultSlots

  return (
    <div
      className="p-4 rounded-lg border glow-hover"
      style={{
        backgroundColor: 'var(--bg-surface)',
        borderColor: 'var(--border-subtle)',
      }}
    >
      <div
        className="text-xs font-semibold uppercase tracking-wider mb-3"
        style={{ color: 'var(--text-muted)' }}
      >
        Today's Schedule
      </div>
      <div className="flex flex-col gap-2">
        {displaySlots.map((slot) => {
          const isFree = slot.entry_type === 'free'
          const color =
            slot.entry_type === 'scrim'
              ? 'var(--color-accent-cyan)'
              : slot.entry_type === 'solo_queue'
                ? 'var(--color-accent-emerald)'
                : slot.entry_type === 'rest'
                  ? 'var(--color-win)'
                  : 'var(--text-muted)'
          const icon =
            slot.entry_type === 'scrim' ? (
              <Swords size={12} />
            ) : slot.entry_type === 'solo_queue' ? (
              <Gamepad2 size={12} />
            ) : slot.entry_type === 'rest' ? (
              <Moon size={12} />
            ) : null
          const label =
            slot.entry_type === 'scrim'
              ? `Scrim vs ${slot.opponent ?? '?'}`
              : slot.entry_type === 'solo_queue'
                ? `Solo Queue${slot.focus ? ` · ${slot.focus}` : ''}`
                : slot.entry_type === 'rest'
                  ? 'Rest'
                  : '—'

          return (
            <div key={slot.time_slot} className="flex items-center gap-3">
              <span
                className="text-xs font-medium w-20 shrink-0"
                style={{ color: 'var(--text-muted)' }}
              >
                {slot.time_slot}
              </span>
              {isFree ? (
                <span className="text-xs" style={{ color: 'var(--text-muted)' }}>—</span>
              ) : (
                <div className="flex items-center gap-1.5" style={{ color }}>
                  {icon}
                  <span className="text-xs font-medium">{label}</span>
                </div>
              )}
            </div>
          )
        })}
      </div>
    </div>
  )
}

interface DashboardProps {
  teamName?: string
  record?: string
  standing?: string
  winStreak?: number
  budget?: string
  inboxCount?: number
  urgentCount?: number
  rosterPlayers?: { name: string; role: string; stamina?: number; morale?: number }[]
  nextMatch?: { blueTeam: string; redTeam: string; day: number; playerSide: 'blue' | 'red' }
  todaySlots?: ScheduleSlotInfo[]
}

export function Dashboard({
  record = '8-2',
  standing = '2nd place · LCK Spring',
  winStreak = 3,
  budget = '$1.2M',
  inboxCount = 3,
  urgentCount = 1,
  rosterPlayers,
  nextMatch,
  todaySlots,
}: DashboardProps) {
  return (
    <div className="flex flex-col gap-6 animate-fade-in-up">
      {/* KPI row */}
      <div className="grid grid-cols-4 gap-4">
        <StatCard
          icon={<Trophy size={18} />}
          label="Record"
          value={record}
          subtext={standing}
          accentColor="var(--color-win)"
        />
        <StatCard
          icon={<Swords size={18} />}
          label="Win Streak"
          value={String(winStreak)}
          subtext="Last: W vs DRX"
          accentColor="var(--color-accent-cyan)"
        />
        <StatCard
          icon={<TrendingUp size={18} />}
          label="Budget"
          value={budget}
          subtext="+$42.5K this week"
          accentColor="var(--color-warning)"
        />
        <StatCard
          icon={<Inbox size={18} />}
          label="Inbox"
          value={String(inboxCount)}
          subtext={`${urgentCount} urgent`}
          accentColor="var(--color-loss)"
        />
      </div>

      {/* Two-column layout */}
      <div className="grid grid-cols-3 gap-4">
        <div className="col-span-2">
          <RosterPreview players={rosterPlayers} />
        </div>
        <div className="flex flex-col gap-4">
          <UpcomingMatch nextMatch={nextMatch} />
          <TodaySchedule slots={todaySlots} />
        </div>
      </div>
    </div>
  )
}
