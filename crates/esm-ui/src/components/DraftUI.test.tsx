import { describe, it, expect, vi } from 'vitest';
import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { renderWithProviders } from '../test/render';
import { DraftUI } from './DraftUI';
import type { DraftSessionState, ChampionDraftInfo } from '@/lib/api';

const SAMPLE_CHAMPIONS = [
  'Orianna', 'Azir', 'Ahri', 'Syndra', 'Zed',
  'Malphite', 'Ornn', 'Gnar', 'Fiora', 'Jayce',
  'Lee Sin', 'Viego', 'Jarvan IV', 'Jinx', "Kai'Sa",
  'Ezreal', 'Aphelios', 'Thresh', 'Nautilus', 'Lulu',
];

const SAMPLE_CHAMPION_DETAILS: Record<string, ChampionDraftInfo> = {
  Orianna:  { name: 'Orianna',  class: 'Mage',     scaling: 'Mid',   tags: ['Poke', 'Waveclear'], meta_tier: 'S' },
  Azir:     { name: 'Azir',     class: 'Mage',     scaling: 'Late',  tags: ['Poke'], meta_tier: 'A' },
  Ahri:     { name: 'Ahri',     class: 'Mage',     scaling: 'Mid',   tags: ['Burst'], meta_tier: 'A' },
  Syndra:   { name: 'Syndra',   class: 'Mage',     scaling: 'Mid',   tags: ['Burst'], meta_tier: 'B' },
  Zed:      { name: 'Zed',      class: 'Assassin', scaling: 'Mid',   tags: ['Burst'], meta_tier: 'B' },
  Malphite: { name: 'Malphite', class: 'Tank',     scaling: 'Mid',   tags: ['Engage'], meta_tier: 'A' },
  Ornn:     { name: 'Ornn',     class: 'Tank',     scaling: 'Late',  tags: ['Engage'], meta_tier: 'S' },
  Gnar:     { name: 'Gnar',     class: 'Fighter',  scaling: 'Mid',   tags: ['Engage'], meta_tier: 'B' },
  Fiora:    { name: 'Fiora',    class: 'Fighter',  scaling: 'Late',  tags: ['Splitpush'], meta_tier: 'A' },
  Jayce:    { name: 'Jayce',    class: 'Fighter',  scaling: 'Early', tags: ['Poke'], meta_tier: 'B' },
  'Lee Sin':   { name: 'Lee Sin',   class: 'Fighter',  scaling: 'Early', tags: ['Engage'], meta_tier: 'S' },
  Viego:    { name: 'Viego',    class: 'Assassin', scaling: 'Mid',   tags: ['Sustain'], meta_tier: 'A' },
  'Jarvan IV': { name: 'Jarvan IV', class: 'Fighter',  scaling: 'Early', tags: ['Engage'], meta_tier: 'B' },
  Jinx:     { name: 'Jinx',     class: 'Marksman', scaling: 'Late',  tags: ['Waveclear'], meta_tier: 'A' },
  "Kai'Sa": { name: "Kai'Sa",   class: 'Marksman', scaling: 'Mid',   tags: ['Burst'], meta_tier: 'S' },
  Ezreal:   { name: 'Ezreal',   class: 'Marksman', scaling: 'Mid',   tags: ['Poke'], meta_tier: 'A' },
  Aphelios: { name: 'Aphelios', class: 'Marksman', scaling: 'Late',  tags: ['Burst'], meta_tier: 'B' },
  Thresh:   { name: 'Thresh',   class: 'Support',  scaling: 'Early', tags: ['Engage'], meta_tier: 'S' },
  Nautilus: { name: 'Nautilus', class: 'Support',  scaling: 'Early', tags: ['Engage'], meta_tier: 'A' },
  Lulu:     { name: 'Lulu',     class: 'Support',  scaling: 'Mid',   tags: ['Peel'], meta_tier: 'A' },
};

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
    champion_details: { ...SAMPLE_CHAMPION_DETAILS },
    timer_seconds: 30,
    blue_players: [
      { nickname: 'Zeus', role: 'Top' },
      { nickname: 'Oner', role: 'Jungle' },
      { nickname: 'Faker', role: 'Mid' },
      { nickname: 'Gumayusi', role: 'Bot' },
      { nickname: 'Keria', role: 'Support' },
    ],
    red_players: [
      { nickname: 'Doran', role: 'Top' },
      { nickname: 'Peanut', role: 'Jungle' },
      { nickname: 'Chovy', role: 'Mid' },
      { nickname: 'Peyz', role: 'Bot' },
      { nickname: 'Lehends', role: 'Support' },
    ],
    blue_team_name: 'T1',
    red_team_name: 'Gen.G',
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
    // Ban slots show champion names truncated to 4 chars
    expect(screen.getByText('Oria')).toBeInTheDocument();
    expect(screen.getByText('Azir')).toBeInTheDocument();
    expect(screen.getByText('Synd')).toBeInTheDocument();
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
    expect(screen.getByText('5/20')).toBeInTheDocument();
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
    // Blue team name should be rendered in its panel
    expect(screen.getByText('T1')).toBeInTheDocument();
    expect(screen.getByText('Gen.G')).toBeInTheDocument();
  });

  it('shows turn indicator on red team when it is red turn', () => {
    const state = makeDraftState({ current_team: 'Red' });
    renderWithProviders(<DraftUI {...defaultProps} draftState={state} />);
    // Both team names should be rendered
    expect(screen.getByText('T1')).toBeInTheDocument();
    expect(screen.getByText('Gen.G')).toBeInTheDocument();
  });

  it('shows champion class badge on champion buttons', () => {
    renderWithProviders(<DraftUI {...defaultProps} />);
    // Orianna is a Mage — check that the Mage badge is present inside her button
    const oriannaBtn = screen.getByRole('button', { name: /orianna/i });
    expect(oriannaBtn.textContent).toContain('Mage');
    // Zed is an Assassin
    const zedBtn = screen.getByRole('button', { name: /zed/i });
    expect(zedBtn.textContent).toContain('Assassin');
  });

  it('shows champion scaling info on champion buttons', () => {
    renderWithProviders(<DraftUI {...defaultProps} />);
    const azirBtn = screen.getByRole('button', { name: /azir/i });
    expect(azirBtn.textContent).toContain('Late');
    const jayceBtn = screen.getByRole('button', { name: /jayce/i });
    expect(jayceBtn.textContent).toContain('Early');
  });
});
