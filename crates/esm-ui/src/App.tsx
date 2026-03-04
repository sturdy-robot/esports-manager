import { useState } from 'react'
import { MainMenu } from '@/components/MainMenu'
import { NewGame } from '@/components/NewGame'
import { LoadGame } from '@/components/LoadGame'
import { TeamSelection } from '@/components/TeamSelection'
import { Settings } from '@/components/Settings'
import { GameShell } from '@/components/GameShell'
import type { MenuTarget } from '@/components/MainMenu'
import type { TeamOption } from '@/components/TeamSelection'

type AppScreen =
  | 'main-menu'
  | 'new-game'
  | 'team-selection'
  | 'load-game'
  | 'settings'
  | 'playing'

// Placeholder teams for development until wired to Tauri backend
const PLACEHOLDER_TEAMS: TeamOption[] = [
  { name: 'T1', tag: 'T1', playerCount: 5, reputation: 92 },
  { name: 'Gen.G', tag: 'GEN', playerCount: 5, reputation: 88 },
  { name: 'DRX', tag: 'DRX', playerCount: 5, reputation: 72 },
  { name: 'KT Rolster', tag: 'KT', playerCount: 5, reputation: 68 },
  { name: 'Hanwha Life', tag: 'HLE', playerCount: 5, reputation: 75 },
  { name: 'Dplus KIA', tag: 'DK', playerCount: 5, reputation: 80 },
]

function App() {
  const [screen, setScreen] = useState<AppScreen>('main-menu')

  const handleMenuNavigate = (target: MenuTarget) => {
    if (target === 'exit') {
      // In Tauri this would close the window; for now, no-op
      return
    }
    setScreen(target as AppScreen)
  }

  const goToMenu = () => setScreen('main-menu')
  const goToPlaying = () => setScreen('playing')

  switch (screen) {
    case 'main-menu':
      return <MainMenu onNavigate={handleMenuNavigate} />
    case 'new-game':
      return (
        <NewGame
          onBack={goToMenu}
          onStart={() => setScreen('team-selection')}
        />
      )
    case 'team-selection':
      return (
        <TeamSelection
          teams={PLACEHOLDER_TEAMS}
          onBack={() => setScreen('new-game')}
          onSelect={() => goToPlaying()}
        />
      )
    case 'load-game':
      return <LoadGame onBack={goToMenu} onLoad={() => goToPlaying()} />
    case 'settings':
      return <Settings onBack={goToMenu} />
    case 'playing':
      return <GameShell onSaveAndExit={goToMenu} />
    default:
      return <MainMenu onNavigate={handleMenuNavigate} />
  }
}

export default App
