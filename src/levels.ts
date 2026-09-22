import { LevelData, Pudding, Star, WorldData, PuddingType, WorldTheme } from './types';
import { v4 as uuidv4 } from 'uuid';

const createPudding = (x: number, y: number, type: PuddingType, id: string): Pudding => ({
  id, type, x, y, size: 40,
  isMoving: false, targetX: null, targetY: null, velocityX: 0, velocityY: 0,
});

const createStar = (x: number, y: number, id: string): Star => ({
  id, x, y, size: 30, collected: false,
});

const circlePositions = (centerX: number, centerY: number, radius: number, count: number): { x: number; y: number }[] => {
  return Array.from({ length: count }, (_, i) => {
    const angle = (2 * Math.PI * i) / count;
    return { x: centerX + radius * Math.cos(angle), y: centerY + radius * Math.sin(angle) };
  });
};

const gridPositions = (startX: number, startY: number, spacing: number, rows: number, cols: number): { x: number; y: number }[] => {
  const positions: { x: number; y: number }[] = [];
  for (let row = 0; row < rows; row++) {
    for (let col = 0; col < cols; col++) {
      positions.push({ x: startX + col * spacing, y: startY + row * spacing });
    }
  }
  return positions;
};

const generateLevels = (): WorldData[] => {
  const worlds: WorldData[] = [];
  const worldThemes: WorldTheme[] = ['forest', 'desert', 'ice', 'volcano', 'space'];
  const worldNames = ['Forest World', 'Desert World', 'Ice World', 'Volcano World', 'Space World'];
  const typeCycle: PuddingType[] = ['normal', 'green', 'purple'];

  for (let worldIdx = 0; worldIdx < worldThemes.length; worldIdx++) {
    const levels: LevelData[] = [];

    for (let levelIdx = 0; levelIdx < 25; levelIdx++) {
      const puddingCount = 3 + Math.min(levelIdx * 2, 20);
      const maxMoves = 10 + levelIdx * 3;
      const starsCount = Math.min(Math.floor((levelIdx + 2) / 4), 3);
      const requiredStars = Math.max(1, Math.floor(starsCount / 2));

      let positions: { x: number; y: number }[];

      switch (levelIdx % 4) {
        case 0:
          positions = gridPositions(100, 100, 50, Math.ceil(puddingCount / 3), 3);
          break;
        case 1:
          positions = circlePositions(250, 250, 80 + levelIdx * 5, puddingCount);
          break;
        case 2:
          positions = Array.from({ length: puddingCount }, (_, i) => {
            const angle = (2 * Math.PI * i) / puddingCount;
            const radius = 60 + i * 15;
            return { x: 250 + radius * Math.cos(angle), y: 250 + radius * Math.sin(angle) };
          });
          break;
        default:
          positions = gridPositions(80, 80, 45, Math.ceil(puddingCount / 2), 2);
      }

      const puddings = positions.slice(0, puddingCount).map((pos, i) =>
        createPudding(pos.x, pos.y, typeCycle[(i + levelIdx + worldIdx) % 3], uuidv4())
      );

      const stars = Array.from({ length: starsCount }, (_, s) =>
        createStar(150 + s * 150, 100 + s * 50, uuidv4())
      );

      levels.push({
        name: worldNames[worldIdx] + ' - Level ' + (levelIdx + 1),
        puddings,
        maxMoves,
        stars,
        requiredStars,
      });
    }

    worlds.push({ name: worldNames[worldIdx], theme: worldThemes[worldIdx], levels });
  }

  return worlds;
};

let worldsCache: WorldData[] | null = null;

export const getWorlds = (): WorldData[] => {
  if (!worldsCache) {
    worldsCache = generateLevels();
  }
  return worldsCache;
};

export const getLevel = (worldIndex: number, levelIndex: number): LevelData | null => {
  const worlds = getWorlds();
  if (worldIndex < worlds.length && levelIndex < worlds[worldIndex].levels.length) {
    return worlds[worldIndex].levels[levelIndex];
  }
  return null;
};

export const getTotalLevels = (): number => {
  return getWorlds().reduce((total, world) => total + world.levels.length, 0);
};

export const WORLDS: WorldData[] = generateLevels();