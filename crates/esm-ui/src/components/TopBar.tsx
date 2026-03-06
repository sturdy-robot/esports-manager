import { Sun, Moon, Play, Save } from 'lucide-react'
import { useTheme } from '@/lib/use-theme'

const MONTH_NAMES = [
  'Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun',
  'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec',
]

interface TopBarProps {
  title: string
  year?: number
  month?: number
  day?: number
  phase?: string
  teamName?: string
  isMatchDay?: boolean
  onSave?: () => void
  onContinue?: () => void
  onPlayMatch?: () => void
}

export function TopBar({
  title,
  year = 2025,
  month = 1,
  day = 1,
  phase = 'Morning',
  teamName,
  isMatchDay = false,
  onSave,
  onContinue,
  onPlayMatch,
}: TopBarProps) {
  const { theme, toggleTheme } = useTheme()
  const monthLabel = MONTH_NAMES[(month - 1) % 12]

  return (
    <header
      className="flex items-center justify-between h-14 px-6 border-b shrink-0"
      style={{
        borderColor: 'var(--border-subtle)',
        backgroundColor: 'var(--bg-surface)',
      }}
    >
      {/* Left: title + team badge */}
      <div className="flex items-center gap-3">
        <h1 className="text-xl font-bold" style={{ color: 'var(--text-primary)' }}>
          {title}
        </h1>
        {teamName && (
          <span
            className="px-2 py-0.5 rounded text-xs font-semibold"
            style={{
              backgroundColor: 'var(--bg-elevated)',
              color: 'var(--color-accent-cyan)',
              border: '1px solid var(--border-subtle)',
            }}
          >
            {teamName}
          </span>
        )}
      </div>

      {/* Right: game clock + actions */}
      <div className="flex items-center gap-3">
        {/* Game clock */}
        <div
          className="flex items-center gap-2 px-3 py-1 rounded-md text-sm"
          style={{
            backgroundColor: 'var(--bg-elevated)',
            color: 'var(--text-secondary)',
          }}
        >
          <span
            className="inline-block w-2 h-2 rounded-full animate-live"
            style={{ backgroundColor: 'var(--color-accent-cyan)' }}
          />
          <span className="font-mono">
            {monthLabel} {day}, {year}
          </span>
          <span style={{ color: 'var(--text-muted)' }}>·</span>
          <span>{phase}</span>
        </div>

        {/* Save */}
        {onSave && (
          <button
            onClick={onSave}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-medium cursor-pointer border transition-colors"
            style={{
              borderColor: 'var(--border-subtle)',
              backgroundColor: 'transparent',
              color: 'var(--text-secondary)',
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.borderColor = 'var(--color-accent-cyan)'
              e.currentTarget.style.color = 'var(--text-primary)'
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.borderColor = 'var(--border-subtle)'
              e.currentTarget.style.color = 'var(--text-secondary)'
            }}
          >
            <Save size={12} />
            Save
          </button>
        )}

        {/* Theme toggle */}
        <button
          onClick={toggleTheme}
          className="flex items-center justify-center w-8 h-8 rounded-md cursor-pointer border transition-colors"
          style={{
            borderColor: 'var(--border-subtle)',
            backgroundColor: 'transparent',
            color: 'var(--text-secondary)',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.borderColor = 'var(--color-accent-cyan)'
            e.currentTarget.style.color = 'var(--text-primary)'
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.borderColor = 'var(--border-subtle)'
            e.currentTarget.style.color = 'var(--text-secondary)'
          }}
          title={`Switch to ${theme === 'dark' ? 'light' : 'dark'} mode`}
        >
          {theme === 'dark' ? <Sun size={16} /> : <Moon size={16} />}
        </button>

        {/* Play Match or Continue — always rightmost */}
        {isMatchDay && onPlayMatch ? (
          <button
            onClick={onPlayMatch}
            className="flex items-center gap-1.5 px-4 py-1.5 rounded-md text-xs font-semibold cursor-pointer border-none transition-all"
            style={{
              background: 'linear-gradient(135deg, var(--color-warning), var(--color-loss))',
              color: '#fff',
            }}
          >
            <Play size={12} />
            Play Match
          </button>
        ) : onContinue ? (
          <button
            onClick={onContinue}
            className="flex items-center gap-1.5 px-4 py-1.5 rounded-md text-xs font-semibold cursor-pointer border-none transition-all"
            style={{
              background: 'linear-gradient(135deg, var(--color-accent-cyan), var(--color-accent-violet))',
              color: '#fff',
            }}
          >
            <Play size={12} />
            Continue
          </button>
        ) : null}
      </div>
    </header>
  )
}
