import { useState } from 'react';
import { Bot } from 'lucide-react';

export function DraftUI() {
    const [activeTab, setActiveTab] = useState('bans');

    return (
        <div className="flex flex-col h-full animate-fade-in-up">
            <div className="flex items-center justify-between mb-6">
                <h2 className="text-2xl font-bold font-display" style={{ color: 'var(--text-primary)' }}>Draft Phase</h2>
                <div className="flex bg-elevated rounded-lg p-1" style={{ backgroundColor: 'var(--bg-elevated)' }}>
                    <button
                        onClick={() => setActiveTab('bans')}
                        className={`px-4 py-1.5 text-sm font-semibold rounded-md ${activeTab === 'bans' ? 'accent-gradient text-white' : ''}`}
                        style={{ color: activeTab === 'bans' ? '#fff' : 'var(--text-secondary)' }}
                    >
                        Bans
                    </button>
                    <button
                        onClick={() => setActiveTab('picks')}
                        className={`px-4 py-1.5 text-sm font-semibold rounded-md ${activeTab === 'picks' ? 'accent-gradient text-white' : ''}`}
                        style={{ color: activeTab === 'picks' ? '#fff' : 'var(--text-secondary)' }}
                    >
                        Picks
                    </button>
                </div>
            </div>

            <div
                className="flex-1 rounded-xl flex items-center justify-center border"
                style={{
                    backgroundColor: 'var(--bg-surface)',
                    borderColor: 'var(--border-subtle)',
                }}
            >
                <div className="text-center max-w-sm">
                    <Bot size={48} className="mx-auto mb-4" style={{ color: 'var(--color-accent-cyan)' }} />
                    <h3 className="text-xl font-bold mb-2" style={{ color: 'var(--text-primary)' }}>Draft Interface Pending</h3>
                    <p className="text-sm" style={{ color: 'var(--text-secondary)' }}>
                        The interactive Draft UI is under construction. It will feature real-time champion selection, timers, and state machine transitions hooked to the Rust core.
                    </p>
                </div>
            </div>
        </div>
    )
}
