"use client";

import { useState } from "react";
import { Peer } from "peerjs";
import NameInput from "../(components)/name_input";
import ShareInput from "../(components)/share_input";
import Login from "../(auth)/login/page";

export default function Share() {
  var peer;
  const [name, setName] = useState<string>("no_name");
  const [ready, setReady] = useState<boolean>(false);

  function initialize_peer() {
    peer = new Peer(name);
    alert(peer.id);
    setReady(true);
  }

  return (
    <>
      <Login />
      <h1 className="text-center border-black border-2 mt-2 text-2xl w-1/2 ml-auto mr-auto">
        Share
      </h1>
      {!ready ? (
        <NameInput nameHandle={setName} peerHandle={() => initialize_peer} />
      ) : (
        <ShareInput />
      )}
    </>
  );
}
