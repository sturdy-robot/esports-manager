import { describe, it, expect, vi } from 'vitest';
import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { renderWithProviders } from '@/test/render';
import { MatchFlow } from './MatchFlow';
import type { MatchMode } from './PlayMatchButton';

const defaultProps = {
  mode: 'participate' as MatchMode,
  teamName: 'T1',
  opponentName: 'Gen.G',
  teamSide: 'blue' as const,
  onComplete: vi.fn(),
};

describe('MatchFlow', () => {
  it('renders without sidebar or top bar (full-screen)', () => {
    renderWithProviders(<MatchFlow {...defaultProps} />);
    // Should NOT have sidebar or topbar elements
    expect(screen.queryByText(/dashboard/i)).not.toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /continue/i })).not.toBeInTheDocument();
  });

  it('shows the team matchup header', () => {
    renderWithProviders(<MatchFlow {...defaultProps} />);
    expect(screen.getByText('T1')).toBeInTheDocument();
    expect(screen.getByText('Gen.G')).toBeInTheDocument();
  });

  it('starts in pre-match phase for participate mode', () => {
    renderWithProviders(<MatchFlow {...defaultProps} mode="participate" />);
    expect(screen.getByRole('heading', { name: /pre-match/i })).toBeInTheDocument();
  });

  it('shows draft phase when user proceeds from pre-match in participate mode', async () => {
    const user = userEvent.setup();
    renderWithProviders(<MatchFlow {...defaultProps} mode="participate" />);

    await user.click(screen.getByRole('button', { name: /proceed to draft/i }));
    // DraftUI renders "Ban Phase" as the initial phase indicator
    await waitFor(() => {
      expect(screen.getByText(/ban phase/i)).toBeInTheDocument();
    });
  });

  it('skips directly to simulating in delegate mode', () => {
    renderWithProviders(<MatchFlow {...defaultProps} mode="delegate" />);
    expect(screen.getByRole('heading', { name: /simulating/i })).toBeInTheDocument();
  });

  it('shows draft then skips match in draft-delegate mode', async () => {
    const user = userEvent.setup();
    renderWithProviders(<MatchFlow {...defaultProps} mode="draft-delegate" />);
    // Starts at pre-match, proceed to draft
    expect(screen.getByRole('heading', { name: /pre-match/i })).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: /proceed to draft/i }));
    await waitFor(() => {
      expect(screen.getByText(/ban phase/i)).toBeInTheDocument();
    });
  });

  it('calls onComplete when user exits match flow', async () => {
    const onComplete = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(<MatchFlow {...defaultProps} mode="delegate" onComplete={onComplete} />);

    // In delegate mode, results should show quickly, then exit button
    const exitBtn = await screen.findByRole('button', { name: /return to dashboard/i });
    await user.click(exitBtn);
    expect(onComplete).toHaveBeenCalledTimes(1);
  });

  it('displays the side assignment', () => {
    renderWithProviders(<MatchFlow {...defaultProps} teamSide="blue" />);
    expect(screen.getByText(/blue side/i)).toBeInTheDocument();
  });
});
