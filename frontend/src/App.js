import React from 'react';
import { WebSocketProvider, MessageSender } from './WebSocketContext';

function App() {
  return (
    <WebSocketProvider>
        <h1>WebSocket Example</h1>
        <MessageSender />
    </WebSocketProvider>
  );
}

export default App;
