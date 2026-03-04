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
} from 'lucide-react'
import { useState } from 'react'

interface NavItem {
  icon: React.ReactNode
  label: string
  id: string
}

interface NavGroup {
  title: string
  items: NavItem[]
}

const navGroups: NavGroup[] = [
  {
    title: 'Team',
    items: [
      { icon: <LayoutDashboard size={20} />, label: 'Dashboard', id: 'dashboard' },
      { icon: <Inbox size={20} />, label: 'Inbox', id: 'inbox' },
      { icon: <Users size={20} />, label: 'Roster', id: 'roster' },
      { icon: <Calendar size={20} />, label: 'Schedule', id: 'schedule' },
    ],
  },
  {
    title: 'League',
    items: [
      { icon: <Trophy size={20} />, label: 'Standings', id: 'standings' },
      { icon: <FileText size={20} />, label: 'Results', id: 'results' },
    ],
  },
  {
    title: 'Management',
    items: [
      { icon: <DollarSign size={20} />, label: 'Finances', id: 'finances' },
      { icon: <UserCog size={20} />, label: 'Staff', id: 'staff' },
      { icon: <Search size={20} />, label: 'Scouting', id: 'scouting' },
    ],
  },
]

interface SidebarProps {
  activeItem: string
  onNavigate: (id: string) => void
  onExitToMenu?: () => void
}

export function Sidebar({ activeItem, onNavigate, onExitToMenu }: SidebarProps) {
  const [collapsed, setCollapsed] = useState(false)

  return (
    <aside
      className={`flex flex-col border-r transition-all ${collapsed ? 'w-14' : 'w-60'}`}
      style={{
        borderColor: 'var(--border-subtle)',
        backgroundColor: 'var(--bg-surface)',
      }}
    >
      {/* Logo */}
      <div
        className="flex items-center gap-2 px-4 h-14 border-b"
        style={{ borderColor: 'var(--border-subtle)' }}
      >
        {!collapsed && (
          <span className="text-base font-bold tracking-tight accent-gradient-text">
            ESM
          </span>
        )}
        {!collapsed && (
          <span
            className="text-xs font-medium"
            style={{ color: 'var(--text-secondary)' }}
          >
            eSports Manager
          </span>
        )}
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto py-2">
        {navGroups.map((group) => (
          <div key={group.title} className="mb-2">
            {!collapsed && (
              <div
                className="px-4 py-1 text-xs font-semibold uppercase tracking-wider"
                style={{ color: 'var(--text-muted)' }}
              >
                {group.title}
              </div>
            )}
            {group.items.map((item) => {
              const isActive = activeItem === item.id
              return (
                <button
                  key={item.id}
                  onClick={() => onNavigate(item.id)}
                  className={`flex items-center gap-3 w-full px-4 py-2 text-sm font-medium transition-colors cursor-pointer border-none outline-none ${collapsed ? 'justify-center' : ''}`}
                  style={{
                    backgroundColor: isActive ? 'var(--bg-elevated)' : 'transparent',
                    color: isActive ? 'var(--text-primary)' : 'var(--text-secondary)',
                    borderLeft: isActive
                      ? '3px solid var(--color-accent-cyan)'
                      : '3px solid transparent',
                  }}
                  onMouseEnter={(e) => {
                    if (!isActive) {
                      e.currentTarget.style.backgroundColor = 'var(--bg-surface)'
                      e.currentTarget.style.color = 'var(--text-primary)'
                    }
                  }}
                  onMouseLeave={(e) => {
                    if (!isActive) {
                      e.currentTarget.style.backgroundColor = 'transparent'
                      e.currentTarget.style.color = 'var(--text-secondary)'
                    }
                  }}
                >
                  {item.icon}
                  {!collapsed && <span>{item.label}</span>}
                </button>
              )
            })}
          </div>
        ))}
      </nav>

      {/* Exit to Menu */}
      {onExitToMenu && (
        <button
          onClick={onExitToMenu}
          className={`flex items-center gap-3 w-full px-4 py-2.5 text-sm font-medium cursor-pointer border-none outline-none transition-colors ${collapsed ? 'justify-center' : ''}`}
          style={{
            borderTop: '1px solid var(--border-subtle)',
            color: 'var(--text-secondary)',
            backgroundColor: 'transparent',
          }}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = 'var(--bg-elevated)'
            e.currentTarget.style.color = 'var(--color-loss)'
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = 'transparent'
            e.currentTarget.style.color = 'var(--text-secondary)'
          }}
        >
          <LogOut size={18} />
          {!collapsed && <span>Exit to Menu</span>}
        </button>
      )}

      {/* Collapse toggle */}
      <button
        onClick={() => setCollapsed(!collapsed)}
        className="flex items-center justify-center h-10 border-t cursor-pointer border-none"
        style={{
          borderTop: '1px solid var(--border-subtle)',
          color: 'var(--text-muted)',
          backgroundColor: 'transparent',
        }}
      >
        {collapsed ? <ChevronRight size={16} /> : <ChevronLeft size={16} />}
      </button>
    </aside>
  )
}
