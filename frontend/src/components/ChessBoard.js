import { jsx as _jsx } from "react/jsx-runtime";
import React, { useState } from 'react';
import './ChessBoard.css';
const ChessBoard = ({ board, onPieceClick, onMove, currentTurn }) => {
    const [selectedPiece, setSelectedPiece] = useState(null);
    const [validMoves, setValidMoves] = useState([]);
    const handleSquareClick = async (x, y) => {
        // If a piece is selected, try to move it
        if (selectedPiece) {
            const [fromX, fromY] = selectedPiece;
            onMove(fromX, fromY, x, y);
            setSelectedPiece(null);
            setValidMoves([]);
            return;
        }
        // Otherwise, try to select a piece
        const piece = board.cells[y][x];
        if (piece) {
            const moves = await onPieceClick(x, y);
            setSelectedPiece([x, y]);
            setValidMoves(moves);
        }
    };
    const isSelected = (x, y) => {
        return selectedPiece && selectedPiece[0] === x && selectedPiece[1] === y;
    };
    const isValidMove = (x, y) => {
        return validMoves.some(([mx, my]) => mx === x && my === y);
    };
    const getPieceCharacter = (piece) => {
        if (!piece)
            return '';
        const typeMap = {
            'General': '将',
            'Advisor': '士',
            'Elephant': '象',
            'Horse': '马',
            'Chariot': '车',
            'Cannon': '炮',
            'Soldier': '卒'
        };
        return typeMap[piece.piece_type] || '';
    };
    return (_jsx("div", { className: "chess-board", children: board.cells.map((row, y) => (_jsx("div", { className: "board-row", children: row.map((cell, x) => (_jsx("div", { className: `board-cell ${isSelected(x, y) ? 'selected' : ''} ${isValidMove(x, y) ? 'valid-move' : ''}`, onClick: () => handleSquareClick(x, y), children: cell && (_jsx("div", { className: `piece ${cell.color.toLowerCase()}`, children: getPieceCharacter(cell) })) }, x))) }, y))) }));
};
export default ChessBoard;
//# sourceMappingURL=ChessBoard.js.map