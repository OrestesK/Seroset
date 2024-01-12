export default function ShareInput() {
    return (
        <>
            <div>
                <input type="text" placeholder="Enter name"></input>
                <button className="border-2 border-black mt-1.5">Send</button>
            </div>
            <div>
                <button
                    className="border-2 border-black mt-1.5"
                    onClick={() => alert("receive")}
                >
                    Receive
                </button>
            </div>
        </>
    );
}
