import React from 'react';
import './ControlPanel.css';

interface ControlPanelProps {
  onNewGame: () => void;
  onUndo: () => void;
}

const ControlPanel: React.FC<ControlPanelProps> = ({
  onNewGame,
  onUndo
}) => {
  return (
    <div className="control-panel">
      <button onClick={onNewGame} className="control-button new-game">
        新游戏
      </button>
      <button onClick={onUndo} className="control-button undo">
        悔棋
      </button>
    </div>
  );
};

export default ControlPanel;