import { describe, it, expect, vi } from 'vitest';
import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { renderWithProviders } from '../test/render';
import { PlayerTalksPanel } from './PlayerTalksPanel';

describe('PlayerTalksPanel', () => {
  it('renders the Team Talk heading', () => {
    renderWithProviders(<PlayerTalksPanel onDone={vi.fn()} />);
    expect(screen.getByText('Team Talk')).toBeInTheDocument();
  });

  it('renders all five mock players', async () => {
    renderWithProviders(<PlayerTalksPanel onDone={vi.fn()} />);
    // Mock roster has 5 players: Zeus, Oner, Faker, Gumayusi, Keria
    expect(await screen.findByText('Zeus')).toBeInTheDocument();
    expect(screen.getByText('Oner')).toBeInTheDocument();
    expect(screen.getByText('Faker')).toBeInTheDocument();
    expect(screen.getByText('Gumayusi')).toBeInTheDocument();
    expect(screen.getByText('Keria')).toBeInTheDocument();
  });

  it('renders talk action buttons', async () => {
    renderWithProviders(<PlayerTalksPanel onDone={vi.fn()} />);
    await screen.findByText('Zeus');
    // Each player has 4 talk buttons; check at least the title attributes
    const motivateButtons = screen.getAllByTitle(/Motivate/);
    expect(motivateButtons.length).toBe(5); // one per player
  });

  it('renders the Continue button', () => {
    renderWithProviders(<PlayerTalksPanel onDone={vi.fn()} />);
    expect(screen.getByText('Continue to Next Game')).toBeInTheDocument();
  });

  it('calls onDone when Continue is clicked', async () => {
    const onDone = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(<PlayerTalksPanel onDone={onDone} />);
    await user.click(screen.getByText('Continue to Next Game'));
    expect(onDone).toHaveBeenCalledOnce();
  });

  it('shows confidence badges', async () => {
    renderWithProviders(<PlayerTalksPanel onDone={vi.fn()} />);
    await screen.findByText('Zeus');
    // Faker is "confident", Gumayusi is "slumping"
    expect(screen.getByText('Confident')).toBeInTheDocument();
    expect(screen.getByText('Slumping')).toBeInTheDocument();
  });

  it('disables all talk buttons for a player after one talk is applied', async () => {
    const user = userEvent.setup();
    renderWithProviders(<PlayerTalksPanel onDone={vi.fn()} />);
    await screen.findByText('Zeus');

    // Get all Motivate buttons (one per player)
    const motivateButtons = screen.getAllByTitle(/Motivate/);
    // Click the first player's Motivate
    await user.click(motivateButtons[0]);

    // After the talk, all 4 talk buttons for the first player should be disabled
    const firstPlayerRow = motivateButtons[0].closest('[data-testid="player-card"]');
    expect(firstPlayerRow).toBeTruthy();
    const buttonsInRow = firstPlayerRow!.querySelectorAll('button');
    const disabledButtons = Array.from(buttonsInRow).filter((b) => b.disabled);
    expect(disabledButtons.length).toBe(4);
  });
});
