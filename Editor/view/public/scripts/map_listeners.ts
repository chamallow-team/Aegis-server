import { Offset } from './pkg/map_editor.js'

function init_listeners(canvas: HTMLCanvasElement){
  // canvas.addEventListener("click", function(m){
  //   if (m.target === canvas)
  //     onclick()
  // });
  canvas.addEventListener("mousemove", mouse_move);

  document.addEventListener("wheel", function(e){
    if (e.target === canvas)
      wheel(e)
  });

  canvas.addEventListener("mousedown", function(e){
    if (e.target === canvas)
      start_dragging(canvas)
  });

  document.getElementById("add_new_node_btn")
    .addEventListener("click", add_node)
}

let cursor_position = { x: 0, y: 0 };

function mouse_move(e: MouseEvent){
  e.preventDefault();

  cursor_position.x = e.clientX;
  cursor_position.y = e.clientY;
}

/**
 * This method will be used to "move" the offset of the map
 *
 * If the dragging last less than 125ms, it will be considered as a click
 */
function start_dragging(canvas: HTMLCanvasElement){
  // clone the position
  let positions = { ...cursor_position };
  let n = Date.now();

  function handle_drag(event: MouseEvent) {
    let offset = window['map'].get_offset();
    let scale = window['map'].get_scale();

    let newOffsetX = (event.clientX - positions.x) / scale;
    let newOffsetY = (event.clientY - positions.y) / scale;

    positions.x = event.clientX;
    positions.y = event.clientY;


    window['map'].set_offset(offset.x + newOffsetX, offset.y + newOffsetY);
    window['map'].update_canvas();
  }


  function stop_drag() {
    document.removeEventListener('mousemove', handle_drag);
    document.removeEventListener('mouseup', stop_drag);

    if (Date.now() - n < 125)
      onclick();
  }

  document.addEventListener('mousemove', handle_drag);
  document.addEventListener('mouseup', stop_drag);
}

function onclick(){
  let offset: Offset = window['map'].get_offset();

  (document.getElementById("new_node_x") as HTMLInputElement).value = `${offset.x + cursor_position.x}`;
  (document.getElementById("new_node_y") as HTMLInputElement).value = `${offset.y + cursor_position.y}`;

  let new_node_html = document.getElementById("new_node");

  new_node_html.style.setProperty('--t', `${cursor_position.y}px`);
  new_node_html.style.setProperty('--l', `${cursor_position.x}px`);
  new_node_html.style.removeProperty("display");
}

function add_node(){
  let x = parseInt((document.getElementById("new_node_x") as HTMLInputElement).value);
  let y = parseInt((document.getElementById("new_node_y") as HTMLInputElement).value);

  let land_type = parseInt((document.getElementById("new_node_land_type") as HTMLInputElement).value);

  let r = window["map"].add_node(land_type, x, y);

  if (!r)
    alert("The land type is invalid")
  else {
    close_new_node();
    window["map"].update_canvas();
  }
}

function close_new_node(){
  let new_node_html = document.getElementById("new_node");
  new_node_html.style.setProperty("display", "none");
}

function wheel(s: WheelEvent) {
  close_new_node();
  const STEP = 0.05;

  let currentScale = window["map"].get_scale();
  let offset = window["map"].get_offset();

  let mouseX = s.clientX;
  let mouseY = s.clientY;

  if (s.deltaY > 0) {
    currentScale -= STEP;
  } else {
    currentScale += STEP;
  }

  if (currentScale !== window["map"].get_scale()) {
    let newOffsetX = offset.x - (mouseX - offset.x) * (currentScale - window["map"].get_scale());
    let newOffsetY = offset.y - (mouseY - offset.y) * (currentScale - window["map"].get_scale());

    window["map"].set_offset(newOffsetX, newOffsetY);
  }

  window["map"].set_scale(currentScale);
  window["map"].update_canvas();
}






export { init_listeners }