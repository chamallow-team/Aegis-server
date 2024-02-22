use web_sys::CanvasRenderingContext2d;
use crate::Map;

const NODE_COLOR: &str = "#d3d3d3";

pub(crate) fn draw(m: &Map) {
    let offset = m.get_offset();
    let scale = m.get_scale() as f64;

    let map = m.map.get_map();

    let nodes = map.get_nodes();

    clear_canvas(&m.ctx);

    for node in nodes {
        let coordinates = node.get_coordinates();

        let x = (coordinates.x() as f64 + offset.x as f64) * scale;
        let y = (coordinates.y() as f64 + offset.y as f64) * scale;

        draw_node(&m.ctx, (x, y), scale);
    }
}


fn clear_canvas(ctx: &CanvasRenderingContext2d){
    ctx.clear_rect(0.0, 0.0, 1000000.0, 1000000.0);
}

//
// fn draw_node(ctx: &CanvasRenderingContext2d, coordinates: (f64, f64), scale: f64) {
//     const DEFAULT_SIZE: f32 = 20.;
//
//     let width = DEFAULT_SIZE * scale as f32;
//
//     // draw a circle for the node
//     ctx.begin_path();
//     ctx.arc(
//         coordinates.0,
//         coordinates.1,
//         width as f64,
//         0.0,
//         std::f64::consts::PI * 2.0
//     ).expect("Cannot make a Arc for the Node");
//     ctx.set_fill_style(&NODE_COLOR.into());
//     ctx.fill();
//     ctx.close_path();
//
//     // draw a border around the circle
//
//     let border_width: f64 = 2.;
//     let border_distance: f64 = 2.;
//     let border_radius: f64 = width as f64 + border_distance + border_width / 2.0;
//
//     ctx.begin_path();
//     ctx.arc(
//         coordinates.0 + 0.5 * scale,
//         coordinates.1 + 0.5 * scale,
//         border_radius,
//         0.0,
//         std::f64::consts::PI * 2.0,
//     ).expect("Cannot make a Arc for the node border");
//     ctx.set_stroke_style(&NODE_COLOR.into());
//     ctx.set_line_width(border_width);
//     ctx.stroke();
//     ctx.close_path();
// }

fn draw_node(ctx: &CanvasRenderingContext2d, coordinates: (f64, f64), scale: f64) {
    const DEFAULT_SIZE: f32 = 20.;

    let width = DEFAULT_SIZE * scale as f32;

    // draw a circle for the node
    ctx.begin_path();
    ctx.arc(
        coordinates.0,
        coordinates.1,
        width as f64,
        0.0,
        std::f64::consts::PI * 2.0
    ).expect("Cannot make an Arc for the Node");
    ctx.set_fill_style(&NODE_COLOR.into());
    ctx.fill();
    ctx.close_path();

    // draw a border around the circle

    let border_width: f64 = 2.;
    let border_distance: f64 = 2.;
    let border_radius: f64 = width as f64 + border_distance + border_width / 2.0;

    ctx.begin_path();
    ctx.arc(
        coordinates.0,
        coordinates.1,
        border_radius,
        0.0,
        std::f64::consts::PI * 2.0,
    ).expect("Cannot make an Arc for the node border");
    ctx.set_stroke_style(&NODE_COLOR.into());
    ctx.set_line_width(border_width);
    ctx.stroke();
    ctx.close_path();
}
