import {TOKEN} from "./token.js";
import {Staff} from "./staff.js";
import {Fretboard} from "./fretboard.js";

/**
 * Return the last part of the path.
 * getPathEnd("/games/42") === "42"
 */
function getPathEnd(pathname) {
  const parts = pathname.split("/");
  return parts[parts.length - 1];
}

const GAME_ID = getPathEnd(location.pathname);
if (!GAME_ID) throw new Error("could not extract game id from path");

let socket;

const connectMsg = (token, channel) => JSON.stringify({token, channel});

if (TOKEN) {
  socket = new WebSocket("ws://localhost:4000/ws");

  socket.onopen = event => {
    console.log("ws connected:", event);
    socket.send(connectMsg(TOKEN, GAME_ID));
  }
  socket.onmessage = event => {
    console.log("msg recv:", event);
  }
  socket.onclose = event => {
    console.log("ws closed:", event);
  }
  socket.onerror = event => {
    console.error("ws error:", event);
  }
} else {
  throw new Error("TOKEN is null");
}

let noteToDraw;

const noteData = document.querySelector("#note-data");
if (noteData) {
  noteToDraw = noteData.dataset.note;
}

const STAFF_WIDTH = 200;
const STAFF_HEIGHT = 130;

const staffContainer = document.querySelector("#staff-container");
if (staffContainer) {
  const staff = new Staff(staffContainer, STAFF_WIDTH, STAFF_HEIGHT, noteToDraw);
  staffContainer.onclick = () => staff.clear();
}

function onFbClick(coord) {
  console.log("clicked", coord);
}

const fbContainer = document.querySelector("#fretboard-container");
if (fbContainer) {
  new Fretboard(fbContainer, {drawDotOnHover: true, onClick: onFbClick});
}

const startGameBtn = document.querySelector("#start-game-btn");
if (startGameBtn) {
  startGameBtn.onclick = () => {
    const msg = {StartGame: {token: TOKEN, game_id: parseInt(GAME_ID)}};
    socket.send(JSON.stringify(msg));
  };
}
