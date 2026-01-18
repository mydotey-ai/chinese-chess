import React, { useState, useEffect } from 'react';
import ChessBoard from './components/ChessBoard.tsx';
import HistoryPanel from './components/HistoryPanel.tsx';
import GameStatus from './components/GameStatus.tsx';
import ControlPanel from './components/ControlPanel.tsx';
import { invoke } from '@tauri-apps/api';

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
      console.log('Trying to invoke new_game...');
      const state = await invoke<GameState>('new_game');
      console.log('new_game result:', state);
      setGameState(state);
      setMoveHistory([]);
    } catch (error) {
      console.error('Error initializing game:', error);
      console.error('Error type:', typeof error);
      console.error('Error details:', JSON.stringify(error, null, 2));
      
      // 提供一个默认的游戏状态，以避免一直显示 Loading...
      const defaultState: GameState = {
        board: {
          cells: Array(10).fill(null).map(() => Array(9).fill(null))
        },
        current_turn: 'Red',
        is_in_check: false,
        is_ended: false,
        winner: null
      };
      
      setGameState(defaultState);
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
      const moves = await invoke<[number, number][]>('get_valid_moves', { x, y });
      return moves;
    } catch (error) {
      console.error('Error getting valid moves:', error);
      // 显示错误信息给用户
      return [];
    }
  };

  const addMoveToHistory = (fromX: number, fromY: number, toX: number, toY: number) => {
    const moveStr = `(${fromX},${fromY}) → (${toX},${toY})`;
    setMoveHistory(prev => [...prev, moveStr]);
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