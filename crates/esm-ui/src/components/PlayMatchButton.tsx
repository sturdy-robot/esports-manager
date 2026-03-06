import { useState, useRef, useEffect } from 'react';
import { Play, ChevronDown } from 'lucide-react';

export type MatchMode = 'delegate' | 'draft-delegate' | 'spectate' | 'participate';

interface MatchModeOption {
  value: MatchMode;
  label: string;
  description: string;
}

const MATCH_MODES: MatchModeOption[] = [
  { value: 'participate', label: 'Participate', description: 'Full draft & live match experience' },
  { value: 'spectate', label: 'Spectate', description: 'Watch auto-draft and live simulation' },
  { value: 'draft-delegate', label: 'Draft & Delegate', description: 'You draft, AI plays the match' },
  { value: 'delegate', label: 'Delegate to Assistant', description: 'AI handles draft and match instantly' },
];

interface PlayMatchButtonProps {
  onConfirm: (mode: MatchMode) => void;
}

export function PlayMatchButton({ onConfirm }: PlayMatchButtonProps) {
  const [mode, setMode] = useState<MatchMode>('participate');
  const [dropdownOpen, setDropdownOpen] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const current = MATCH_MODES.find((m) => m.value === mode)!;

  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setDropdownOpen(false);
      }
    }
    if (dropdownOpen) {
      document.addEventListener('mousedown', handleClickOutside);
    }
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, [dropdownOpen]);

  const handleSelect = (value: MatchMode) => {
    setMode(value);
    setDropdownOpen(false);
  };

  const handleMainClick = () => {
    setModalOpen(true);
  };

  const handleConfirm = () => {
    setModalOpen(false);
    onConfirm(mode);
  };

  const handleCancel = () => {
    setModalOpen(false);
  };

  return (
    <>
      <div ref={containerRef} className="relative inline-flex">
        {/* Main button */}
        <button
          onClick={handleMainClick}
          className="flex items-center gap-1.5 px-4 py-1.5 rounded-l-md text-xs font-semibold cursor-pointer border-none transition-all"
          style={{
            background: 'linear-gradient(135deg, var(--color-warning), var(--color-loss))',
            color: '#fff',
          }}
        >
          <Play size={12} />
          {current.label}
        </button>

        {/* Arrow dropdown toggle */}
        <button
          onClick={() => setDropdownOpen((prev) => !prev)}
          aria-label="Match options"
          className="flex items-center px-2 py-1.5 rounded-r-md text-xs cursor-pointer border-none border-l transition-all"
          style={{
            background: 'linear-gradient(135deg, var(--color-warning), var(--color-loss))',
            color: '#fff',
            borderLeft: '1px solid rgba(255,255,255,0.3)',
          }}
        >
          <ChevronDown size={12} />
        </button>

        {/* Dropdown menu */}
        {dropdownOpen && (
          <div
            className="absolute right-0 top-full mt-1 w-64 rounded-lg border shadow-lg z-50 overflow-hidden"
            style={{
              backgroundColor: 'var(--bg-surface)',
              borderColor: 'var(--border-subtle)',
            }}
          >
            {MATCH_MODES.map((opt) => (
              <button
                key={opt.value}
                onClick={() => handleSelect(opt.value)}
                className="w-full text-left px-4 py-3 cursor-pointer border-none transition-colors"
                style={{
                  backgroundColor: opt.value === mode ? 'var(--bg-elevated)' : 'transparent',
                  color: 'var(--text-primary)',
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.backgroundColor = 'var(--bg-elevated)';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.backgroundColor =
                    opt.value === mode ? 'var(--bg-elevated)' : 'transparent';
                }}
              >
                <div className="text-sm font-semibold">{opt.label}</div>
                <div className="text-xs mt-0.5" style={{ color: 'var(--text-secondary)' }}>
                  {opt.description}
                </div>
              </button>
            ))}
          </div>
        )}
      </div>

      {/* Confirmation modal */}
      {modalOpen && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center"
          style={{ backgroundColor: 'rgba(0, 0, 0, 0.6)' }}
        >
          <div
            className="w-full max-w-md rounded-xl border p-6"
            style={{
              backgroundColor: 'var(--bg-surface)',
              borderColor: 'var(--border-subtle)',
            }}
          >
            <h3
              className="text-lg font-bold mb-2"
              style={{ color: 'var(--text-primary)' }}
            >
              Confirm Match
            </h3>
            <p className="text-sm mb-6" style={{ color: 'var(--text-secondary)' }}>
              You are about to play the match in <strong style={{ color: 'var(--text-primary)' }}>{current.label}</strong> mode.
              {' '}{current.description}.
            </p>
            <div className="flex justify-end gap-3">
              <button
                onClick={handleCancel}
                className="px-4 py-2 rounded-md text-sm font-medium cursor-pointer border transition-colors"
                style={{
                  borderColor: 'var(--border-subtle)',
                  backgroundColor: 'transparent',
                  color: 'var(--text-secondary)',
                }}
              >
                Cancel
              </button>
              <button
                onClick={handleConfirm}
                className="px-4 py-2 rounded-md text-sm font-semibold cursor-pointer border-none transition-all"
                style={{
                  background: 'linear-gradient(135deg, var(--color-accent-cyan), var(--color-accent-violet))',
                  color: '#fff',
                }}
              >
                Confirm
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
