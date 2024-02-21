import { Map } from './pkg/map_editor.js';
import { init_flying_boxes } from './flying_box.js'

let map = new Map();

//
//
//    CONTAINERS NAVIGATION
//
//

function change_container(id: string) {
  document.querySelectorAll("#container .container")
    .forEach(e => e.classList.remove("selected"));

  let container = document.querySelector(`#container section#${id}`);
  if (!container)
    throw new Error(`No section container with the ID '${id}'`);

  container.classList.add("selected");
}

function load_container_nav(){
  document.querySelectorAll("nav .cases svg")
    .forEach(function(svg){
      svg.addEventListener(
        "click",
        () => change_container(svg.classList.item(0))
      );
    })
}

//
//
//  Event Listener
//
//

document.addEventListener("DOMContentLoaded", function(){
  load_container_nav();

  init_flying_boxes();
})