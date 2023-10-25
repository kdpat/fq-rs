import {Staff} from "./staff.js";
import {Fretboard} from "./fretboard.js";

const staffContainer = document.querySelector("#staff-container");

if (staffContainer) {
  const _staff = new Staff(staffContainer, 200, 130);
  // staffContainer.onclick = _ => staff.clear();
}

const fbContainer = document.querySelector("#fretboard-container");

if (fbContainer) {
  new Fretboard(fbContainer, {drawDotOnHover: true, onClick: onFbClick});
}

function onFbClick(coord) {
  console.log("clicked", coord);
}
