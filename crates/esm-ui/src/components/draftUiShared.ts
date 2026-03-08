import type { ChampionClass, ChampionScaling } from '@/lib/api';

export const ROLE_SHORT: Record<string, string> = {
  Top: 'TOP',
  Jungle: 'JG',
  Mid: 'MID',
  Bot: 'ADC',
  Support: 'SUP',
};

export const ALL_CLASSES: ChampionClass[] = ['Tank', 'Fighter', 'Assassin', 'Mage', 'Marksman', 'Support'];
export const ALL_ROLES = ['Top', 'Jungle', 'Mid', 'Bot', 'Support'] as const;

export function roleShort(role: string): string {
  return ROLE_SHORT[role] ?? role.slice(0, 3).toUpperCase();
}

export function classColor(cls: ChampionClass): string {
  switch (cls) {
    case 'Tank':
      return 'var(--color-info)';
    case 'Fighter':
      return 'var(--color-warning)';
    case 'Assassin':
      return 'var(--color-loss)';
    case 'Mage':
      return 'var(--color-accent-emerald)';
    case 'Marksman':
      return 'var(--color-accent-cyan)';
    case 'Support':
      return 'var(--color-win)';
  }
}

export function scalingColor(scaling: ChampionScaling): string {
  switch (scaling) {
    case 'Early':
      return 'var(--color-win)';
    case 'Mid':
      return 'var(--color-warning)';
    case 'Late':
      return 'var(--color-loss)';
  }
}

export function masteryColor(mastery: string): string {
  switch (mastery) {
    case 'Challenger':
      return '#F59E0B';
    case 'Master':
      return '#A855F7';
    case 'Diamond':
      return '#06B6D4';
    case 'Platinum':
      return '#22D3EE';
    case 'Gold':
      return '#EAB308';
    case 'Silver':
      return '#94A3B8';
    case 'Bronze':
      return 'var(--text-muted)';
    default:
      return 'var(--text-muted)';
  }
}

export function masteryShort(mastery: string): string {
  switch (mastery) {
    case 'Challenger':
      return 'CHL';
    case 'Master':
      return 'MAS';
    case 'Diamond':
      return 'DIA';
    case 'Platinum':
      return 'PLT';
    case 'Gold':
      return 'GLD';
    case 'Silver':
      return 'SLV';
    case 'Bronze':
      return 'BRZ';
    default:
      return mastery.slice(0, 3).toUpperCase();
  }
}

export function metaTierColor(tier: string): string {
  switch (tier) {
    case 'S':
      return '#F59E0B';
    case 'A':
      return '#06B6D4';
    case 'B':
      return 'var(--text-secondary)';
    case 'C':
      return 'var(--text-muted)';
    case 'D':
      return 'rgba(255,255,255,0.25)';
    default:
      return 'var(--text-muted)';
  }
}
