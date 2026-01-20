import React, { useState } from 'react';
import './ChessBoard.css';

interface ChessBoardProps {
  board: any;
  onPieceClick: (x: number, y: number) => Promise<[number, number][]>;
  onMove: (fromX: number, fromY: number, toX: number, toY: number) => void;
}

interface ChessBoardProps {
  board: any;
  onPieceClick: (x: number, y: number) => Promise<[number, number][]>;
  onMove: (fromX: number, fromY: number, toX: number, toY: number) => void;
  isEnded: boolean; // 新增属性
}

const ChessBoard: React.FC<ChessBoardProps> = ({ 
  board, 
  onPieceClick, 
  onMove,
  isEnded
}) => {
  const [selectedPiece, setSelectedPiece] = useState<[number, number] | null>(null);
  const [validMoves, setValidMoves] = useState<[number, number][]>([]);

  const handleSquareClick = async (x: number, y: number) => {
    // 如果游戏已经结束，不允许任何操作
    if (isEnded) {
      return;
    }

    if (selectedPiece) {
      const [fromX, fromY] = selectedPiece;
      // 只有在目标位置是有效的移动位置时才执行移动
      if (isValidMove(x, y)) {
        onMove(fromX, fromY, x, y);
        setSelectedPiece(null);
        setValidMoves([]);
      } else {
        // 如果点击了无效位置，取消选择
        setSelectedPiece(null);
        setValidMoves([]);
      }
      return;
    }

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
    
    const typeMap: Record<string, Record<string, string>> = {
      'Red': {
        'General': '帅',
        'Advisor': '仕',
        'Elephant': '相',
        'Horse': '馬',
        'Chariot': '车',
        'Cannon': '炮',
        'Soldier': '兵'
      },
      'Black': {
        'General': '将',
        'Advisor': '士',
        'Elephant': '象',
        'Horse': '马',
        'Chariot': '车',
        'Cannon': '炮',
        'Soldier': '卒'
      }
    };

    return typeMap[piece.color]?.[piece.piece_type] || '';
  };

  return (
    <div className="chess-board">
      <svg className="board-lines" width="400" height="450" viewBox="0 0 400 450">
        {/* Horizontal lines - 10 lines (y=0 to y=9, step 50) */}
        {[...Array(10)].map((_, y) => (
          <line
            key={`h${y}`}
            x1="0"
            y1={y * 50}
            x2="400"
            y2={y * 50}
            stroke="#000"
            strokeWidth="1"
          />
        ))}

        {/* Left vertical border (full height) */}
        <line x1="0" y1="0" x2="0" y2="450" stroke="#000" strokeWidth="1" />

        {/* Right vertical border (full height) */}
        <line x1="400" y1="0" x2="400" y2="450" stroke="#000" strokeWidth="1" />

        {/* Inner vertical lines - upper half (x=50 to x=350, y=0 to y=200) */}
        {[...Array(7)].map((_, i) => (
          <line
            key={`v-upper${i}`}
            x1={(i + 1) * 50}
            y1="0"
            x2={(i + 1) * 50}
            y2="200"
            stroke="#000"
            strokeWidth="1"
          />
        ))}

        {/* Inner vertical lines - lower half (x=50 to x=350, y=250 to y=450) */}
        {[...Array(7)].map((_, i) => (
          <line
            key={`v-lower${i}`}
            x1={(i + 1) * 50}
            y1="250"
            x2={(i + 1) * 50}
            y2="450"
            stroke="#000"
            strokeWidth="1"
          />
        ))}

        {/* Top palace (black) diagonals: (150,0)-(250,100) and (250,0)-(150,100) */}
        <line x1="150" y1="0" x2="250" y2="100" stroke="#000" strokeWidth="1" />
        <line x1="250" y1="0" x2="150" y2="100" stroke="#000" strokeWidth="1" />

        {/* Bottom palace (red) diagonals: (150,350)-(250,450) and (250,350)-(150,450) */}
        <line x1="150" y1="350" x2="250" y2="450" stroke="#000" strokeWidth="1" />
        <line x1="250" y1="350" x2="150" y2="450" stroke="#000" strokeWidth="1" />

        {/* River text - 楚河 (left side) */}
        <text x="100" y="238" fontSize="20" fill="#8b4513" textAnchor="middle" fontWeight="bold">楚河</text>
        
        {/* River text - 汉界 (right side) */}
        <text x="300" y="238" fontSize="20" fill="#8b4513" textAnchor="middle" fontWeight="bold">汉界</text>
      </svg>

      <div className="pieces-container">
        {board.cells.map((row: any[], y: number) => (
          <React.Fragment key={y}>
            {row.map((cell: any, x: number) => (
              <div
                key={`${x}-${y}`}
                className={`piece-container ${
                  isSelected(x, y) ? 'selected' : ''
                } ${
                  isValidMove(x, y) ? 'valid-move' : ''
                }`}
                style={{
                  left: `${x * 50 - 20}px`,
                  top: `${y * 50 - 20}px`
                }}
                onClick={() => handleSquareClick(x, y)}
              >
                {cell && (
                  <div className={`piece ${cell.color.toLowerCase()}`}>
                    {getPieceCharacter(cell)}
                  </div>
                )}
              </div>
            ))}
          </React.Fragment>
        ))}
      </div>
    </div>
  );
};

export default ChessBoard;
