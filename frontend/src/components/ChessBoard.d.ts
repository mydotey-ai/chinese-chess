import React from 'react';
import './ChessBoard.css';
interface ChessBoardProps {
    board: any;
    onPieceClick: (x: number, y: number) => Promise<[number, number][]>;
    onMove: (fromX: number, fromY: number, toX: number, toY: number) => void;
    currentTurn: 'Red' | 'Black';
}
declare const ChessBoard: React.FC<ChessBoardProps>;
export default ChessBoard;
//# sourceMappingURL=ChessBoard.d.ts.map