import React from 'react';
import './HistoryPanel.css';

interface HistoryPanelProps {
  history: string[];
}

const HistoryPanel: React.FC<HistoryPanelProps> = ({
  history
}) => {
  return (
    <div className="history-panel">
      <h3>走棋记录</h3>
      <div className="history-content">
        {history.length === 0 ? (
          <div className="no-history">
            暂无走棋记录
          </div>
        ) : (
          <div className="history-list">
            {history.map((moveStr, index) => (
              <div key={index} className="move-item">
                <span className="move-number">{index + 1}.</span>
                <span className="move-text">{moveStr}</span>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default HistoryPanel;