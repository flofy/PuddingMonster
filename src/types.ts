export type PuddingType = 'normal' | 'green' | 'purple';

export interface Pudding {
  id: string;
  type: PuddingType;
  x: number;
  y: number;
  size: number;
  isMoving: boolean;
  targetX: number | null;
  targetY: number | null;
  velocityX: number;
  velocityY: number;
}

export interface Star {
  id: string;
  x: number;
  y: number;
  size: number;
  collected: boolean;
}

export interface LevelData {
  name: string;
  puddings: Pudding[];
  maxMoves: number;
  stars: Star[];
  requiredStars: number;
}

export interface WorldData {
  name: string;
  theme: WorldTheme;
  levels: LevelData[];
}

export type WorldTheme = 'forest' | 'desert' | 'ice' | 'volcano' | 'space';

export interface GameState {
  currentWorld: number;
  currentLevel: number;
  movesRemaining: number;
  starsCollected: number;
  isDragging: boolean;
  dragStart: { x: number; y: number } | null;
  selectedPudding: string | null;
  unlockedWorlds: boolean[];
}

export const BOARD_WIDTH = 800;
export const BOARD_HEIGHT = 600;