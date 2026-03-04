import { Sun, Moon, Settings } from 'lucide-react'
import { useTheme } from '@/lib/use-theme'

interface TopBarProps {
  title: string
}

export function TopBar({ title }: TopBarProps) {
  const { theme, toggleTheme } = useTheme()

  return (
    <header
      className="flex items-center justify-between h-14 px-6 border-b shrink-0"
      style={{
        borderColor: 'var(--border-subtle)',
        backgroundColor: 'var(--bg-surface)',
      }}
    >
      <h1 className="text-xl font-bold" style={{ color: 'var(--text-primary)' }}>
        {title}
      </h1>

      <div className="flex items-center gap-3">
        {/* Game clock placeholder */}
        <div
          className="flex items-center gap-2 px-3 py-1 rounded-md text-sm"
          style={{
            backgroundColor: 'var(--bg-elevated)',
            color: 'var(--text-secondary)',
          }}
        >
          <span className="inline-block w-2 h-2 rounded-full animate-live" style={{ backgroundColor: 'var(--color-accent-cyan)' }} />
          Season 2025 · Week 1 · Day 1
        </div>

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

        {/* Settings */}
        <button
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
        >
          <Settings size={16} />
        </button>
      </div>
    </header>
  )
}
