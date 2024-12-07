import { initializeApp } from "firebase/app";
import { getAuth, GoogleAuthProvider, signInWithPopup } from "firebase/auth";
import { firebaseConfig } from "./firebase.tsx";
import React, { Dispatch, SetStateAction, useEffect, useState } from "react";
import "../styles/signin.css";

const app = initializeApp(firebaseConfig);
var provider = new GoogleAuthProvider();
const auth = getAuth();

interface SignInProps {
  setToken: Dispatch<SetStateAction<string>>;
  setEmail: Dispatch<SetStateAction<string>>;
  setName: Dispatch<SetStateAction<string>>;
  setPhotoURL: Dispatch<SetStateAction<string>>;
}

export default function SignIn(props: SignInProps) {
  const signin = async () => {
    signInWithPopup(auth, provider)
      .then(async (result) => {
        // This gives you a Google Access Token. You can use it to access the Google API.
        const credential = GoogleAuthProvider.credentialFromResult(result);
        if (credential) {
          const user = result.user;
          const token: string = await user.getIdToken();
          const email = user.email;
          const name = user.displayName;
          const photoURL = user.photoURL;
          console.log(user, token, email, name);
          if (token !== undefined) {
            props.setToken(token);
          }
          if (email !== null) {
            props.setEmail(email);
          }
          if (name !== null) {
            props.setName(name);
          }
          if (photoURL !== null) {
            props.setPhotoURL(photoURL);
          }
        }
      })
      .catch((error) => {
        console.log("Error: ", error);
      });
  };

  return (
    <div className="all">
      <div className="panel">
        <p className="signin_label">Sign In</p>
        <button className="signin_button" onClick={signin}>
          Sign In With Google
        </button>
      </div>
    </div>
  );
}
