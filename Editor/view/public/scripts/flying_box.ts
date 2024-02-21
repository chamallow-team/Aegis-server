// Flying box are elements that can be moved freely around the window, allowing users to move it as he wants

function init_flying_boxes(){
  document.querySelectorAll("#container .container .flying_box")
    .forEach(init_box)
}

function init_box(elm: HTMLElement) {
  let head = elm.querySelector(".head");

  head.addEventListener('mousedown', (e: MouseEvent) => {
    e.preventDefault();
    start_dragging(elm, e.clientX, e.clientY);
  });

}

function start_dragging(
  box: HTMLElement,
  startX: number,
  startY: number
) {
  let offsetX: number, offsetY: number;
  let n = Date.now();

  function handle_drag(event: MouseEvent) {
    const newOffsetX = startX - event.clientX;
    const newOffsetY = startY - event.clientY;

    let allowed_rectangle = document.querySelector("#container").getBoundingClientRect();

    box.style.setProperty('--t', `${offsetY - newOffsetY}px`);
    box.style.setProperty('--l', `${offsetX - newOffsetX}px`);

    let box_rect = box.getBoundingClientRect();

    if (box_rect.top < allowed_rectangle.top)
      box.style.setProperty('--t', `${allowed_rectangle.top}px`);

    if (box_rect.left < allowed_rectangle.left)
      box.style.setProperty('--l', `${allowed_rectangle.left}px`);

    if (box_rect.bottom > allowed_rectangle.bottom)
      box.style.setProperty(
        '--t',
        `${allowed_rectangle.bottom - box_rect.height}px`
      )

    if (box_rect.right >= allowed_rectangle.right) {
      let newLeft = allowed_rectangle.right - box_rect.width;
      if (newLeft < allowed_rectangle.left) {
        newLeft = allowed_rectangle.left;
      }
      box.style.setProperty('--l', `${newLeft}px`);
    }
  }

  function stop_drag() {
    document.removeEventListener('mousemove', handle_drag);
    document.removeEventListener('mouseup', stop_drag);

    if (Date.now() - n < 125)
      head_clicked(box);
  }

  document.addEventListener('mousemove', handle_drag);
  document.addEventListener('mouseup', stop_drag);

  offsetY = parseInt(getComputedStyle(box).getPropertyValue('--t')) || 0;
  offsetX = parseInt(getComputedStyle(box).getPropertyValue('--l')) || 0;
}

function head_clicked(box: HTMLElement) {
  if (box.classList.contains("open")) {
    box.classList.remove("open");
  } else {
    box.classList.add("open");
  }
}



export { init_flying_boxes }