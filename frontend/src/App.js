import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import React, { useState, useEffect } from 'react';
import ChessBoard from './components/ChessBoard';
import HistoryPanel from './components/HistoryPanel';
import GameStatus from './components/GameStatus';
import ControlPanel from './components/ControlPanel';
import { invoke } from '@tauri-apps/api';
const App = () => {
    const [gameState, setGameState] = useState(null);
    const [moveHistory, setMoveHistory] = useState([]);
    useEffect(() => {
        // Initialize game
        initGame();
    }, []);
    const initGame = async () => {
        const state = await invoke('new_game');
        setGameState(state);
        setMoveHistory([]);
    };
    const handleMakeMove = async (fromX, fromY, toX, toY) => {
        try {
            const state = await invoke('make_move', {
                fromX, fromY, toX, toY
            });
            setGameState(state);
            addMoveToHistory(fromX, fromY, toX, toY);
        }
        catch (error) {
            console.error('Error making move:', error);
        }
    };
    const handleUndoMove = async () => {
        try {
            const state = await invoke('undo_move');
            setGameState(state);
            setMoveHistory(prev => prev.slice(0, -1));
        }
        catch (error) {
            console.error('Error undoing move:', error);
        }
    };
    const handleGetValidMoves = async (x, y) => {
        try {
            const moves = await invoke('get_valid_moves', { x, y });
            return moves;
        }
        catch (error) {
            console.error('Error getting valid moves:', error);
            return [];
        }
    };
    const addMoveToHistory = (fromX, fromY, toX, toY) => {
        const moveStr = `(${fromX},${fromY}) → (${toX},${toY})`;
        setMoveHistory(prev => [...prev, moveStr]);
    };
    if (!gameState) {
        return _jsx("div", { className: "app", children: "Loading..." });
    }
    return (_jsxs("div", { className: "app", children: [_jsx("header", { className: "app-header", children: _jsx("h1", { children: "\u4E2D\u56FD\u8C61\u68CB" }) }), _jsx("main", { className: "app-main", children: _jsxs("div", { className: "game-container", children: [_jsx(GameStatus, { currentTurn: gameState.current_turn, isInCheck: gameState.is_in_check, isEnded: gameState.is_ended, winner: gameState.winner }), _jsx(ChessBoard, { board: gameState.board, onPieceClick: handleGetValidMoves, onMove: handleMakeMove, currentTurn: gameState.current_turn }), _jsx(ControlPanel, { onNewGame: initGame, onUndo: handleUndoMove }), _jsx(HistoryPanel, { history: moveHistory })] }) }), _jsx("footer", { className: "app-footer", children: _jsx("p", { children: "\u4E2D\u56FD\u8C61\u68CB - \u57FA\u4E8E Rust \u548C Tauri \u7684\u684C\u9762\u5E94\u7528" }) })] }));
};
export default App;
//# sourceMappingURL=App.js.map