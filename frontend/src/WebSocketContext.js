import React, { createContext, useContext, useEffect, useState } from 'react';

// Create a WebSocket Context
const WebSocketContext = createContext(null);

// WebSocket Provider Component
export const WebSocketProvider = ({ children }) => {
    const [socket, setSocket] = useState(null);
    const [message, setMessage] = useState(null);

    // Establish WebSocket connection
    useEffect(() => {
        const ws = new WebSocket('ws://127.0.0.1:8080/echo');

        ws.onopen = () => {
            console.log('WebSocket connection established');
        };

        ws.onmessage = (event) => {
            console.log('Message from server:', event.data);
            setMessage(event.data);
        };

        ws.onerror = (error) => {
            console.log('WebSocket error:', error);
        };

        ws.onclose = () => {
            console.log('WebSocket connection closed');
        };

        setSocket(ws);

        // Cleanup WebSocket connection on component unmount
        return () => {
            ws.close();
        };
    }, []);

    // Send a message to the WebSocket server
    const sendMessage = (msg) => {
        if (socket && socket.readyState === WebSocket.OPEN) {
            socket.send(msg);
        }
    };

    return (
        <WebSocketContext.Provider value={{ socket, sendMessage, message }}>
            {children}
        </WebSocketContext.Provider>
    );
};

// Custom hook to access the WebSocket context
export const useWebSocket = () => {
    return useContext(WebSocketContext);
};
