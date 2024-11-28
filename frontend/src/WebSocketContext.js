import React, { useEffect, useState } from 'react';

const WebSocketCounter = () => {
  const [counter, setCounter] = useState(0);
  const [ws, setWs] = useState(null);

  useEffect(() => {
    // Create WebSocket connection when the component is mounted
    const socket = new WebSocket('ws://localhost:8080/echo/');
    
    // Set up event listeners for WebSocket events
    socket.onopen = () => {
      console.log('WebSocket connected');
    };

    socket.onmessage = (event) => {
      // Update counter state when a message is received
      setCounter(event.data);
    };

    socket.onclose = () => {
      console.log('WebSocket closed');
    };

    socket.onerror = (error) => {
      console.error('WebSocket error: ', error);
    };

    // Save WebSocket instance in state
    setWs(socket);

    // Cleanup WebSocket connection when the component unmounts
    return () => {
      if (socket) {
        socket.close();
      }
    };
  }, []);

  return (
    <div className="p-4 text-center">
      <h1 className="text-3xl font-bold mb-4">WebSocket Counter</h1>
      <p className="text-xl">Counter: {counter}</p>
    </div>
  );
};

export default WebSocketCounter;
