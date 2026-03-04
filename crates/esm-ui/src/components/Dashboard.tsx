import { Swords, Trophy, TrendingUp, Inbox } from 'lucide-react'

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
        className="text-2xl font-bold tracking-tight"
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

function UpcomingMatch() {
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
      <div className="flex items-center justify-between">
        <div className="flex flex-col items-center gap-1">
          <div
            className="w-12 h-12 rounded-full flex items-center justify-center text-sm font-bold accent-gradient"
            style={{ color: '#fff' }}
          >
            T1
          </div>
          <span className="text-xs font-medium" style={{ color: 'var(--text-primary)' }}>
            T1
          </span>
        </div>
        <div className="flex flex-col items-center gap-1">
          <span className="text-lg font-bold" style={{ color: 'var(--text-muted)' }}>
            VS
          </span>
          <span className="text-xs" style={{ color: 'var(--text-secondary)' }}>
            Tomorrow · 14:00
          </span>
        </div>
        <div className="flex flex-col items-center gap-1">
          <div
            className="w-12 h-12 rounded-full flex items-center justify-center text-sm font-bold"
            style={{
              backgroundColor: 'var(--bg-elevated)',
              color: 'var(--text-primary)',
              border: '1px solid var(--border-subtle)',
            }}
          >
            GEN
          </div>
          <span className="text-xs font-medium" style={{ color: 'var(--text-primary)' }}>
            Gen.G
          </span>
        </div>
      </div>
    </div>
  )
}

function PlayerRow({
  name,
  role,
  kda,
  winRate,
}: {
  name: string
  role: string
  kda: string
  winRate: number
}) {
  return (
    <tr
      className="transition-colors"
      style={{ borderBottom: '1px solid var(--border-subtle)' }}
      onMouseEnter={(e) => {
        e.currentTarget.style.backgroundColor = 'var(--bg-elevated)'
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.backgroundColor = 'transparent'
      }}
    >
      <td className="py-2 px-3 text-sm font-medium" style={{ color: 'var(--text-primary)' }}>
        {name}
      </td>
      <td className="py-2 px-3 text-xs" style={{ color: 'var(--text-secondary)' }}>
        <span
          className="px-2 py-0.5 rounded"
          style={{
            backgroundColor: 'rgba(6, 182, 212, 0.1)',
            color: 'var(--color-accent-cyan)',
          }}
        >
          {role}
        </span>
      </td>
      <td
        className="py-2 px-3 text-sm tabular-nums text-right"
        style={{ color: 'var(--text-primary)' }}
      >
        {kda}
      </td>
      <td className="py-2 px-3 text-sm tabular-nums text-right">
        <span style={{ color: winRate >= 50 ? 'var(--color-win)' : 'var(--color-loss)' }}>
          {winRate}%
        </span>
      </td>
    </tr>
  )
}

interface RosterPreviewProps {
  players?: { name: string; role: string }[]
}

function RosterPreview({ players: playersProp }: RosterPreviewProps) {
  const players = (playersProp ?? [
    { name: 'Zeus', role: 'Top' },
    { name: 'Oner', role: 'Jungle' },
    { name: 'Faker', role: 'Mid' },
    { name: 'Gumayusi', role: 'Bot' },
    { name: 'Keria', role: 'Support' },
  ]).map((p) => ({ ...p, kda: '—', winRate: 0 }))

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
              className="text-right text-xs font-semibold uppercase tracking-wider py-2 px-3"
              style={{ color: 'var(--text-muted)' }}
            >
              KDA
            </th>
            <th
              className="text-right text-xs font-semibold uppercase tracking-wider py-2 px-3"
              style={{ color: 'var(--text-muted)' }}
            >
              Win%
            </th>
          </tr>
        </thead>
        <tbody>
          {players.map((p) => (
            <PlayerRow key={p.name} {...p} />
          ))}
        </tbody>
      </table>
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
  rosterPlayers?: { name: string; role: string }[]
}

export function Dashboard({
  record = '8-2',
  standing = '2nd place · LCK Spring',
  winStreak = 3,
  budget = '$1.2M',
  inboxCount = 3,
  urgentCount = 1,
  rosterPlayers,
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
        <div>
          <UpcomingMatch />
        </div>
      </div>
    </div>
  )
}
