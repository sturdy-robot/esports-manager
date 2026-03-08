import { useState } from 'react'
import { MainMenu } from '@/components/MainMenu'
import { NewGame } from '@/components/NewGame'
import { LoadGame } from '@/components/LoadGame'
import { TeamSelection } from '@/components/TeamSelection'
import { Settings } from '@/components/Settings'
import { GameShell } from '@/components/GameShell'
import { SeriesFlow } from '@/components/SeriesFlow'
import { useListSaves, useDeleteSave, useLoadSave, useNewGame, useSaveGame, useAdvanceTurn } from '@/lib/use-api'
import { getGameInfo } from '@/lib/api'
import type { MenuTarget } from '@/components/MainMenu'
import type { ManagerFormData } from '@/components/NewGame'
import type { TeamOption } from '@/components/TeamSelection'
import type { GameInfo } from '@/lib/api'
import type { MatchMode } from '@/components/PlayMatchButton'
import type { ContinueMode } from '@/components/TopBar'

type AppScreen =
  | 'main-menu'
  | 'new-game'
  | 'team-selection'
  | 'load-game'
  | 'settings'
  | 'playing'
  | 'match-flow'

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
  const [managerData, setManagerData] = useState<ManagerFormData | null>(null)
  const [saveName, setSaveName] = useState<string | null>(null)
  const [appError, setAppError] = useState<string | null>(null)
  const [matchMode, setMatchMode] = useState<MatchMode>('participate')

  // API hooks
  const { saves, loading: savesLoading, refresh: refreshSaves } = useListSaves()
  const { deleteSave } = useDeleteSave()
  const { loadSave } = useLoadSave()
  const { createGame } = useNewGame()
  const { saveGame } = useSaveGame()
  const { advanceTurn } = useAdvanceTurn()

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

  const handleManagerCreated = (data: ManagerFormData) => {
    setManagerData(data)
    setScreen('team-selection')
  }

  const handleTeamSelected = async (teamIndex: number) => {
    if (!managerData) return
    setAppError(null)
    const name = `${managerData.nickname}_${Date.now()}`
    try {
      const info = await createGame({
        first_name: managerData.firstName,
        last_name: managerData.lastName,
        nickname: managerData.nickname,
        nationality: managerData.nationality,
        esport_type: managerData.esportType,
        datapack_path: 'data/sample_datapack.json',
        team_index: teamIndex,
        save_name: name,
      })
      if (!info) {
        setAppError('Failed to create new game. Check the console for details.')
        return
      }
      setSaveName(name)
      goToPlaying(info)
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e)
      console.error('new_game failed:', msg)
      setAppError(`Failed to create game: ${msg}`)
    }
  }

  const handleLoadSave = async (name: string) => {
    const info = await loadSave(name)
    if (info) {
      setSaveName(name)
      goToPlaying(info)
    }
  }

  const handleSave = async () => {
    if (!saveName) {
      console.error('Cannot save: no save name set')
      return
    }
    const ok = await saveGame(saveName)
    if (ok) {
      console.log('Game saved successfully as:', saveName)
    }
  }

  const handleContinue = async (mode: ContinueMode) => {
    const info = await advanceTurn(mode)
    if (info) {
      setGameInfo(info)
    }
  }

  const handleDeleteSave = async (name: string) => {
    await deleteSave(name)
    refreshSaves()
  }

  const handlePlayMatch = (mode: MatchMode) => {
    setMatchMode(mode)
    setScreen('match-flow')
  }

  const handleMatchComplete = async () => {
    const info = await getGameInfo()
    if (info) setGameInfo(info)
    setScreen('playing')
  }

  switch (screen) {
    case 'main-menu':
      return <MainMenu onNavigate={handleMenuNavigate} />
    case 'new-game':
      return (
        <NewGame
          onBack={goToMenu}
          onStart={handleManagerCreated}
        />
      )
    case 'team-selection':
      return (
        <>
          {appError && (
            <div style={{
              position: 'fixed', top: 16, left: '50%', transform: 'translateX(-50%)',
              zIndex: 9999, padding: '12px 24px', borderRadius: 8,
              backgroundColor: '#DC2626', color: '#fff', fontSize: 14, fontWeight: 500,
              boxShadow: '0 4px 12px rgba(0,0,0,0.3)', cursor: 'pointer',
            }} onClick={() => setAppError(null)}>
              {appError}
            </div>
          )}
          <TeamSelection
            teams={PLACEHOLDER_TEAMS}
            onBack={() => setScreen('new-game')}
            onSelect={handleTeamSelected}
          />
        </>
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
          dayOfWeek={gameInfo?.day_of_week}
          phase={gameInfo?.phase}
          isMatchDay={gameInfo?.is_match_day}
          onContinue={handleContinue}
          onPlayMatch={handlePlayMatch}
          onSave={handleSave}
          onExitToMenu={goToMenu}
        />
      )
    case 'match-flow':
      return (
        <SeriesFlow
          mode={matchMode}
          teamName={gameInfo?.team_name ?? 'Team'}
          opponentName="Opponent"
          teamSide="blue"
          onComplete={handleMatchComplete}
        />
      )
    default:
      return <MainMenu onNavigate={handleMenuNavigate} />
  }
}

export default App
