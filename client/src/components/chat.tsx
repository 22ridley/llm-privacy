import React, { useEffect, useState } from "react";
import "../styles/chat.css";

class Message {
  user: boolean;
  text: string;
  constructor(text, user) {
    this.text = text;
    this.user = user;
  }
}

export default function Chat() {
  const [chatText, setChatText] = useState<string>("");
  const [chatHistory, setChatHistory] = useState<Message[]>([]);

  // Submit chat to backend and receive response
  const submitChat = async () => {
    const data = {
      chat: chatText,
    };
    try {
      const message = new Message(chatText, true);
      setChatText("");
      const filler = new Message("...", false);
      setChatHistory(chatHistory.concat([message, filler]));
      const response = await fetch("http://localhost:8000/submit", {
        method: "POST", // Specify the HTTP method
        headers: {
          "Content-Type": "application/json", // Inform the server you're sending JSON
        },
        body: JSON.stringify(data), // Convert JS object to JSON string
      });

      if (response.ok) {
        const result = await response.json(); // Parse JSON response
        console.log("Success:", result);
        const ai_message = new Message(result.message, false);
        setChatHistory(chatHistory.concat([message, ai_message]));
      } else {
        console.error("Error:", response.status, response.statusText);
      }
    } catch (error) {
      console.error("Fetch error:", error);
    }
  };

  // Trigger submitChat on enter key
  const handleKeyPress = (event) => {
    // look for the `Enter` keyCode
    if (event.keyCode === 13 || event.which === 13) {
      submitChat();
    }
  };

  return (
    <div className="chat">
      <div className="history">
        {chatHistory.map((message, id) => (
          <div
            key={id}
            className={message.user === true ? "user-message" : "bot-message"}
          >
            {message.text}
            {message.user}
          </div>
        ))}
      </div>
      <div className="input">
        <input
          type="text"
          className="box"
          placeholder="Type your message here..."
          value={chatText}
          onChange={(ev) => setChatText(ev.target.value)}
          onKeyDown={handleKeyPress}
        ></input>
        <button onClick={submitChat} className="submit">
          Submit
        </button>
      </div>
    </div>
  );
}
