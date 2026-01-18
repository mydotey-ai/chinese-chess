import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import React from 'react';
import './HistoryPanel.css';
const HistoryPanel = ({ history }) => {
    return (_jsxs("div", { className: "history-panel", children: [_jsx("h3", { children: "\u8D70\u68CB\u8BB0\u5F55" }), _jsx("div", { className: "history-content", children: history.length === 0 ? (_jsx("div", { className: "no-history", children: "\u6682\u65E0\u8D70\u68CB\u8BB0\u5F55" })) : (_jsx("div", { className: "history-list", children: history.map((moveStr, index) => (_jsxs("div", { className: "move-item", children: [_jsxs("span", { className: "move-number", children: [index + 1, "."] }), _jsx("span", { className: "move-text", children: moveStr })] }, index))) })) })] }));
};
export default HistoryPanel;
//# sourceMappingURL=HistoryPanel.js.map