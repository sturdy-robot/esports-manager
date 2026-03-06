import { Users, Bot, CirclePlay } from 'lucide-react'

interface MatchInfo {
    homeTeam: string;
    awayTeam: string;
    day: number;
}

interface MatchLobbyProps {
    match?: MatchInfo;
    teamName: string;
    simulating?: boolean;
    onDelegate?: () => void;
    onDraft?: () => void;
}

export function MatchLobby({ match, teamName, simulating, onDelegate, onDraft }: MatchLobbyProps) {
    if (!match) {
        return (
            <div className="flex items-center justify-center h-64">
                <span className="text-lg" style={{ color: "var(--text-muted)" }}>
                    No pending match found.
                </span>
            </div>
        );
    }

    const isHome = match.homeTeam === teamName;
    const opponent = isHome ? match.awayTeam : match.homeTeam;
    const side = isHome ? "Blue Side" : "Red Side";

    return (
        <div className="flex justify-center mt-10 animate-fade-in-up">
            <div
                className="w-full max-w-4xl p-8 rounded-xl border"
                style={{
                    backgroundColor: 'var(--bg-surface)',
                    borderColor: 'var(--border-subtle)',
                }}
            >
                <div className="text-center mb-10">
                    <div className="text-xs font-bold uppercase tracking-widest mb-4" style={{ color: 'var(--color-accent-cyan)' }}>
                        Lck Match Day {match.day}
                    </div>
                    <div className="flex items-center justify-center gap-12">
                        <div className="flex flex-col items-center gap-2">
                            <div
                                className="w-24 h-24 rounded-full flex items-center justify-center text-2xl font-black accent-gradient"
                                style={{ color: '#fff', boxShadow: '0 0 20px rgba(6, 182, 212, 0.3)' }}
                            >
                                {teamName}
                            </div>
                            <span className="text-lg font-bold" style={{ color: 'var(--text-primary)' }}>{teamName}</span>
                            <span className="text-xs font-semibold px-2 py-0.5 rounded" style={{ backgroundColor: 'rgba(6,182,212,0.1)', color: 'var(--color-accent-cyan)' }}>{side}</span>
                        </div>

                        <div className="text-3xl font-black" style={{ color: 'var(--border-subtle)' }}>VS</div>

                        <div className="flex flex-col items-center gap-2">
                            <div
                                className="w-24 h-24 rounded-full flex items-center justify-center text-2xl font-black"
                                style={{ backgroundColor: 'var(--bg-elevated)', color: 'var(--text-primary)', border: '2px solid var(--border-subtle)' }}
                            >
                                {opponent}
                            </div>
                            <span className="text-lg font-bold" style={{ color: 'var(--text-primary)' }}>{opponent}</span>
                            <span className="text-xs font-semibold px-2 py-0.5 rounded" style={{ backgroundColor: 'var(--bg-elevated)', color: 'var(--text-muted)' }}>{isHome ? "Red Side" : "Blue Side"}</span>
                        </div>
                    </div>
                </div>

                <div className="grid grid-cols-3 gap-6 pt-6 border-t" style={{ borderColor: 'var(--border-subtle)' }}>
                    <button
                        onClick={onDraft}
                        className="flex flex-col items-center gap-4 p-6 rounded-lg cursor-pointer transition-transform hover:-translate-y-1"
                        style={{ backgroundColor: 'var(--bg-elevated)', border: '1px solid var(--color-accent-cyan)' }}
                    >
                        <div className="p-4 rounded-full" style={{ backgroundColor: 'rgba(6, 182, 212, 0.1)', color: 'var(--color-accent-cyan)' }}>
                            <Users size={32} />
                        </div>
                        <div className="text-center">
                            <div className="font-bold mb-1" style={{ color: 'var(--text-primary)' }}>Participate</div>
                            <div className="text-xs" style={{ color: 'var(--text-secondary)' }}>You manage the draft phase manually.</div>
                        </div>
                    </button>

                    <button
                        className="flex flex-col items-center gap-4 p-6 rounded-lg cursor-not-allowed opacity-50 transition-none border shadow-none"
                        style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-subtle)' }}
                    >
                        <div className="p-4 rounded-full" style={{ backgroundColor: 'var(--bg-base)', color: 'var(--text-muted)' }}>
                            <CirclePlay size={32} />
                        </div>
                        <div className="text-center">
                            <div className="font-bold mb-1" style={{ color: 'var(--text-primary)' }}>Spectate</div>
                            <div className="text-xs" style={{ color: 'var(--text-secondary)' }}>Watch the Auto-Draft and simulation.</div>
                        </div>
                    </button>

                    <button
                        onClick={onDelegate}
                        disabled={simulating}
                        className="flex flex-col items-center gap-4 p-6 rounded-lg cursor-pointer transition-transform hover:-translate-y-1 border"
                        style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-subtle)' }}
                    >
                        <div className="p-4 rounded-full" style={{ backgroundColor: 'rgba(234, 179, 8, 0.1)', color: 'var(--color-warning)' }}>
                            <Bot size={32} />
                        </div>
                        <div className="text-center">
                            <div className="font-bold mb-1" style={{ color: 'var(--text-primary)' }}>Delegate</div>
                            <div className="text-xs" style={{ color: 'var(--text-secondary)' }}>{simulating ? "Simulating..." : "Auto-resolve Draft and Match instantly."}</div>
                        </div>
                    </button>
                </div>
            </div>
        </div>
    );
}
