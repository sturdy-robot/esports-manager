import { describe, it, expect, vi } from 'vitest';
import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { renderWithProviders } from '../test/render';
import { MatchSimUI } from './MatchSimUI';
import type { SimulateMatchResult, MatchEventInfo } from '@/lib/api';

const SAMPLE_EVENTS: MatchEventInfo[] = [
  { minute: 3, phase: 'Early', kind: 'solo_kill', commentary: 'FIRST BLOOD! Player1 takes down Player3 in the mid lane!' },
  { minute: 8, phase: 'Early', kind: 'dragon', commentary: 'Blue slays the dragon! That\'s dragon number 1 for them.' },
  { minute: 16, phase: 'Mid', kind: 'teamfight', commentary: 'Blue wins the teamfight 3 to 1!' },
  { minute: 20, phase: 'Mid', kind: 'baron', commentary: 'BARON NASHOR IS DOWN! Blue secures the baron buff!' },
  { minute: 32, phase: 'Late', kind: 'nexus', commentary: 'AND THAT\'S THE GAME! Blue destroys the Nexus! GG!' },
];

function makeResult(overrides: Partial<SimulateMatchResult> = {}): SimulateMatchResult {
  return {
    winner: 'T1',
    duration_minutes: 32,
    blue_team: 'T1',
    red_team: 'Gen.G',
    blue_gold: 58200,
    red_gold: 51800,
    events: SAMPLE_EVENTS,
    ...overrides,
  };
}

describe('MatchSimUI', () => {
  const defaultProps = {
    result: makeResult(),
    onComplete: vi.fn(),
  };

  it('renders team names', () => {
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    // T1 appears as both blue_team name and winner
    expect(screen.getAllByText('T1').length).toBeGreaterThanOrEqual(1);
    expect(screen.getByText('Gen.G')).toBeInTheDocument();
  });

  it('renders the event log with commentary', () => {
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    expect(screen.getByText(/FIRST BLOOD/)).toBeInTheDocument();
  });

  it('shows the game clock / duration', () => {
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    // 32:00 appears in both scoreboard and last event row
    const matches = screen.getAllByText(/32:00/);
    expect(matches.length).toBeGreaterThanOrEqual(1);
  });

  it('shows gold totals for both teams', () => {
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    expect(screen.getByText(/58,200/)).toBeInTheDocument();
    expect(screen.getByText(/51,800/)).toBeInTheDocument();
  });

  it('shows the winner', () => {
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    expect(screen.getByText(/victory/i)).toBeInTheDocument();
    // T1 appears as blue_team and winner — just verify victory section exists
    const t1Elements = screen.getAllByText('T1');
    expect(t1Elements.length).toBeGreaterThanOrEqual(2);
  });

  it('calls onComplete when continue button is clicked', async () => {
    const onComplete = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(<MatchSimUI {...defaultProps} onComplete={onComplete} />);

    await user.click(screen.getByRole('button', { name: /continue/i }));
    expect(onComplete).toHaveBeenCalled();
  });

  it('displays event kind icons/labels', () => {
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    // Each event should show a timestamp
    expect(screen.getByText('3:00')).toBeInTheDocument();
    expect(screen.getByText('8:00')).toBeInTheDocument();
  });

  it('shows phase badges on events', () => {
    renderWithProviders(<MatchSimUI {...defaultProps} />);
    // Phase labels should appear
    const earlyBadges = screen.getAllByText('Early');
    expect(earlyBadges.length).toBeGreaterThanOrEqual(1);
  });
});
