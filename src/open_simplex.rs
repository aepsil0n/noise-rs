// Copyright 2013 The noise-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Note that this is NOT Ken Perlin's simplex noise, as that is patent encumbered.
//! Instead, these functions use the OpenSimplex algorithm, as detailed here:
//! http://uniblock.tumblr.com/post/97868843242/noise

use std::num::{cast, Float};

use {gradient, math, Seed};

const STRETCH_CONSTANT_2D: f64 = -0.211324865405187; //(1/sqrt(2+1)-1)/2;
const SQUISH_CONSTANT_2D: f64 = 0.366025403784439; //(sqrt(2+1)-1)/2;
const STRETCH_CONSTANT_3D: f64 = -1.0 / 6.0; //(1/Math.sqrt(3+1)-1)/3;
const SQUISH_CONSTANT_3D: f64 = 1.0 / 3.0; //(Math.sqrt(3+1)-1)/3;

const NORM_CONSTANT_2D: f32 = 1.0 / 14.0;
const NORM_CONSTANT_3D: f32 = 1.0 / 14.0;

pub fn open_simplex2<T: Float>(seed: &Seed, point: &::Point2<T>) -> T {
    fn gradient<T: Float>(seed: &Seed, xs_floor: T, ys_floor: T, dx: T, dy: T) -> T {
        let zero: T = math::cast(0);

        let attn = math::cast::<_, T>(2.0_f64) - dx * dx - dy * dy;
        if attn > zero {
            let index = seed.get2::<isize>([math::cast(xs_floor), math::cast(ys_floor)]);
            let vec = gradient::get2::<T>(index);
            math::pow4(attn) * (dx * vec[0] + dy * vec[1])
        } else {
            zero
        }
    }

    let zero: T = math::cast(0);
    let one: T = math::cast(1);
    let two: T = math::cast(2);
    let squish_constant: T = math::cast(SQUISH_CONSTANT_2D);

    //Place input coordinates onto grid.
    let stretch_offset = (point[0] + point[1]) * math::cast(STRETCH_CONSTANT_2D);
    let xs = point[0] + stretch_offset;
    let ys = point[1] + stretch_offset;

    //Floor to get grid coordinates of rhombus (stretched square) super-cell origin.
    let mut xs_floor = xs.floor();
    let mut ys_floor = ys.floor();

    //Skew out to get actual coordinates of rhombus origin. We'll need these later.
    let squish_offset = (xs_floor + ys_floor) * squish_constant;
    let x_floor = xs_floor + squish_offset;
    let y_floor = ys_floor + squish_offset;

    //Compute grid coordinates relative to rhombus origin.
    let xs_frac = xs - xs_floor;
    let ys_frac = ys - ys_floor;

    //Sum those together to get a value that determines which region we're in.
    let frac_sum = xs_frac + ys_frac;

    //Positions relative to origin point.
    let mut dx0 = point[0] - x_floor;
    let mut dy0 = point[1] - y_floor;

    let mut value: T = zero;

    //Contribution (1,0)
    let dx1 = dx0 - one - squish_constant;
    let dy1 = dy0 - zero - squish_constant;
    value = value + gradient(seed, xs_floor + one, ys_floor + zero, dx1, dy1);

    //Contribution (0,1)
    let dx2 = dx0 - zero - squish_constant;
    let dy2 = dy0 - one - squish_constant;
    value = value + gradient(seed, xs_floor + zero, ys_floor + one, dx2, dy2);

    if frac_sum > one {
        xs_floor = xs_floor + one;
        ys_floor = ys_floor + one;
        dx0 = dx0 - one - two * squish_constant;
        dy0 = dy0 - one - two * squish_constant;
    }

    //Contribution (0,0) or (1,1)
    value = value + gradient(seed, xs_floor, ys_floor, dx0, dy0);

    value * math::cast(NORM_CONSTANT_2D)
}

pub fn open_simplex3<T: Float>(seed: &Seed, point: &::Point3<T>) -> T {
    fn gradient<T: Float>(seed: &Seed, xs_floor: T, ys_floor: T, zs_floor: T, dx: T, dy: T, dz: T) -> T {
        let zero: T = math::cast(0);

        let attn = math::cast::<_, T>(2.0_f64) - dx * dx - dy * dy - dz * dz;
        if attn > zero {
            let index = seed.get3::<isize>([math::cast(xs_floor), math::cast(ys_floor), math::cast(zs_floor)]);
            let vec = gradient::get3::<T>(index);
            math::pow4(attn) * (dx * vec[0] + dy * vec[1] + dz * vec[2])
        } else {
            zero
        }
    }

    let zero: T = math::cast(0);
    let one: T = math::cast(1);
    let two: T = math::cast(2);
    let three: T = math::cast(3);
    let squish_constant: T = math::cast(SQUISH_CONSTANT_3D);

    //Place input coordinates on simplectic honeycomb.
    let stretch_offset = (point[0] + point[1] + point[2]) * math::cast(STRETCH_CONSTANT_3D);
    let xs = point[0] + stretch_offset;
    let ys = point[1] + stretch_offset;
    let zs = point[2] + stretch_offset;

    //Floor to get simplectic honeycomb coordinates of rhombohedron (stretched cube) super-cell origin.
    let xsb = xs.floor();
    let ysb = ys.floor();
    let zsb = zs.floor();

    //Skew out to get actual coordinates of rhombohedron origin. We'll need these later.
    let squish_offset = (xsb + ysb + zsb) * squish_constant;
    let xb = xsb + squish_offset;
    let yb = ysb + squish_offset;
    let zb = zsb + squish_offset;

    //Compute simplectic honeycomb coordinates relative to rhombohedral origin.
    let xins = xs - xsb;
    let yins = ys - ysb;
    let zins = zs - zsb;

    //Sum those together to get a value that determines which region we're in.
    let in_sum = xins + yins + zins;

    //Positions relative to origin point.
    let mut dx0 = point[0] - xb;
    let mut dy0 = point[1] - yb;
    let mut dz0 = point[2] - zb;

    let mut value = zero;
    if in_sum <= one { //We're inside the tetrahedron (3-Simplex) at (0,0,0)
        //Contribution (0,0,0)
        value = value + gradient(seed, xsb, ysb, zsb, dx0, dy0, dz0);

        //Contribution (1,0,0)
        let dx1 = dx0 - one - squish_constant;
        let dy1 = dy0 - zero - squish_constant;
        let dz1 = dz0 - zero - squish_constant;
        value = value + gradient(seed, xsb + one, ysb, zsb, dx1, dy1, dz1);

        //Contribution (0,1,0)
        let dx2 = dx0 - zero - squish_constant;
        let dy2 = dy0 - one - squish_constant;
        let dz2 = dz1;
        value = value + gradient(seed, xsb, ysb + one, zsb, dx2, dy2, dz2);

        //Contribution (0,0,1)
        let dx3 = dx2;
        let dy3 = dy1;
        let dz3 = dz0 - one - squish_constant;
        value = value + gradient(seed, xsb, ysb, zsb + one, dx3, dy3, dz3);
    } else if in_sum >= two { //We're inside the tetrahedron (3-Simplex) at (1,11)
        //Contribution (1,1,0)
        let dx3 = dx0 - one - two * squish_constant;
        let dy3 = dy0 - one - two * squish_constant;
        let dz3 = dz0 - zero - two * squish_constant;
        value = value + gradient(seed, xsb + one, ysb + one, zsb, dx3, dy3, dz3);

        //Contribution (1,0,1)
        let dx2 = dx3;
        let dy2 = dy0 - zero - two * squish_constant;
        let dz2 = dz0 - one - two * squish_constant;
        value = value + gradient(seed, xsb + one, ysb, zsb + one, dx2, dy2, dz2);

        //Contribution (0,1,1)
        let dx1 = dx0 - zero - two * squish_constant;
        let dy1 = dy3;
        let dz1 = dz2;
        value = value + gradient(seed, xsb, ysb + one, zsb + one, dx1, dy1, dz1);

        //Contribution (1,1,1)
        dx0 = dx0 - one - three * squish_constant;
        dy0 = dy0 - one - three * squish_constant;
        dz0 = dz0 - one - three * squish_constant;
        value = value + gradient(seed, xsb + one, ysb + one, zsb + one, dx0, dy0, dz0);
    } else { //We're inside the octahedron (Rectified 3-Simplex) in between.
        //Contribution (1,0,0)
        let dx1 = dx0 - one - squish_constant;
        let dy1 = dy0 - zero - squish_constant;
        let dz1 = dz0 - zero - squish_constant;
        value = value + gradient(seed, xsb + one, ysb, zsb, dx1, dy1, dz1);

        //Contribution (0,1,0)
        let dx2 = dx0 - zero - squish_constant;
        let dy2 = dy0 - one - squish_constant;
        let dz2 = dz1;
        value = value + gradient(seed, xsb, ysb + one, zsb, dx2, dy2, dz2);

        //Contribution (0,0,1)
        let dx3 = dx2;
        let dy3 = dy1;
        let dz3 = dz0 - one - squish_constant;
        value = value + gradient(seed, xsb, ysb, zsb + one, dx3, dy3, dz3);

        //Contribution (1,1,0)
        let dx4 = dx0 - one - two * squish_constant;
        let dy4 = dy0 - one - two * squish_constant;
        let dz4 = dz0 - zero - two * squish_constant;
        value = value + gradient(seed, xsb + one, ysb + one, zsb, dx4, dy4, dz4);

        //Contribution (1,0,1)
        let dx5 = dx4;
        let dy5 = dy0 - zero - two * squish_constant;
        let dz5 = dz0 - one - two * squish_constant;
        value = value + gradient(seed, xsb + one, ysb, zsb + one, dx5, dy5, dz5);

        //Contribution (0,1,1)
        let dx6 = dx0 - zero - two * squish_constant;
        let dy6 = dy4;
        let dz6 = dz5;
        value = value + gradient(seed, xsb, ysb + one, zsb + one, dx6, dy6, dz6);
    }

    return value * math::cast(NORM_CONSTANT_3D);
}
