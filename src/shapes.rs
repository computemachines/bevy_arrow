use itertools::multizip;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::ATTRIBUTE_WEIGHT;


/*  1   3
 *  | / |
 *  2   4
 */
fn unit_quad_strip_x_splits(
    pos_x_splits: Vec<f32>,
    uv_x_splits: Vec<f32>,
    weight_x_splits: Vec<f32>,
) -> Mesh {
    let mut quads = Mesh::new(PrimitiveTopology::TriangleStrip);

    let mut v_pos = vec![];
    let mut v_uv = vec![];
    let mut v_weight = vec![];
    let mut indices = vec![];
    let mut i = 0;
    for (&pos_x_split, &uv_x_split, &weight_x_split) in
        multizip((&pos_x_splits, &uv_x_splits, &weight_x_splits))
    {
        v_pos.push([pos_x_split, 1.0, 0.0]);
        v_uv.push([uv_x_split, 1.0]);
        v_weight.push(weight_x_split);
        indices.push(i);
        i += 1;

        v_pos.push([pos_x_split, -1.0, 0.0]);
        v_uv.push([uv_x_split, 0.0]);
        v_weight.push(weight_x_split);
        indices.push(i);
        i += 1;
    }

    quads.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    quads.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_uv);
    quads.insert_attribute(ATTRIBUTE_WEIGHT, v_weight);
    quads.set_indices(Some(Indices::U32(indices)));

    quads
}

pub fn build_arrow_strip_mesh() -> Mesh {
    let mut quad = unit_quad_strip_x_splits(
        vec![-1.0, 1.0, -1.0, 1.0, -1.0, 1.0],
        vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0],
        vec![1.0, 1.0, 0.2, 0.2, 0.0, 0.0],
    );

    // let v_color = vec![
    //     [1.0, 0.0, 0.0, 1.0],
    //     [0.0, 1.0, 0.0, 1.0],
    //     [0.0, 0.0, 1.0, 1.0],
    //     [0.0, 0.0, 0.0, 1.0],
    //     [1.0, 0.0, 0.0, 1.0],
    //     [0.0, 1.0, 0.0, 1.0],
    //     [0.0, 0.0, 1.0, 1.0],
    //     [0.0, 0.0, 0.0, 1.0],
    // ];
    // quad.set_attribute(Mesh::ATTRIBUTE_COLOR, v_color);
    quad
}