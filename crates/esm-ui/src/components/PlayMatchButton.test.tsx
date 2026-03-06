import { describe, it, expect, vi } from 'vitest';
import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { renderWithProviders } from '@/test/render';
import { PlayMatchButton } from './PlayMatchButton';

describe('PlayMatchButton', () => {
  it('renders "Participate" as the default mode label', () => {
    renderWithProviders(<PlayMatchButton onConfirm={vi.fn()} />);
    expect(screen.getByRole('button', { name: /participate/i })).toBeInTheDocument();
  });

  it('shows dropdown menu when arrow button is clicked', async () => {
    const user = userEvent.setup();
    renderWithProviders(<PlayMatchButton onConfirm={vi.fn()} />);

    const arrow = screen.getByRole('button', { name: /match options/i });
    await user.click(arrow);

    expect(screen.getByText(/delegate to assistant/i)).toBeInTheDocument();
    expect(screen.getByText(/draft & delegate/i)).toBeInTheDocument();
    expect(screen.getByText(/watch auto-draft/i)).toBeInTheDocument();
    expect(screen.getByText(/full draft & live match/i)).toBeInTheDocument();
  });

  it('changes mode label when a dropdown option is selected', async () => {
    const user = userEvent.setup();
    renderWithProviders(<PlayMatchButton onConfirm={vi.fn()} />);

    await user.click(screen.getByRole('button', { name: /match options/i }));
    await user.click(screen.getByText(/delegate to assistant/i));

    // Main button label should now show "Delegate"
    expect(screen.getByRole('button', { name: /delegate/i })).toBeInTheDocument();
  });

  it('shows confirmation modal when main button is clicked', async () => {
    const user = userEvent.setup();
    renderWithProviders(<PlayMatchButton onConfirm={vi.fn()} />);

    await user.click(screen.getByRole('button', { name: /participate/i }));

    expect(screen.getByText(/confirm match/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /^confirm$/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /cancel/i })).toBeInTheDocument();
  });

  it('calls onConfirm with default mode when modal is confirmed', async () => {
    const onConfirm = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(<PlayMatchButton onConfirm={onConfirm} />);

    // Click the main button (first button with "Participate")
    const buttons = screen.getAllByRole('button');
    const mainBtn = buttons.find((b) => b.textContent?.includes('Participate'))!;
    await user.click(mainBtn);
    await user.click(screen.getByRole('button', { name: /^confirm$/i }));

    expect(onConfirm).toHaveBeenCalledWith('participate');
  });

  it('closes modal when cancel is clicked without calling onConfirm', async () => {
    const onConfirm = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(<PlayMatchButton onConfirm={onConfirm} />);

    await user.click(screen.getByRole('button', { name: /participate/i }));
    expect(screen.getByText(/confirm match/i)).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: /cancel/i }));
    expect(screen.queryByText(/confirm match/i)).not.toBeInTheDocument();
    expect(onConfirm).not.toHaveBeenCalled();
  });

  it('passes selected mode through confirmation flow', async () => {
    const onConfirm = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(<PlayMatchButton onConfirm={onConfirm} />);

    // Change to spectate
    await user.click(screen.getByRole('button', { name: /match options/i }));
    await user.click(screen.getByText(/spectate/i));

    // Click main button
    await user.click(screen.getByRole('button', { name: /spectate/i }));
    await user.click(screen.getByRole('button', { name: /^confirm$/i }));

    expect(onConfirm).toHaveBeenCalledWith('spectate');
  });

  it('closes dropdown when clicking outside', async () => {
    const user = userEvent.setup();
    renderWithProviders(
      <div>
        <PlayMatchButton onConfirm={vi.fn()} />
        <div data-testid="outside">outside</div>
      </div>
    );

    await user.click(screen.getByRole('button', { name: /match options/i }));
    expect(screen.getByText(/delegate to assistant/i)).toBeInTheDocument();

    await user.click(screen.getByTestId('outside'));
    expect(screen.queryByText(/delegate to assistant/i)).not.toBeInTheDocument();
  });
});
