import React, { createContext, useContext, useEffect, useState } from 'react';

// Create a WebSocket Context
const WebSocketContext = createContext(null);

// WebSocket Provider Component
export const WebSocketProvider = ({ children }) => {
    const [socket, setSocket] = useState(null);
    const [message, setMessage] = useState(null);
    const [counter, setCounter] = useState(0);  // Counter state for the server's counter

    // Establish WebSocket connection
    useEffect(() => {
        const ws = new WebSocket('ws://127.0.0.1:8080/echo');

        ws.onopen = () => {
            console.log('WebSocket connection established');
        };

        ws.onmessage = (event) => {
            console.log('Message from server:', event.data);
            const serverMessage = event.data;

            // Parse the server response to extract the counter
            const parts = serverMessage.split(' | Counter: ');
            if (parts.length === 2) {
                setMessage(parts[0]);
                setCounter(parseInt(parts[1], 10));  // Update the counter
            } else {
                setMessage(serverMessage);  // In case the message format changes
            }
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
            console.log('Sending message to server:', msg);
            socket.send(msg);
        }
    };

    return (
        <WebSocketContext.Provider value={{ sendMessage, message, counter }}>
            {children}
        </WebSocketContext.Provider>
    );
};

// Custom hook to access the WebSocket context
export const useWebSocket = () => {
    return useContext(WebSocketContext);
};
