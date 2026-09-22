import React, { useState, useEffect, useCallback, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { WORLDS, getLevel } from './levels';
import { Pudding, Star, GameState, BOARD_WIDTH, BOARD_HEIGHT, WorldTheme } from './types';
import { v4 as uuidv4 } from 'uuid';

const App: React.FC = () => {
  const [gameState, setGameState] = useState<GameState>(() => ({
    currentWorld: 0,
    currentLevel: 0,
    movesRemaining: 0,
    starsCollected: 0,
    isDragging: false,
    dragStart: null,
    selectedPudding: null,
    unlockedWorlds: [true, false, false, false, false],
  }));

  const [puddings, setPuddings] = useState<Pudding[]>([]);
  const [stars, setStars] = useState<Star[]>([]);
  const [showWinModal, setShowWinModal] = useState(false);
  const [showWorldSelect, setShowWorldSelect] = useState(false);
  const [boardTheme, setBoardTheme] = useState<WorldTheme>('forest');

  const boardRef = useRef<HTMLDivElement>(null);

  const initializeLevel = useCallback((worldIndex: number, levelIndex: number) => {
    const level = getLevel(worldIndex, levelIndex);
    if (!level) return;

    const newPuddings = level.puddings.map(p => ({
      ...p,
      id: uuidv4(),
      isMoving: false,
      targetX: null,
      targetY: null,
      velocityX: 0,
      velocityY: 0,
    }));

    const newStars = level.stars.map(s => ({
      ...s,
      id: uuidv4(),
      collected: false,
    }));

    setPuddings(newPuddings);
    setStars(newStars);
    setGameState(prev => ({
      ...prev,
      currentWorld: worldIndex,
      currentLevel: levelIndex,
      movesRemaining: level.maxMoves,
      starsCollected: 0,
      isDragging: false,
      dragStart: null,
      selectedPudding: null,
    }));

    setBoardTheme(WORLDS[worldIndex].theme);
  }, []);

  useEffect(() => {
    initializeLevel(0, 0);
  }, [initializeLevel]);

  useEffect(() => {
    if (showWinModal) {
      const timer = setTimeout(() => {
        const nextLevelIndex = gameState.currentLevel + 1;
        const currentWorld = WORLDS[gameState.currentWorld];

        if (nextLevelIndex < currentWorld.levels.length) {
          initializeLevel(gameState.currentWorld, nextLevelIndex);
        } else {
          const nextWorldIndex = gameState.currentWorld + 1;
          if (nextWorldIndex < WORLDS.length) {
            setGameState(prev => ({
              ...prev,
              unlockedWorlds: prev.unlockedWorlds.map((unlocked, index) =>
                index === nextWorldIndex ? true : unlocked
              ),
            }));
            initializeLevel(nextWorldIndex, 0);
          } else {
            initializeLevel(0, 0);
          }
        }
      }, 1500);

      return () => clearTimeout(timer);
    }
  }, [showWinModal, gameState, initializeLevel]);

  useEffect(() => {
    if (puddings.length === 1) {
      const collected = stars.filter(s => s.collected).length;
      const level = getLevel(gameState.currentWorld, gameState.currentLevel);
      if (level && collected >= level.requiredStars) {
        setShowWinModal(true);
      }
    }
  }, [puddings, stars, gameState.currentWorld, gameState.currentLevel]);

  const handleMouseDown = useCallback((e: React.MouseEvent<HTMLDivElement>) => {
    if (gameState.isDragging || gameState.movesRemaining === 0) return;

    const rect = boardRef.current?.getBoundingClientRect();
    if (!rect) return;

    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    for (const pudding of puddings) {
      const halfSize = pudding.size / 2;
      if (x >= pudding.x - halfSize && x <= pudding.x + halfSize &&
          y >= pudding.y - halfSize && y <= pudding.y + halfSize) {
        setGameState(prev => ({
          ...prev,
          isDragging: true,
          dragStart: { x, y },
          selectedPudding: pudding.id,
        }));
        break;
      }
    }
  }, [gameState, puddings]);

  const handleMouseUp = useCallback((e: React.MouseEvent<HTMLDivElement>) => {
    if (!gameState.isDragging || !gameState.dragStart || !gameState.selectedPudding) return;

    const rect = boardRef.current?.getBoundingClientRect();
    if (!rect) return;

    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    const dx = x - gameState.dragStart.x;
    const dy = y - gameState.dragStart.y;
    const distance = Math.sqrt(dx * dx + dy * dy);

    if (distance > 5) {
      const directionX = dx / distance;
      const directionY = dy / distance;

      setPuddings(prev => {
        let updated = prev.map(p => {
          if (p.id === gameState.selectedPudding) {
            return {
              ...p,
              isMoving: true,
              targetX: p.x + directionX * 200,
              targetY: p.y + directionY * 200,
              velocityX: directionX * 200,
              velocityY: directionY * 200,
            };
          }
          return p;
        });

        const movedPudding = prev.find(p => p.id === gameState.selectedPudding);
        if (movedPudding && movedPudding.type === 'purple') {
          updated = updated.map(p => {
            if (p.type === 'purple' && p.id !== gameState.selectedPudding) {
              return {
                ...p,
                isMoving: true,
                targetX: p.x + directionX * 200,
                targetY: p.y + directionY * 200,
                velocityX: directionX * 200,
                velocityY: directionY * 200,
              };
            }
            return p;
          });
        }

        return updated;
      });

      setGameState(prev => ({
        ...prev,
        movesRemaining: Math.max(0, prev.movesRemaining - 1),
      }));
    }

    setGameState(prev => ({
      ...prev,
      isDragging: false,
      dragStart: null,
      selectedPudding: null,
    }));
  }, [gameState, setPuddings]);

  useEffect(() => {
    if (puddings.some(p => p.isMoving)) {
      const animationId = requestAnimationFrame(() => {
        setPuddings(prev => {
          let hasMoving = false;
          const updated = prev.map(p => {
            if (p.isMoving && p.targetX !== null && p.targetY !== null) {
              hasMoving = true;
              const dx = p.targetX - p.x;
              const dy = p.targetY - p.y;
              const distance = Math.sqrt(dx * dx + dy * dy);

              if (distance < 10) {
                return {
                  ...p,
                  x: p.targetX,
                  y: p.targetY,
                  isMoving: false,
                  velocityX: 0,
                  velocityY: 0,
                  targetX: null,
                  targetY: null,
                };
              } else {
                const speed = 5;
                const moveX = (dx / distance) * speed;
                const moveY = (dy / distance) * speed;
                return { ...p, x: p.x + moveX, y: p.y + moveY };
              }
            }
            return p;
          });

          if (!hasMoving) {
            checkCollisions(updated);
          }

          return updated;
        });
      });

      return () => cancelAnimationFrame(animationId);
    }
  }, [puddings]);

  const checkCollisions = (currentPuddings: Pudding[]) => {
    const merged = new Set<string>();

    for (let i = 0; i < currentPuddings.length; i++) {
      if (merged.has(currentPuddings[i].id)) continue;

      for (let j = i + 1; j < currentPuddings.length; j++) {
        if (merged.has(currentPuddings[j].id)) continue;

        const p1 = currentPuddings[i];
        const p2 = currentPuddings[j];
        const dx = p1.x - p2.x;
        const dy = p1.y - p2.y;
        const distance = Math.sqrt(dx * dx + dy * dy);

        if (distance < 40) {
          merged.add(p2.id);
          break;
        }
      }
    }

    if (merged.size > 0) {
      setPuddings(prev => prev.filter(p => !merged.has(p.id)));
    }
  };

  useEffect(() => {
    if (!gameState.isDragging && puddings.length > 0) {
      const collectedStars = stars.map(star => {
        if (star.collected) return star;

        for (const pudding of puddings) {
          const dx = pudding.x - star.x;
          const dy = pudding.y - star.y;
          const distance = Math.sqrt(dx * dx + dy * dy);

          if (distance < 35) {
            return { ...star, collected: true };
          }
        }
        return star;
      });

      const collectedCount = collectedStars.filter(s => s.collected).length;
      if (collectedCount !== stars.filter(s => s.collected).length) {
        setStars(collectedStars);
        setGameState(prev => ({
          ...prev,
          starsCollected: collectedCount,
        }));
      }
    }
  }, [puddings, stars, gameState.isDragging]);

  const getThemeColors = () => {
    switch (boardTheme) {
      case 'forest':
        return { board: '#1a3d2a', puddingNormal: '#d62828', puddingGreen: '#059669', puddingPurple: '#7c3aed', star: '#fbbf24' };
      case 'desert':
        return { board: '#374151', puddingNormal: '#dc2626', puddingGreen: '#16a34a', puddingPurple: '#9333ea', star: '#fbbf24' };
      case 'ice':
        return { board: '#0f172a', puddingNormal: '#ef4444', puddingGreen: '#22c55e', puddingPurple: '#a855f7', star: '#fbbf24' };
      case 'volcano':
        return { board: '#1f2937', puddingNormal: '#fca5a5', puddingGreen: '#4ade80', puddingPurple: '#c084fc', star: '#fbbf24' };
      case 'space':
        return { board: '#0f0f23', puddingNormal: '#f87171', puddingGreen: '#34d399', puddingPurple: '#a78bfa', star: '#fbbf24' };
      default:
        return { board: '#1a1a2e', puddingNormal: '#ef4444', puddingGreen: '#22c55e', puddingPurple: '#a855f7', star: '#fbbf24' };
    }
  };

  const colors = getThemeColors();

  return (
    <div style={{
      height: '100vh',
      width: '100vw',
      background: 'linear-gradient(135deg, #1a1a2e 0%, #16213e 100%)',
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
      padding: '20px',
      overflow: 'hidden',
    }}>
      <h1 style={{ color: 'white', marginBottom: '10px' }}>Pudding Monsters</h1>

      <div style={{
        display: 'flex',
        justifyContent: 'space-between',
        width: '100%',
        maxWidth: BOARD_WIDTH + 'px',
        marginBottom: '10px',
      }}>
        <div style={{ color: 'white' }}>
          World: {WORLDS[gameState.currentWorld].name} | Level: {gameState.currentLevel + 1}/{WORLDS[gameState.currentWorld].levels.length}
        </div>
        <div style={{ color: 'white' }}>
          Moves: {gameState.movesRemaining} | Stars: {gameState.starsCollected}/{stars.length}
        </div>
      </div>

      <div
        ref={boardRef}
        style={{
          position: 'relative',
          width: BOARD_WIDTH,
          height: BOARD_HEIGHT,
          background: colors.board,
          borderRadius: '8px',
          overflow: 'hidden',
          cursor: 'grab',
          boxShadow: '0 0 20px rgba(0, 0, 0, 0.5)',
        }}
        onMouseDown={handleMouseDown}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
      >
        <AnimatePresence>
          {stars.map(star => (
            !star.collected && (
              <motion.div
                key={star.id}
                style={{
                  position: 'absolute',
                  left: star.x - star.size / 2,
                  top: star.y - star.size / 2,
                  width: star.size,
                  height: star.size,
                  background: colors.star,
                  borderRadius: '50%',
                  boxShadow: '0 0 10px rgba(255, 255, 0, 0.7)',
                }}
                initial={{ scale: 0 }}
                animate={{ scale: 1 }}
                exit={{ scale: 0, opacity: 0 }}
                transition={{ duration: 0.3 }}
              />
            )
          ))}
        </AnimatePresence>

        {puddings.map(pudding => (
          <motion.div
            key={pudding.id}
            style={{
              position: 'absolute',
              left: pudding.x - pudding.size / 2,
              top: pudding.y - pudding.size / 2,
              width: pudding.size,
              height: pudding.size,
              borderRadius: '50%',
              background: pudding.type === 'normal' ? colors.puddingNormal :
                          pudding.type === 'green' ? colors.puddingGreen : colors.puddingPurple,
              boxShadow: '0 0 10px rgba(0, 0, 0, 0.5)',
            }}
            initial={{ scale: 0 }}
            animate={{ scale: 1 }}
            transition={{ type: 'spring', damping: 10, stiffness: 100 }}
          />
        ))}

        <AnimatePresence>
          {showWinModal && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              style={{
                position: 'absolute',
                top: 0, left: 0, right: 0, bottom: 0,
                background: 'rgba(0, 0, 0, 0.7)',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                zIndex: 100,
              }}
            >
              <motion.div
                initial={{ scale: 0.5, opacity: 0 }}
                animate={{ scale: 1, opacity: 1 }}
                exit={{ scale: 0.5, opacity: 0 }}
                transition={{ duration: 0.3 }}
                style={{
                  background: 'linear-gradient(135deg, #1a3d2a 0%, #0f2617 100%)',
                  padding: '40px',
                  borderRadius: '16px',
                  textAlign: 'center',
                  boxShadow: '0 0 20px rgba(0, 0, 0, 0.5)',
                }}
              >
                <h2 style={{ color: '#fbbf24', marginBottom: '20px' }}>Level Complete!</h2>
                <p style={{ color: 'white', marginBottom: '20px' }}>
                  Stars collected: {gameState.starsCollected}/{stars.length}
                </p>
                <motion.button
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  onClick={() => setShowWinModal(false)}
                  style={{
                    background: '#22c55e',
                    color: 'white',
                    border: 'none',
                    padding: '12px 24px',
                    borderRadius: '8px',
                    fontSize: '16px',
                    cursor: 'pointer',
                  }}
                >
                  Continue
                </motion.button>
              </motion.div>
            </motion.div>
          )}
        </AnimatePresence>

        <button
          onClick={() => setShowWorldSelect(!showWorldSelect)}
          style={{
            position: 'absolute',
            top: '10px',
            right: '10px',
            background: 'rgba(255, 255, 255, 0.2)',
            color: 'white',
            border: 'none',
            padding: '8px 12px',
            borderRadius: '4px',
            cursor: 'pointer',
            zIndex: 1000,
          }}
        >
          Worlds
        </button>

        <AnimatePresence>
          {showWorldSelect && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              style={{
                position: 'absolute',
                top: 0, left: 0, right: 0, bottom: 0,
                background: 'rgba(0, 0, 0, 0.8)',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                zIndex: 1000,
              }}
              onClick={() => setShowWorldSelect(false)}
            >
              <motion.div
                initial={{ scale: 0.5, opacity: 0 }}
                animate={{ scale: 1, opacity: 1 }}
                exit={{ scale: 0.5, opacity: 0 }}
                transition={{ duration: 0.3 }}
                onClick={e => e.stopPropagation()}
                style={{
                  background: 'linear-gradient(135deg, #1a1a2e 0%, #16213e 100%)',
                  padding: '40px',
                  borderRadius: '16px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '20px',
                  maxWidth: '500px',
                }}
              >
                <h2 style={{ color: 'white', textAlign: 'center' }}>Select World</h2>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                  {WORLDS.map((world, index) => (
                    <motion.button
                      key={index}
                      whileHover={{ scale: 1.02 }}
                      whileTap={{ scale: 0.98 }}
                      onClick={() => {
                        if (gameState.unlockedWorlds[index]) {
                          setShowWorldSelect(false);
                          initializeLevel(index, 0);
                        }
                      }}
                      disabled={!gameState.unlockedWorlds[index]}
                      style={{
                        background: gameState.unlockedWorlds[index]
                          ? getThemeColorForWorld(world.theme)
                          : 'rgba(255, 255, 255, 0.1)',
                        color: gameState.unlockedWorlds[index] ? 'white' : 'rgba(255, 255, 255, 0.5)',
                        border: 'none',
                        padding: '16px',
                        borderRadius: '8px',
                        fontSize: '18px',
                        cursor: gameState.unlockedWorlds[index] ? 'pointer' : 'not-allowed',
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'center',
                      }}
                    >
                      <span>{world.name}</span>
                      <span>{gameState.unlockedWorlds[index] ? 'OK' : 'LOCKED'}</span>
                    </motion.button>
                  ))}
                </div>
                <button
                  onClick={() => setShowWorldSelect(false)}
                  style={{
                    background: 'rgba(255, 255, 255, 0.2)',
                    color: 'white',
                    border: 'none',
                    padding: '12px',
                    borderRadius: '8px',
                    cursor: 'pointer',
                  }}
                >
                  Close
                </button>
              </motion.div>
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </div>
  );
};

const getThemeColorForWorld = (theme: WorldTheme) => {
  switch (theme) {
    case 'forest': return 'linear-gradient(135deg, #1a3d2a 0%, #0f2617 100%)';
    case 'desert': return 'linear-gradient(135deg, #f59e0b 0%, #d97706 100%)';
    case 'ice': return 'linear-gradient(135deg, #06b6d4 0%, #0891b2 100%)';
    case 'volcano': return 'linear-gradient(135deg, #ea580c 0%, #dc2626 100%)';
    case 'space': return 'linear-gradient(135deg, #1e1b4b 0%, #7c3aed 100%)';
    default: return 'linear-gradient(135deg, #1a1a2e 0%, #16213e 100%)';
  }
};

export default App;