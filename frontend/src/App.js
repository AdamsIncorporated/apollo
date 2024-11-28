import React from 'react';
import { WebSocketProvider } from './WebSocketContext';
import { MessageSender } from './MessageSender';

function App() {
  return (
    <WebSocketProvider>
      <div className="App" style={{margin: '5%'}}>
        <h1>WebSocket Stock Example</h1>
        <MessageSender /> 
      </div>
    </WebSocketProvider>
  );
}

export default App;
