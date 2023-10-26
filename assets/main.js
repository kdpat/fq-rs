import {Staff} from "./staff.js";
import {Fretboard} from "./fretboard.js";

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
