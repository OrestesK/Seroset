"use client";
import Link from "next/link";

export default function Root() {
  const start = (): void => {
    alert("aa");
  };

  return (
    <main>
      <Link href="/register"> Register </Link>
      <Link href="/login"> Login </Link>
      <div>
        <button onClick={start}> Start </button>
        <form id="fileInfo">
          <input type="file" id="fileInput" name="files" />
        </form>
        <button disabled id="sendFile">
          Send
        </button>
        <button disabled id="abortButton">
          Abort
        </button>
      </div>

      <div className="progress">
        <div className="label">Send progress: </div>
        <progress id="sendProgress" max="0" value="0"></progress>
      </div>

      <div className="progress">
        <div className="label">Receive progress: </div>
        <progress id="receiveProgress" max="0" value="0"></progress>
      </div>

      <div id="bitrate"></div>
      <a id="download"></a>
      <span id="status"></span>
    </main>
  );
}
