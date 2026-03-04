import { useState } from 'react'
import { MainMenu } from '@/components/MainMenu'
import { NewGame } from '@/components/NewGame'
import { LoadGame } from '@/components/LoadGame'
import { TeamSelection } from '@/components/TeamSelection'
import { Settings } from '@/components/Settings'
import { GameShell } from '@/components/GameShell'
import { useListSaves, useDeleteSave, useLoadSave } from '@/lib/use-api'
import type { MenuTarget } from '@/components/MainMenu'
import type { TeamOption } from '@/components/TeamSelection'
import type { GameInfo } from '@/lib/api'

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
  const [gameInfo, setGameInfo] = useState<GameInfo | null>(null)

  // API hooks
  const { saves, loading: savesLoading, refresh: refreshSaves } = useListSaves()
  const { deleteSave } = useDeleteSave()
  const { loadSave } = useLoadSave()

  const handleMenuNavigate = (target: MenuTarget) => {
    if (target === 'exit') {
      // In Tauri this would close the window; for now, no-op
      return
    }
    if (target === 'load-game') {
      refreshSaves()
    }
    setScreen(target as AppScreen)
  }

  const goToMenu = () => setScreen('main-menu')

  const goToPlaying = (info?: GameInfo | null) => {
    if (info) setGameInfo(info)
    setScreen('playing')
  }

  const handleLoadSave = async (name: string) => {
    const info = await loadSave(name)
    if (info) goToPlaying(info)
  }

  const handleDeleteSave = async (name: string) => {
    await deleteSave(name)
    refreshSaves()
  }

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
      return (
        <LoadGame
          onBack={goToMenu}
          onLoad={handleLoadSave}
          saves={saves}
          loading={savesLoading}
          onDelete={handleDeleteSave}
        />
      )
    case 'settings':
      return <Settings onBack={goToMenu} />
    case 'playing':
      return (
        <GameShell
          teamName={gameInfo?.team_name}
          year={gameInfo?.year}
          month={gameInfo?.month}
          day={gameInfo?.day}
          onSaveAndExit={goToMenu}
        />
      )
    default:
      return <MainMenu onNavigate={handleMenuNavigate} />
  }
}

export default App
