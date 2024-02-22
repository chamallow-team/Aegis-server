import map_editor, { init } from './pkg/map_editor.js'
import { init_flying_boxes } from './flying_box.js'
import { init_listeners } from './map_listeners.js'

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


  map_editor().then(init_map)
})

function init_map(){
  let map = init();
  console.log(map);

  let canvas = document.getElementById("map_canvas") as HTMLCanvasElement;

  window['map'] = map;
  init_listeners(canvas);

  // set the canvas size to the size of the container
  let container = document.querySelector("#container .container#map");
  let width = container.clientWidth;
  let height = container.clientHeight;

  canvas.width = width;
  canvas.height = height;

  let resizeObserver = new ResizeObserver(function(entries){
    // change the canvas size to the new size
    let entry = entries[0];
    let width = entry.contentRect.width;
    let height = entry.contentRect.height;

    canvas.width = width;
    canvas.height = window.innerHeight;
  });

  resizeObserver.observe(container);
}
