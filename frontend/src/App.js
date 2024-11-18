import React from 'react';
import { WebSocketProvider } from './WebSocketContext';
import { MessageSender } from './MessageSender';

function App() {
  return (
    <WebSocketProvider>
      <div className="App">
        <h1>WebSocket Counter Example</h1>
        <MessageSender /> 
      </div>
    </WebSocketProvider>
  );
}

export default App;
