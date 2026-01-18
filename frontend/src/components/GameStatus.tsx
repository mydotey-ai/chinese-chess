import React from 'react';
import './GameStatus.css';

interface GameStatusProps {
  currentTurn: 'Red' | 'Black';
  isInCheck: boolean;
  isEnded: boolean;
  winner: 'Red' | 'Black' | null;
}

const GameStatus: React.FC<GameStatusProps> = ({
  currentTurn,
  isInCheck,
  isEnded,
  winner
}) => {
  const getTurnColor = () => {
    return currentTurn === 'Red' ? 'red' : 'black';
  };

  return (
    <div className="game-status">
      {isEnded ? (
        <div className="game-ended">
          <h2>游戏结束</h2>
          <div className="winner">
            获胜方: <span className={winner?.toLowerCase()}>{winner}</span>
          </div>
        </div>
      ) : (
        <div className="game-playing">
          <div className="turn-info">
            当前回合: <span className={getTurnColor()}>{currentTurn}</span>
          </div>
          {isInCheck && (
            <div className="check-warning">
              <span className="check-icon">⚠️</span>
              将军!
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default GameStatus;