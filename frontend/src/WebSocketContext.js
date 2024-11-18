import React, { createContext, useContext, useEffect, useState } from 'react';

const WebSocketContext = createContext(null);

export const WebSocketProvider = ({ children }) => {
    const [socket, setSocket] = useState(null);
    const [message, setMessage] = useState(null);
    const [isConnected, setIsConnected] = useState(false);
    const [retryCount, setRetryCount] = useState(0);

    const connectWebSocket = () => {
        const ws = new WebSocket('ws://127.0.0.1:8080/echo');

        ws.onopen = () => {
            console.log('WebSocket connection established');
            setIsConnected(true);
            setRetryCount(0); // Reset retry count
        };

        ws.onmessage = (event) => {
            console.log('Message from server:', event.data);
            setMessage(event.data);
        };

        ws.onerror = (error) => {
            console.log('WebSocket error:', error);
            setIsConnected(false);
        };

        ws.onclose = () => {
            console.log('WebSocket connection closed');
            setIsConnected(false);
            if (retryCount < 5) {
                // Retry after a delay if the connection failed
                setTimeout(() => {
                    setRetryCount(retryCount + 1);
                    connectWebSocket();
                }, 3000); // Retry every 3 seconds
            }
        };

        setSocket(ws);
    };

    // Try to connect when the component mounts
    useEffect(() => {
        connectWebSocket();

        // Cleanup WebSocket connection on component unmount
        return () => {
            if (socket) {
                socket.close();
            }
        };
    }, [retryCount]); // Re-run when retryCount changes

    const sendMessage = (msg) => {
        if (socket && socket.readyState === WebSocket.OPEN) {
            console.log('Sending message to server:', msg);
            socket.send(msg);
        }
    };

    return (
        <WebSocketContext.Provider value={{ socket, sendMessage, message, isConnected }}>
            {children}
        </WebSocketContext.Provider>
    );
};

export const useWebSocket = () => {
    return useContext(WebSocketContext);
};

export const MessageSender = () => {
    const { sendMessage, message, isConnected } = useWebSocket();

    const handleSendMessage = () => {
        const msg = 'Hello, server!';
        sendMessage(msg);
    };

    return (
        <div>
            <button onClick={handleSendMessage} disabled={!isConnected}>Send Message</button>
            {!isConnected && <p>Attempting to connect to WebSocket...</p>}
            {message && <p>Received from server: {message}</p>}
        </div>
    );
};
