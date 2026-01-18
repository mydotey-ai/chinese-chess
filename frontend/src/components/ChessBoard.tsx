import React, { useState } from 'react';
import './ChessBoard.css';

interface ChessBoardProps {
  board: any;
  onPieceClick: (x: number, y: number) => Promise<[number, number][]>;
  onMove: (fromX: number, fromY: number, toX: number, toY: number) => void;
}

const ChessBoard: React.FC<ChessBoardProps> = ({ 
  board, 
  onPieceClick, 
  onMove 
}) => {
  const [selectedPiece, setSelectedPiece] = useState<[number, number] | null>(null);
  const [validMoves, setValidMoves] = useState<[number, number][]>([]);

  const handleSquareClick = async (x: number, y: number) => {
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

  const isSelected = (x: number, y: number) => {
    return selectedPiece && selectedPiece[0] === x && selectedPiece[1] === y;
  };

  const isValidMove = (x: number, y: number) => {
    return validMoves.some(([mx, my]) => mx === x && my === y);
  };

  const getPieceCharacter = (piece: any) => {
    if (!piece) return '';
    
    const typeMap: Record<string, string> = {
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

  return (
    <div className="chess-board">
      {board.cells.map((row: any[], y: number) => (
        <div key={y} className="board-row">
          {row.map((cell: any, x: number) => (
            <div
              key={x}
              className={`board-cell ${
                isSelected(x, y) ? 'selected' : ''
              } ${
                isValidMove(x, y) ? 'valid-move' : ''
              }`}
              onClick={() => handleSquareClick(x, y)}
            >
              {cell && (
                <div className={`piece ${cell.color.toLowerCase()}`}>
                  {getPieceCharacter(cell)}
                </div>
              )}
            </div>
          ))}
        </div>
      ))}
    </div>
  );
};

export default ChessBoard;