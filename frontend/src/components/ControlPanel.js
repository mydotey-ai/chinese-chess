import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import React from 'react';
import './ControlPanel.css';
const ControlPanel = ({ onNewGame, onUndo }) => {
    return (_jsxs("div", { className: "control-panel", children: [_jsx("button", { onClick: onNewGame, className: "control-button new-game", children: "\u65B0\u6E38\u620F" }), _jsx("button", { onClick: onUndo, className: "control-button undo", children: "\u6094\u68CB" })] }));
};
export default ControlPanel;
//# sourceMappingURL=ControlPanel.js.map