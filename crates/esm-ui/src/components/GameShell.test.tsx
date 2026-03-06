import { describe, it, expect, vi } from 'vitest';
import { renderWithProviders } from '../test/render';
import { screen, fireEvent } from '@testing-library/react';
import { GameShell } from './GameShell';

const mockFetch = vi.fn();

// Mock the API hooks
vi.mock('../lib/use-api', async () => {
    const actual = await vi.importActual('../lib/use-api');
    return {
        ...actual,
        useRoster: vi.fn(() => ({ roster: [], fetchRoster: mockFetch })),
        useInbox: vi.fn(() => ({ messages: [], fetchInbox: mockFetch })),
        useStandings: vi.fn(() => ({ standings: [], fetchStandings: mockFetch })),
        useSchedule: vi.fn(() => ({ schedule: [], fetchSchedule: mockFetch })),
        useResolveMessage: vi.fn(() => ({ resolve: mockFetch })),
        usePlayMatchDelegate: vi.fn(() => ({ playMatchDelegate: mockFetch, playing: false })),
    };
});

describe('GameShell', () => {
    const defaultProps = {
        teamName: 'T1',
        year: 2025,
        month: 1,
        day: 1,
        phase: 'Morning',
    };

    it('renders GameShell mapping and initial dashboard', () => {
        renderWithProviders(<GameShell {...defaultProps} />);
        expect(screen.getAllByText('T1').length).toBeGreaterThan(0);
        expect(screen.getAllByText('Dashboard').length).toBeGreaterThan(0);
    });

    it('navigates to different pages via sidebar', () => {
        renderWithProviders(<GameShell {...defaultProps} />);

        // Roster
        fireEvent.click(screen.getByRole('button', { name: /roster/i }));
        expect(screen.getAllByText('Roster').length).toBeGreaterThan(0);

        // Schedule
        fireEvent.click(screen.getByRole('button', { name: /schedule/i }));
        expect(screen.getAllByText('Schedule').length).toBeGreaterThan(0);

        // Inbox
        fireEvent.click(screen.getByRole('button', { name: /inbox/i }));
        expect(screen.getAllByText('Inbox').length).toBeGreaterThan(0);

        // Standings
        fireEvent.click(screen.getByRole('button', { name: /standings/i }));
        expect(screen.getAllByText('Standings').length).toBeGreaterThan(0);
    });
});
