import React, { useState, useEffect } from 'react';
import ChessBoard from './components/ChessBoard.tsx';
import HistoryPanel from './components/HistoryPanel.tsx';
import GameStatus from './components/GameStatus.tsx';
import ControlPanel from './components/ControlPanel.tsx';
import { invoke } from '@tauri-apps/api/core';

interface GameState {
  board: any;
  current_turn: 'Red' | 'Black';
  is_in_check: boolean;
  is_ended: boolean;
  winner: 'Red' | 'Black' | null;
}

const App: React.FC = () => {
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [moveHistory, setMoveHistory] = useState<string[]>([]);

  useEffect(() => {
    // Initialize game
    initGame();
  }, []);

  const initGame = async () => {
    try {
      const state = await invoke<GameState>('new_game');
      setGameState(state);
      setMoveHistory([]);
    } catch (error) {
      console.error('Error initializing game:', error);
      // Create fallback game state
      const cells = Array(10).fill(null).map((_: any, y: number) => 
        Array(9).fill(null).map((_: any, x: number) => {
          // Black pieces (top, y=0)
          if (y === 0) {
            if ([0, 8].includes(x)) return { piece_type: 'Chariot', color: 'Black' };
            if ([1, 7].includes(x)) return { piece_type: 'Horse', color: 'Black' };
            if ([2, 6].includes(x)) return { piece_type: 'Elephant', color: 'Black' };
            if ([3, 5].includes(x)) return { piece_type: 'Advisor', color: 'Black' };
            if (x === 4) return { piece_type: 'General', color: 'Black' };
          }
          if (y === 2 && [1, 7].includes(x)) return { piece_type: 'Cannon', color: 'Black' };
          if (y === 3 && x % 2 === 0) return { piece_type: 'Soldier', color: 'Black' };
          
          // Red pieces (bottom, y=9)
          if (y === 9) {
            if ([0, 8].includes(x)) return { piece_type: 'Chariot', color: 'Red' };
            if ([1, 7].includes(x)) return { piece_type: 'Horse', color: 'Red' };
            if ([2, 6].includes(x)) return { piece_type: 'Elephant', color: 'Red' };
            if ([3, 5].includes(x)) return { piece_type: 'Advisor', color: 'Red' };
            if (x === 4) return { piece_type: 'General', color: 'Red' };
          }
          if (y === 7 && [1, 7].includes(x)) return { piece_type: 'Cannon', color: 'Red' };
          if (y === 6 && x % 2 === 0) return { piece_type: 'Soldier', color: 'Red' };
          
          return null;
        })
      );
      
      setGameState({
        board: { cells },
        current_turn: 'Red',
        is_in_check: false,
        is_ended: false,
        winner: null
      });
      setMoveHistory([]);
    }
  };

  const handleMakeMove = async (fromX: number, fromY: number, toX: number, toY: number) => {
    try {
      const state = await invoke<GameState>('make_move', { 
        from_x: fromX, from_y: fromY, to_x: toX, to_y: toY 
      });
      setGameState(state);
      addMoveToHistory(fromX, fromY, toX, toY);
    } catch (error) {
      console.error('Error making move, using fallback:', error);
      if (gameState) {
        const newBoard = JSON.parse(JSON.stringify(gameState.board));
        const piece = newBoard.cells[fromY][fromX];
        const capturedPiece = newBoard.cells[toY][toX];
        
        newBoard.cells[fromY][fromX] = null;
        newBoard.cells[toY][toX] = piece;
        
        // 检查是否吃掉了对方的将/帅
        const isGameEnded = capturedPiece && capturedPiece.piece_type === 'General';
        
        setGameState({
          ...gameState,
          board: newBoard,
          current_turn: gameState.current_turn === 'Red' ? 'Black' : 'Red',
          is_ended: isGameEnded,
          winner: isGameEnded ? gameState.current_turn : null
        });
        
        addMoveToHistory(fromX, fromY, toX, toY);
      }
    }
  };

  const handleUndoMove = async () => {
    try {
      const state = await invoke<GameState>('undo_move');
      setGameState(state);
      setMoveHistory(prev => prev.slice(0, -1));
    } catch (error) {
      console.error('Error undoing move:', error);
    }
  };

  const handleGetValidMoves = async (x: number, y: number): Promise<[number, number][]> => {
    try {
      console.log('Getting valid moves for:', x, y);
      const moves = await invoke<[number, number][]>('get_valid_moves', { x, y });
      console.log('Valid moves received:', moves);
      return moves;
    } catch (error) {
      console.error('Error getting valid moves, using fallback:', error);
      if (!gameState) return [];
      
      const piece = gameState.board.cells[y][x];
      if (!piece) return [];
      
      const moves: [number, number][] = [];
      const { piece_type, color } = piece;
      
      for (let ty = 0; ty < 10; ty++) {
        for (let tx = 0; tx < 9; tx++) {
          if (tx === x && ty === y) continue;
          const target = gameState.board.cells[ty][tx];
          if (target && target.color === color) continue;
          
          let isValid = false;
          
          if (piece_type === 'Chariot' || piece_type === 'Cannon') {
            if (tx === x || ty === y) {
              let count = 0;
              if (tx === x) {
                for (let cy = Math.min(y, ty) + 1; cy < Math.max(y, ty); cy++) {
                  if (gameState.board.cells[cy][x]) count++;
                }
              } else {
                for (let cx = Math.min(x, tx) + 1; cx < Math.max(x, tx); cx++) {
                  if (gameState.board.cells[y][cx]) count++;
                }
              }
              if (target && count === 1) isValid = true;
              if (!target && count === 0) isValid = true;
            }
          } else if (piece_type === 'Horse') {
            const dx = Math.abs(tx - x);
            const dy = Math.abs(ty - y);
            if ((dx === 1 && dy === 2) || (dx === 2 && dy === 1)) {
              let blockX = x, blockY = y;
              if (dy === 2) blockY = y + (ty > y ? 1 : -1);
              else blockX = x + (tx > x ? 1 : -1);
              if (!gameState.board.cells[blockY][blockX]) isValid = true;
            }
          } else if (piece_type === 'Soldier') {
            const forward = color === 'Red' ? -1 : 1;
            const crossed = color === 'Red' ? y <= 4 : y >= 5;
            if (ty === y + forward && tx === x) isValid = true;
            if (crossed && ty === y && Math.abs(tx - x) === 1) isValid = true;
          }
          
          if (isValid) moves.push([tx, ty]);
        }
      }
      
      return moves;
    }
  };

  const getPieceName = (piece: { piece_type: string; color: string }) => {
    const chineseNames: Record<string, Record<string, string>> = {
      General: { Red: '帅', Black: '将' },
      Advisor: { Red: '仕', Black: '士' },
      Elephant: { Red: '相', Black: '象' },
      Horse: { Red: '马', Black: '马' },
      Chariot: { Red: '车', Black: '车' },
      Cannon: { Red: '炮', Black: '炮' },
      Soldier: { Red: '兵', Black: '卒' }
    };
    return chineseNames[piece.piece_type][piece.color];
  };

  const convertMoveToNotation = (fromX: number, fromY: number, toX: number, toY: number) => {
    if (!gameState) return '';
    
    const piece = gameState.board.cells[fromY][fromX];
    if (!piece) return '';
    
    const pieceName = getPieceName(piece);
    
    const colNum = piece.color === 'Red' ? (9 - fromX) : (fromX + 1);
    
    const dx = toX - fromX;
    const dy = toY - fromY;
    
    let action: string;
    let target: string;
    
    if (dx === 0) {
      action = piece.color === 'Red' ? 
        (dy > 0 ? '退' : '进') : 
        (dy < 0 ? '退' : '进');
      target = Math.abs(dy).toString();
    } else {
      const targetColNum = piece.color === 'Red' ? (9 - toX) : (toX + 1);
      
      if (dy === 0) {
        action = '平';
        target = targetColNum.toString();
      } else if ((piece.color === 'Red' && dy < 0) || (piece.color === 'Black' && dy > 0)) {
        action = '进';
        target = targetColNum.toString();
      } else {
        action = '退';
        target = targetColNum.toString();
      }
    }
    
    return `${pieceName}${colNum}${action}${target}`;
  };

  const addMoveToHistory = (fromX: number, fromY: number, toX: number, toY: number) => {
    const moveStr = convertMoveToNotation(fromX, fromY, toX, toY);
    if (moveStr) {
      setMoveHistory(prev => [...prev, moveStr]);
    }
  };

  if (!gameState) {
    return <div className="app">Loading...</div>;
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>中国象棋</h1>
      </header>
      
      <main className="app-main">
        <div className="game-container">
          <GameStatus 
            currentTurn={gameState.current_turn}
            isInCheck={gameState.is_in_check}
            isEnded={gameState.is_ended}
            winner={gameState.winner}
          />
          
          <ChessBoard
            board={gameState.board}
            onPieceClick={handleGetValidMoves}
            onMove={handleMakeMove}
            isEnded={gameState.is_ended} // 传递游戏结束状态
          />
          
          <ControlPanel
            onNewGame={initGame}
            onUndo={handleUndoMove}
          />
          
          <HistoryPanel history={moveHistory} />
        </div>
      </main>
      
      <footer className="app-footer">
        <p>中国象棋 - 基于 Rust 和 Tauri 的桌面应用</p>
      </footer>
    </div>
  );
};

export default App;