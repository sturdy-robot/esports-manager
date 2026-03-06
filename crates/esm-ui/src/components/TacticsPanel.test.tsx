import { describe, it, expect, vi } from 'vitest';
import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { renderWithProviders } from '../test/render';
import { TacticsPanel } from './TacticsPanel';
import type { TacticsInfo } from '@/lib/api';

const defaultTactics: TacticsInfo = { playstyle: 'balanced', focus: 'teamfight' };

describe('TacticsPanel', () => {
  it('renders playstyle and focus sections', () => {
    renderWithProviders(
      <TacticsPanel tactics={defaultTactics} onConfirm={vi.fn()} />
    );
    expect(screen.getByText('Playstyle')).toBeInTheDocument();
    expect(screen.getByText('Strategic Focus')).toBeInTheDocument();
  });

  it('renders all playstyle options', () => {
    renderWithProviders(
      <TacticsPanel tactics={defaultTactics} onConfirm={vi.fn()} />
    );
    expect(screen.getByText('Aggressive')).toBeInTheDocument();
    expect(screen.getByText('Balanced')).toBeInTheDocument();
    expect(screen.getByText('Defensive')).toBeInTheDocument();
  });

  it('renders all focus options', () => {
    renderWithProviders(
      <TacticsPanel tactics={defaultTactics} onConfirm={vi.fn()} />
    );
    expect(screen.getByText('Teamfight')).toBeInTheDocument();
    expect(screen.getByText('Splitpush')).toBeInTheDocument();
    expect(screen.getByText('Objective')).toBeInTheDocument();
  });

  it('calls onConfirm with selected values when confirm clicked', async () => {
    const onConfirm = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(
      <TacticsPanel tactics={defaultTactics} onConfirm={onConfirm} />
    );

    // Select aggressive
    await user.click(screen.getByText('Aggressive'));
    // Select objective
    await user.click(screen.getByText('Objective'));
    // Confirm
    await user.click(screen.getByText(/Confirm Tactics/));

    expect(onConfirm).toHaveBeenCalledWith('aggressive', 'objective');
  });

  it('calls onConfirm with defaults if unchanged', async () => {
    const onConfirm = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(
      <TacticsPanel tactics={defaultTactics} onConfirm={onConfirm} />
    );

    await user.click(screen.getByText(/Confirm Tactics/));
    expect(onConfirm).toHaveBeenCalledWith('balanced', 'teamfight');
  });

  it('shows context label when provided', () => {
    renderWithProviders(
      <TacticsPanel tactics={defaultTactics} onConfirm={vi.fn()} context="Pre-Match Strategy" />
    );
    expect(screen.getByText('Pre-Match Strategy')).toBeInTheDocument();
  });

  it('renders compact mode', () => {
    renderWithProviders(
      <TacticsPanel tactics={defaultTactics} onConfirm={vi.fn()} compact />
    );
    // Compact mode has "Confirm & Continue" instead of "Confirm Tactics & Proceed"
    expect(screen.getByText(/Confirm/)).toBeInTheDocument();
    expect(screen.getByText('Aggressive')).toBeInTheDocument();
  });

  it('initializes with provided tactics', async () => {
    const onConfirm = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(
      <TacticsPanel
        tactics={{ playstyle: 'aggressive', focus: 'splitpush' }}
        onConfirm={onConfirm}
      />
    );

    // Confirm without changing anything
    await user.click(screen.getByText(/Confirm Tactics/));
    expect(onConfirm).toHaveBeenCalledWith('aggressive', 'splitpush');
  });
});
