import React, { useState } from "react";
import { useWebSocket } from "./WebSocketContext";

export const MessageSender = () => {
  const { sendMessage, message, counter } = useWebSocket();
  const [inputMessage, setInputMessage] = useState("");

  const handleSendMessage = () => {
    sendMessage(inputMessage);
  };

  return (
    <div>
      <h2>Counter: {counter}</h2> {/* Dynamically display the counter */}
      <input
        type="text"
        value={inputMessage}
        onChange={(e) => setInputMessage(e.target.value)}
        placeholder="Type your message"
      />
      <button onClick={handleSendMessage}>Send Message</button>
      {message && <p>Received from server: {message}</p>}
    </div>
  );
};
