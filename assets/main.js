import {Staff} from "./staff.js";
import {Fretboard} from "./fretboard.js";

const socket = new WebSocket("ws://localhost:4000/ws");

socket.onopen = event => {
  console.log("ws connected:", event);
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

let noteToDraw;

const noteData = document.querySelector("#note-data");
if (noteData) {
  noteToDraw = noteData.dataset.note;
}

const staffContainer = document.querySelector("#staff-container");
if (staffContainer) {
  const staff = new Staff(staffContainer, 200, 130, noteToDraw);
  staffContainer.onclick = () => staff.clear();
}

const fbContainer = document.querySelector("#fretboard-container");
if (fbContainer) {
  new Fretboard(fbContainer, {drawDotOnHover: true, onClick: onFbClick});
}

function onFbClick(coord) {
  console.log("clicked", coord);
}

const startGameBtn = document.querySelector("#start-game-btn");
if (startGameBtn) {
  startGameBtn.onclick = () => {
    socket.send(
      JSON.stringify({type: "StartGame", user_id: 42})
    );
  };
}
