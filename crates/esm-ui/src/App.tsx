import { useState } from 'react'
import { MainMenu } from '@/components/MainMenu'
import { NewGame } from '@/components/NewGame'
import { LoadGame } from '@/components/LoadGame'
import { GameShell } from '@/components/GameShell'
import type { MenuTarget } from '@/components/MainMenu'

type AppScreen = 'main-menu' | 'new-game' | 'load-game' | 'settings' | 'playing'

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
      return <NewGame onBack={goToMenu} onStart={goToPlaying} />
    case 'load-game':
      return <LoadGame onBack={goToMenu} onLoad={() => goToPlaying()} />
    case 'settings':
      return (
        <div
          className="flex items-center justify-center min-h-screen w-full"
          style={{ backgroundColor: 'var(--bg-base)', color: 'var(--text-muted)' }}
        >
          Settings — Coming soon
        </div>
      )
    case 'playing':
      return <GameShell />
    default:
      return <MainMenu onNavigate={handleMenuNavigate} />
  }
}

export default App
