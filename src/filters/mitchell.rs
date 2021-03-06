use std::sync::Arc;

use core::filter::Filter;
use core::geometry::{Point2f, Vector2f};
use core::paramset::ParamSet;
use core::pbrt::Float;

pub struct MitchellNetravali {
    width: Float,
    height: Float,
    inv_width: Float,
    inv_height: Float,
    b: Float,
    c: Float,
}

impl MitchellNetravali {
    pub fn new(w: Float, h: Float, b: Float, c: Float) -> MitchellNetravali {
        MitchellNetravali {
            width: w,
            height: h,
            inv_width: 1.0 / w,
            inv_height: 1.0 / h,
            b,
            c,
        }
    }

    fn mitchell_1d(&self, x: Float) -> Float {
        let fx = x.abs() * 2.0;
        if fx < 1.0 {
            ((12.0 - 9.0 * self.b - 6.0 * self.c) * fx * fx * fx
                + (-18.0 + 12.0 * self.b + 6.0 * self.c) * fx * fx
                + (6.0 - 2.0 * self.b))
                * (1.0 / 6.0)
        } else if fx < 2.0 {
            ((-self.b - 6.0 * self.c) * fx * fx * fx
                + (6.0 * self.b + 30.0 * self.c) * fx * fx
                + (-12.0 * self.b - 48.0 * self.c) * fx
                + (8.0 * self.b + 24.0 * self.c))
                * (1.0 / 6.0)
        } else {
            0.0
        }
    }

    pub fn create(ps: &ParamSet) -> Arc<Filter + Sync + Send> {
        let xw = ps.find_one_float("xwidth", 2.0);
        let yw = ps.find_one_float("ywidth", 2.0);
        let b = ps.find_one_float("B", 1.0 / 3.0);
        let c = ps.find_one_float("C", 1.0 / 3.0);

        Arc::new(Self::new(xw, yw, b, c))
    }
}

impl Filter for MitchellNetravali {
    fn evaluate(&self, p: Point2f) -> Float {
        self.mitchell_1d(p.x * self.inv_width) * self.mitchell_1d(p.y * self.inv_height)
    }

    fn get_radius(&self) -> Vector2f {
        Vector2f {
            x: self.width,
            y: self.height,
        }
    }
}
