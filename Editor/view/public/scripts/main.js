import map_editor, { init } from './pkg/map_editor.js';
import { init_flying_boxes } from './flying_box.js';
//
//
//    CONTAINERS NAVIGATION
//
//
function change_container(id) {
    document.querySelectorAll("#container .container")
        .forEach(function (e) { return e.classList.remove("selected"); });
    var container = document.querySelector("#container section#".concat(id));
    if (!container)
        throw new Error("No section container with the ID '".concat(id, "'"));
    container.classList.add("selected");
}
function load_container_nav() {
    document.querySelectorAll("nav .cases svg")
        .forEach(function (svg) {
        svg.addEventListener("click", function () { return change_container(svg.classList.item(0)); });
    });
}
//
//
//  Event Listener
//
//
document.addEventListener("DOMContentLoaded", function () {
    load_container_nav();
    init_flying_boxes();
    map_editor().then(init_map);
});
function init_map() {
    var map = init();
    console.log(map);
    map.move_view(10, 0);
    var canvas = document.getElementById("map_canvas");
    // set the canvas size to the size of the container
    var container = document.querySelector("#container .container#map");
    var width = container.clientWidth;
    var height = container.clientHeight;
    canvas.width = width;
    canvas.height = height;
    var resizeObserver = new ResizeObserver(function (entries) {
        // change the canvas size to the new size
        var entry = entries[0];
        var width = entry.contentRect.width;
        var height = entry.contentRect.height;
        canvas.width = width;
        canvas.height = window.innerHeight;
    });
    resizeObserver.observe(container);
}
