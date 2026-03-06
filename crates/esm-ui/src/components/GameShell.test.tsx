import { describe, it, expect, vi } from 'vitest';
import { renderWithProviders } from '../test/render';
import { screen, fireEvent } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { GameShell } from './GameShell';

const mockFetch = vi.fn();

// Mock the API hooks
const MOCK_WEEK_SCHEDULE = {
    days: Array.from({ length: 7 }, (_, i) => ({
        day_index: i,
        slots: ['Morning', 'Afternoon', 'Evening'].map((ts) => ({
            time_slot: ts,
            entry_type: 'free',
            scrim_id: null,
            opponent: null,
            players: null,
            focus: null,
        })),
    })),
    total_scrims: 0,
    occupied_slots: 0,
};

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
        useTeamSchedule: vi.fn(() => ({
            weekSchedule: MOCK_WEEK_SCHEDULE,
            scrims: [],
            refresh: mockFetch,
            scheduleScrim: mockFetch,
            cancelScrim: mockFetch,
            scheduleSoloQueue: mockFetch,
            scheduleRest: mockFetch,
            clearSlot: mockFetch,
        })),
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

    it('renders weekly schedule content when navigating to schedule page', () => {
        renderWithProviders(<GameShell {...defaultProps} />);
        fireEvent.click(screen.getByRole('button', { name: /schedule/i }));
        expect(screen.getByRole('heading', { name: /weekly schedule/i })).toBeInTheDocument();
        expect(screen.queryByText(/loading schedule/i)).not.toBeInTheDocument();
    });

    it('calls onPlayMatch with mode when PlayMatchButton is confirmed on match day', async () => {
        const user = userEvent.setup();
        const onPlayMatch = vi.fn();
        renderWithProviders(
            <GameShell {...defaultProps} isMatchDay={true} onPlayMatch={onPlayMatch} />
        );

        // TopBar should render PlayMatchButton with Participate as default
        await user.click(screen.getByRole('button', { name: /participate/i }));
        await user.click(screen.getByRole('button', { name: /^confirm$/i }));

        expect(onPlayMatch).toHaveBeenCalledWith('participate');
    });
});
