import { describe, it, expect, vi } from 'vitest';
import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { renderWithProviders } from '../test/render';
import { DraftUI } from './DraftUI';
import type { DraftSessionState } from '@/lib/api';

const SAMPLE_CHAMPIONS = [
  'Orianna', 'Azir', 'Ahri', 'Syndra', 'Zed',
  'Malphite', 'Ornn', 'Gnar', 'Fiora', 'Jayce',
  'Lee Sin', 'Viego', 'Jarvan IV', 'Jinx', "Kai'Sa",
  'Ezreal', 'Aphelios', 'Thresh', 'Nautilus', 'Lulu',
];

function makeDraftState(overrides: Partial<DraftSessionState> = {}): DraftSessionState {
  return {
    current_step: 0,
    total_steps: 20,
    current_phase: 'Ban',
    current_team: 'Blue',
    blue_bans: [],
    red_bans: [],
    blue_picks: [],
    red_picks: [],
    active_hover: null,
    is_complete: false,
    is_player_turn: true,
    available_champions: [...SAMPLE_CHAMPIONS],
    ...overrides,
  };
}

describe('DraftUI', () => {
  const defaultProps = {
    draftState: makeDraftState(),
    playerSide: 'blue' as const,
    teamName: 'T1',
    opponentName: 'Gen.G',
    onHover: vi.fn(),
    onLock: vi.fn(),
    onComplete: vi.fn(),
  };

  it('renders team names for both sides', () => {
    renderWithProviders(<DraftUI {...defaultProps} />);
    expect(screen.getByText('T1')).toBeInTheDocument();
    expect(screen.getByText('Gen.G')).toBeInTheDocument();
  });

  it('renders the champion grid with available champions', () => {
    renderWithProviders(<DraftUI {...defaultProps} />);
    expect(screen.getByText('Orianna')).toBeInTheDocument();
    expect(screen.getByText('Azir')).toBeInTheDocument();
    expect(screen.getByText('Lulu')).toBeInTheDocument();
  });

  it('shows current phase indicator (Ban/Pick)', () => {
    renderWithProviders(<DraftUI {...defaultProps} />);
    expect(screen.getByText(/ban phase/i)).toBeInTheDocument();
  });

  it('shows pick phase when current_phase is Pick', () => {
    const state = makeDraftState({ current_phase: 'Pick', current_step: 6 });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} />);
    expect(screen.getByText(/pick phase/i)).toBeInTheDocument();
  });

  it('highlights the active hover champion', () => {
    const state = makeDraftState({ active_hover: 'Orianna' });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} />);
    const champButton = screen.getByRole('button', { name: /orianna/i });
    expect(champButton).toHaveAttribute('data-hovered', 'true');
  });

  it('calls onHover when a champion is clicked', async () => {
    const onHover = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(<DraftUI {...defaultProps} onHover={onHover} />);

    await user.click(screen.getByRole('button', { name: /orianna/i }));
    expect(onHover).toHaveBeenCalledWith('Orianna');
  });

  it('shows lock button when a champion is hovered', () => {
    const state = makeDraftState({ active_hover: 'Orianna' });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} />);
    expect(screen.getByRole('button', { name: /lock in/i })).toBeInTheDocument();
  });

  it('calls onLock when lock button is clicked', async () => {
    const onLock = vi.fn();
    const user = userEvent.setup();
    const state = makeDraftState({ active_hover: 'Orianna' });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} onLock={onLock} />);

    await user.click(screen.getByRole('button', { name: /lock in/i }));
    expect(onLock).toHaveBeenCalled();
  });

  it('disables champion buttons when it is not player turn', () => {
    const state = makeDraftState({ is_player_turn: false });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} />);
    const champButton = screen.getByRole('button', { name: /orianna/i });
    expect(champButton).toBeDisabled();
  });

  it('displays bans for both teams', () => {
    const state = makeDraftState({
      blue_bans: ['Orianna', 'Azir'],
      red_bans: ['Syndra'],
      current_step: 5,
      available_champions: SAMPLE_CHAMPIONS.filter(c => !['Orianna', 'Azir', 'Syndra'].includes(c)),
    });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} />);
    // Ban slots should show the banned champion names
    expect(screen.getByText('Orianna')).toBeInTheDocument();
    expect(screen.getByText('Azir')).toBeInTheDocument();
    expect(screen.getByText('Syndra')).toBeInTheDocument();
  });

  it('displays picks for both teams', () => {
    const state = makeDraftState({
      blue_picks: ['Malphite', 'Lee Sin'],
      red_picks: ['Fiora'],
      current_step: 9,
      current_phase: 'Pick',
      available_champions: SAMPLE_CHAMPIONS.filter(c => !['Malphite', 'Lee Sin', 'Fiora'].includes(c)),
    });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} />);
    expect(screen.getByText('Malphite')).toBeInTheDocument();
    expect(screen.getByText('Lee Sin')).toBeInTheDocument();
    expect(screen.getByText('Fiora')).toBeInTheDocument();
  });

  it('shows draft complete state and calls onComplete', async () => {
    const onComplete = vi.fn();
    const user = userEvent.setup();
    const state = makeDraftState({
      is_complete: true,
      current_phase: null,
      current_team: null,
      blue_bans: ['Orianna', 'Azir', 'Ahri', 'Syndra', 'Zed'],
      red_bans: ['Malphite', 'Ornn', 'Gnar', 'Fiora', 'Jayce'],
      blue_picks: ['Lee Sin', 'Viego', 'Jinx', 'Thresh', 'Lulu'],
      red_picks: ['Jarvan IV', "Kai'Sa", 'Ezreal', 'Aphelios', 'Nautilus'],
      available_champions: [],
    });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} onComplete={onComplete} />);

    expect(screen.getByText(/draft complete/i)).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /continue/i }));
    expect(onComplete).toHaveBeenCalled();
  });

  it('shows step progress indicator', () => {
    const state = makeDraftState({ current_step: 5, total_steps: 20 });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} />);
    expect(screen.getByText('5 / 20')).toBeInTheDocument();
  });

  it('uses flex-1 layout to fill available space instead of h-full', () => {
    const { container } = renderWithProviders(<DraftUI {...defaultProps} />);
    const root = container.firstElementChild as HTMLElement;
    expect(root.className).toContain('flex-1');
    expect(root.className).toContain('min-h-0');
    expect(root.className).not.toContain('h-full');
  });

  it('shows turn indicator on the active team', () => {
    const state = makeDraftState({ current_team: 'Blue' });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} />);
    const blueTeam = screen.getByTestId('team-header-blue');
    expect(blueTeam.getAttribute('data-active')).toBe('true');
    const redTeam = screen.getByTestId('team-header-red');
    expect(redTeam.getAttribute('data-active')).toBe('false');
  });

  it('shows turn indicator on red team when it is red turn', () => {
    const state = makeDraftState({ current_team: 'Red' });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} />);
    const redTeam = screen.getByTestId('team-header-red');
    expect(redTeam.getAttribute('data-active')).toBe('true');
    const blueTeam = screen.getByTestId('team-header-blue');
    expect(blueTeam.getAttribute('data-active')).toBe('false');
  });
});
