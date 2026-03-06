import { describe, it, expect, vi } from 'vitest';
import { screen, fireEvent } from '@testing-library/react';
import { renderWithProviders } from '../test/render';
import { MatchLobby } from './MatchLobby';

describe('MatchLobby', () => {
    it('renders "No pending match" when no match is provided', () => {
        renderWithProviders(<MatchLobby teamName="T1" />);
        expect(screen.getByText(/no pending match found/i)).toBeInTheDocument();
    });

    it('renders match details correctly', () => {
        const match = { homeTeam: 'T1', awayTeam: 'Gen.G', day: 4 };
        renderWithProviders(<MatchLobby match={match} teamName="T1" />);
        expect(screen.getByText(/lck match day 4/i)).toBeInTheDocument();
    });

    it('calls onDelegate when Delegate button is clicked', () => {
        const onDelegate = vi.fn();
        const match = { homeTeam: 'T1', awayTeam: 'Gen.G', day: 4 };
        renderWithProviders(<MatchLobby match={match} teamName="T1" onDelegate={onDelegate} />);

        // Using role or text
        const delegateBtn = screen.getByText(/delegate/i).closest('button');
        if (delegateBtn) {
            fireEvent.click(delegateBtn);
        }

        expect(onDelegate).toHaveBeenCalledTimes(1);
    });

    it('calls onDraft when Participate button is clicked', () => {
        const onDraft = vi.fn();
        const match = { homeTeam: 'T1', awayTeam: 'Gen.G', day: 4 };
        renderWithProviders(<MatchLobby match={match} teamName="T1" onDraft={onDraft} />);

        const draftBtn = screen.getByText(/participate/i).closest('button');
        if (draftBtn) {
            fireEvent.click(draftBtn);
        }

        expect(onDraft).toHaveBeenCalledTimes(1);
    });
});
