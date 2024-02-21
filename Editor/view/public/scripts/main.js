import { Map } from './pkg/map_editor.js';
import { init_flying_boxes } from './flying_box.js';
var map = new Map();
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
});
