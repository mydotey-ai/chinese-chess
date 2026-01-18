import React, { useState, useEffect } from 'react';
import ChessBoard from './components/ChessBoard';
import HistoryPanel from './components/HistoryPanel';
import GameStatus from './components/GameStatus';
import ControlPanel from './components/ControlPanel';
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
      console.log('=== 初始化游戏 ===');
      console.log('当前时间:', new Date().toISOString());
      console.log('Tauri API 是否可用:', typeof invoke === 'function');
      
      const state = await invoke<GameState>('new_game');
      
      console.log('=== 初始化成功 ===');
      console.log('返回状态:', state);
      
      setGameState(state);
      setMoveHistory([]);
    } catch (error) {
      console.error('=== 初始化失败 ===');
      console.error('错误类型:', typeof error);
      console.error('错误详细信息:', JSON.stringify(error, null, 2));
      
      // 提供默认状态，以避免一直显示 Loading...
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
      console.log('=== 移动棋子 ===');
      console.log('源位置:', fromX, fromY);
      console.log('目标位置:', toX, toY);
      
      const state = await invoke<GameState>('make_move', { 
        from_x: fromX, 
        from_y: fromY, 
        to_x: toX, 
        to_y: toY 
      });
      
      console.log('移动成功:', state);
      
      setGameState(state);
      addMoveToHistory(fromX, fromY, toX, toY);
    } catch (error) {
      console.error('移动失败:', error);
    }
  };

  const handleUndoMove = async () => {
    try {
      const state = await invoke<GameState>('undo_move');
      setGameState(state);
      setMoveHistory(prev => prev.slice(0, -1));
    } catch (error) {
      console.error('悔棋失败:', error);
    }
  };

  const handleGetValidMoves = async (x: number, y: number): Promise<[number, number][]> => {
    try {
      console.log('=== 获取有效移动 ===');
      console.log('位置:', x, y);
      
      const moves = await invoke<[number, number][]>('get_valid_moves', { x, y });
      
      console.log('有效移动:', moves);
      
      return moves;
    } catch (error) {
      console.error('获取有效移动失败:', error);
      return [];
    }
  };

  const addMoveToHistory = (fromX: number, fromY: number, toX: number, toY: number) => {
    const moveStr = `(${fromX},${fromY}) → (${toX},${toY})`;
    setMoveHistory(prev => [...prev, moveStr]);
  };

  // 渲染加载状态
  if (!gameState) {
    return (
      <div className="app">
        <div className="loading-container">
          <div className="loading-spinner"></div>
          <h2>Loading...</h2>
          <p>正在初始化游戏，请稍候...</p>
          <p>如果长时间没有响应，可能是 Tauri 命令调用失败</p>
          <button onClick={initGame} className="retry-button">重新初始化</button>
        </div>
      </div>
    );
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>中国象棋</h1>
        <div className="debug-info">
          <p>调试信息：</p>
          <ul>
            <li>Tauri API: {typeof invoke === 'function' ? '可用' : '不可用'}</li>
            <li>游戏状态: {JSON.stringify(gameState).substring(0, 50)}...</li>
          </ul>
        </div>
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