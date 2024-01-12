import { Dispatch, SetStateAction } from "react";

interface Input {
    nameHandle: Dispatch<SetStateAction<string>>;
    peerHandle: Function;
}
export default function NameInput({ nameHandle, peerHandle }: Input) {
    return (
        <>
            <input
                type="text"
                placeholder="Enter name"
                onChange={(e) => nameHandle(e.target.value)}
            ></input>
            <button className="border-2 border-black mt-1.5" onClick={peerHandle()}>
                Send
            </button>
        </>
    );
}
