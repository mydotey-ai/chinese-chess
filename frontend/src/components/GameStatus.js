import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import React from 'react';
import './GameStatus.css';
const GameStatus = ({ currentTurn, isInCheck, isEnded, winner }) => {
    const getTurnColor = () => {
        return currentTurn === 'Red' ? 'red' : 'black';
    };
    return (_jsx("div", { className: "game-status", children: isEnded ? (_jsxs("div", { className: "game-ended", children: [_jsx("h2", { children: "\u6E38\u620F\u7ED3\u675F" }), _jsxs("div", { className: "winner", children: ["\u83B7\u80DC\u65B9: ", _jsx("span", { className: winner?.toLowerCase(), children: winner })] })] })) : (_jsxs("div", { className: "game-playing", children: [_jsxs("div", { className: "turn-info", children: ["\u5F53\u524D\u56DE\u5408: ", _jsx("span", { className: getTurnColor(), children: currentTurn })] }), isInCheck && (_jsxs("div", { className: "check-warning", children: [_jsx("span", { className: "check-icon", children: "\u26A0\uFE0F" }), "\u5C06\u519B!"] }))] })) }));
};
export default GameStatus;
//# sourceMappingURL=GameStatus.js.map