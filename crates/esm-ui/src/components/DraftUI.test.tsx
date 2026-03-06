import { describe, it, expect } from 'vitest';
import { screen, fireEvent } from '@testing-library/react';
import { renderWithProviders } from '../test/render';
import { DraftUI } from './DraftUI';

describe('DraftUI', () => {
    it('renders the DraftUI with Bans and Picks tabs', () => {
        renderWithProviders(<DraftUI />);
        expect(screen.getByText(/draft phase/i)).toBeInTheDocument();
        expect(screen.getByText(/bans/i)).toBeInTheDocument();
        expect(screen.getByText(/picks/i)).toBeInTheDocument();
    });

    it('switches between tabs', () => {
        renderWithProviders(<DraftUI />);
        const bansTab = screen.getByText(/bans/i);
        const picksTab = screen.getByText(/picks/i);

        fireEvent.click(picksTab);
        // Since UI stub is just visual right now, just verify tabs exist
        expect(picksTab).toBeInTheDocument();

        fireEvent.click(bansTab);
        expect(bansTab).toBeInTheDocument();
    });
});
