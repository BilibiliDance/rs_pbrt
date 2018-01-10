// std
use std::f32::consts::PI;
use std::sync::Arc;
// pbrt
use core::efloat::EFloat;
use core::efloat::quadratic_efloat;
use core::geometry::{Bounds3f, Normal3f, Point2f, Point3f, Ray, Vector3f};
use core::geometry::{nrm_normalize, nrm_abs_dot_vec3, pnt3_distance, pnt3_distance_squared,
                     pnt3_lerp, pnt3_offset_ray_origin, spherical_direction_vec3,
                     vec3_coordinate_system, vec3_cross_vec3, vec3_dot_vec3, vec3_normalize};
use core::interaction::{Interaction, InteractionCommon, SurfaceInteraction};
use core::material::Material;
use core::pbrt::Float;
use core::pbrt::{clamp_t, gamma, lerp, radians};
use core::sampling::{uniform_cone_pdf, uniform_sample_sphere};
use core::shape::Shape;
use core::transform::Transform;

// see curve.h

#[derive(Debug, Clone, PartialEq)]
pub enum CurveType {
    Flat,
    Cylinder,
    Ribbon,
}

#[derive(Clone)]
pub struct CurveCommon {
    pub curve_type: CurveType,
    pub cp_obj: [Point3f; 4],
    pub width: [Float; 2],
    pub n: [Normal3f; 2],
    pub normal_angle: Float,
    pub inv_sin_normal_angle: Float,
}

#[derive(Clone)]
pub struct Curve {
    pub common: Arc<CurveCommon>,
    pub u_min: Float,
    pub u_max: Float,
    // inherited from class Shape (see shape.h)
    object_to_world: Transform,
    world_to_object: Transform,
    reverse_orientation: bool,
    transform_swaps_handedness: bool,
    pub material: Option<Arc<Material + Send + Sync>>,
}

impl Shape for Curve {
    fn object_bound(&self) -> Bounds3f {
        // compute object-space control points for curve segment, _cpObj_
        // Point3f cpObj[4];
        // cpObj[0] = BlossomBezier(common->cpObj, uMin, uMin, uMin);
        // cpObj[1] = BlossomBezier(common->cpObj, uMin, uMin, uMax);
        // cpObj[2] = BlossomBezier(common->cpObj, uMin, uMax, uMax);
        // cpObj[3] = BlossomBezier(common->cpObj, uMax, uMax, uMax);
        // Bounds3f b =
        //     Union(Bounds3f(cpObj[0], cpObj[1]), Bounds3f(cpObj[2], cpObj[3]));
        // Float width[2] = {Lerp(uMin, common->width[0], common->width[1]),
        //                   Lerp(uMax, common->width[0], common->width[1])};
        // return Expand(b, std::max(width[0], width[1]) * 0.5f);
        // TODO
        Bounds3f::default()
    }
    fn world_bound(&self) -> Bounds3f {
        // in C++: Bounds3f Shape::WorldBound() const { return (*ObjectToWorld)(ObjectBound()); }
        self.object_to_world.transform_bounds(self.object_bound())
    }
    fn intersect(&self, r: &Ray) -> Option<(SurfaceInteraction, Float)> {
        // TODO
        None
    }
    fn intersect_p(&self, r: &Ray) -> bool {
        // TODO
        false
    }
    fn get_reverse_orientation(&self) -> bool {
        // TODO
        false
    }
    fn get_transform_swaps_handedness(&self) -> bool {
        // TODO
        false
    }
    fn area(&self) -> Float {
        // TODO
        0.0 as Float
    }
    fn sample(&self, u: Point2f, pdf: &mut Float) -> InteractionCommon {
        // TODO
        InteractionCommon::default()
    }
    fn sample_with_ref_point(
        &self,
        iref: &InteractionCommon,
        u: Point2f,
        pdf: &mut Float,
    ) -> InteractionCommon {
        // TODO
        InteractionCommon::default()
    }
    fn pdf(&self, iref: &Interaction, wi: Vector3f) -> Float {
        // TODO
        0.0 as Float
    }
}

// Curve Utility Functions

fn blossom_bezier(p: &[Point3f; 4], u0: Float, u1: Float, u2: Float) -> Point3f {
    let a: [Point3f; 3] = [
        pnt3_lerp(u0, p[0], p[1]),
        pnt3_lerp(u0, p[1], p[2]),
        pnt3_lerp(u0, p[2], p[3]),
    ];
    let b: [Point3f; 2] = [pnt3_lerp(u1, a[0], a[1]), pnt3_lerp(u1, a[1], a[2])];
    pnt3_lerp(u2, b[0], b[1])
}