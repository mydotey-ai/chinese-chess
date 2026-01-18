import React from 'react';
import './GameStatus.css';
interface GameStatusProps {
    currentTurn: 'Red' | 'Black';
    isInCheck: boolean;
    isEnded: boolean;
    winner: 'Red' | 'Black' | null;
}
declare const GameStatus: React.FC<GameStatusProps>;
export default GameStatus;
//# sourceMappingURL=GameStatus.d.ts.map