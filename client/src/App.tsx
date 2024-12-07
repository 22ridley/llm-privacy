import React from "react";
import { useState } from "react";
import Chat from "./components/chat.tsx";
import SignIn from "./components/signin.tsx";

function App() {
  const [token, setToken] = useState<string>("");
  const [email, setEmail] = useState<string>("");
  const [name, setName] = useState<string>("");
  const [photoURL, setPhotoURL] = useState<string>("");

  if (name == null || name === "") {
    return (
      <div>
        <SignIn
          setToken={setToken}
          setEmail={setEmail}
          setName={setName}
          setPhotoURL={setPhotoURL}
        ></SignIn>
      </div>
    );
  } else {
    return (
      <div>
        <Chat
          token={token}
          email={email}
          name={name}
          photoURL={photoURL}
        ></Chat>
      </div>
    );
  }
}

export default App;
