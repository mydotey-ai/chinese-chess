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
      // 验证移动：只能移动自己颜色的棋子
      const piece = gameState?.board.cells[fromY][fromX];
      if (!piece || piece.color !== gameState?.current_turn) {
        return;
      }

      // 检查目标位置是否是有效的移动位置
      const validMoves = await handleGetValidMoves(fromX, fromY);
      const isValidMove = validMoves.some(([tx, ty]) => tx === toX && ty === toY);
      
      if (!isValidMove) {
        return;
      }

      const state = await invoke<GameState>('make_move', { 
        fromX, fromY, toX, toY 
      });
      setGameState(state);
      addMoveToHistory(fromX, fromY, toX, toY);
    } catch (error) {
      console.error('Error making move:', error);
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
      // 验证：只能获取自己颜色棋子的有效移动
      const piece = gameState?.board.cells[y][x];
      if (!piece || piece.color !== gameState?.current_turn) {
        return [];
      }

      const moves = await invoke<[number, number][]>('get_valid_moves', { x, y });
      return moves;
    } catch (error) {
      console.error('Error getting valid moves:', error);
      return [];
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