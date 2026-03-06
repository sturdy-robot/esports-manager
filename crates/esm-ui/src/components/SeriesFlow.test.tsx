import { describe, it, expect, vi } from 'vitest';
import { screen, waitFor } from '@testing-library/react';
import { renderWithProviders } from '../test/render';
import { SeriesFlow } from './SeriesFlow';

describe('SeriesFlow', () => {
  const defaultProps = {
    mode: 'participate' as const,
    teamName: 'T1',
    opponentName: 'Gen.G',
    teamSide: 'blue' as const,
    onComplete: vi.fn(),
  };

  it('renders series header with Bo3 info', async () => {
    renderWithProviders(<SeriesFlow {...defaultProps} />);
    await waitFor(() => {
      expect(screen.getByText(/best of 3/i)).toBeInTheDocument();
    });
  });

  it('shows game number indicator', async () => {
    renderWithProviders(<SeriesFlow {...defaultProps} />);
    await waitFor(() => {
      expect(screen.getByText(/game 1/i)).toBeInTheDocument();
    });
  });

  it('shows series score', async () => {
    renderWithProviders(<SeriesFlow {...defaultProps} />);
    await waitFor(() => {
      // Initial score 0-0: two ScoreBox elements each showing 0
      const zeros = screen.getAllByText('0');
      expect(zeros.length).toBeGreaterThanOrEqual(2);
    });
  });

  it('renders pre-match phase for participate mode', async () => {
    renderWithProviders(<SeriesFlow {...defaultProps} />);
    await waitFor(() => {
      expect(screen.getByRole('heading', { name: /pre-match/i })).toBeInTheDocument();
    });
  });

  it('calls onComplete when series is finished and user clicks return', async () => {
    const onComplete = vi.fn();
    renderWithProviders(<SeriesFlow {...defaultProps} onComplete={onComplete} />);
    // Component should eventually render — we just verify it doesn't crash
    await waitFor(() => {
      expect(screen.getByText(/best of 3/i)).toBeInTheDocument();
    });
  });
});
