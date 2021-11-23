use dyn_clone::DynClone;
use glam::Vec3;

use crate::hit_record::HitRecord;
use crate::ray::{Ray, ScatteredRay};
use crate::texture::{SolidColor, Texture};
use crate::tools::{onb_build_from_w, onb_local, random_cosine_direction};
use std::f32::consts::PI;

pub trait Material: DynClone + Send + Sync {
	fn scatter(&self, ray_direction: &Vec3, hit_record: &HitRecord) -> Option<ScatteredRay>;
	fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32;
    fn emitted(&self, hit_record: &HitRecord) -> Vec3;
}
dyn_clone::clone_trait_object!(Material);

// material types

#[derive(Clone)]
pub struct Lambertian {
    albedo: Box<dyn Texture>,
}
impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Self { albedo: albedo }
    }
    pub fn new_from_color(albedo: Vec3) -> Self {
        Self {
            albedo: Box::new(SolidColor::new(albedo)),
        }
    }
}
impl Material for Lambertian {
    fn scatter(&self, _ray_direction: &Vec3, hit_record: &HitRecord) -> Option<ScatteredRay> {
		let uvw = onb_build_from_w(&hit_record.normal());
		let scatter_direction = onb_local(uvw, random_cosine_direction());

        Some(ScatteredRay {
            ray: Ray::new(hit_record.point(), scatter_direction),
			albedo: self.albedo.color(hit_record.uv(), &hit_record.point()),
			pdf: scatter_direction.dot(uvw[2]) / PI,
        })
    }
	fn scattering_pdf(&self, _ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32 {
		let cosine = hit_record.normal().dot(scattered.unit_direction());
		if cosine < 0.0 {
			return 0.0
		} else {
			return cosine / PI
		}
	}
    fn emitted(&self, _hit_record: &HitRecord) -> Vec3 {
        Vec3::zero()
    }
}
/*
#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal { albedo: albedo, fuzz: fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, ray_direction: &Vec3, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let reflected: Vec3 = reflect(*ray_direction, hit_record.normal());

        if reflected.dot(hit_record.normal()) > 0.0 {
            Some(ScatteredRay {
                ray: Ray::new(hit_record.point(), reflected + self.fuzz * random_in_unit_sphere()),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
    fn emitted(&self, _uv: (f32, f32), _point: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

#[derive(Clone)]
pub struct Dielectric {
    refractive_index: f32,
}
impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric {
            refractive_index: refractive_index,
        }
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray_direction: &Vec3, hit_record: &HitRecord) -> Option<ScatteredRay> {
        let refractive_index = match hit_record.front_face() {
            true => 1.0 / self.refractive_index,
            false => self.refractive_index,
        };
        let cos_theta = hit_record.normal().dot(-*ray_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if refractive_index * sin_theta > 1.0 {
            // reflect
            let reflected = reflect(*ray_direction, hit_record.normal());
            return Some(ScatteredRay {
                ray: Ray::new(hit_record.point(), reflected),
                attenuation: Vec3::splat(1.0),
            });
        }

        let reflect_prob = schlick(cos_theta, refractive_index);
        if rand::random::<f32>() < reflect_prob {
            // reflect
            let reflected = reflect(*ray_direction, hit_record.normal());
            return Some(ScatteredRay {
                ray: Ray::new(hit_record.point(), reflected),
                attenuation: Vec3::splat(1.0),
            });
        }

        // refract
        let refracted = refract(*ray_direction, hit_record.normal(), refractive_index);
        Some(ScatteredRay {
            ray: Ray::new(hit_record.point(), refracted),
            attenuation: Vec3::splat(1.0),
        })
    }
    fn emitted(&self, _uv: (f32, f32), _point: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}
*/
#[derive(Clone)]
pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}
impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture>) -> Self {
        Self { emit: emit }
    }
    pub fn new_from_color(color: Vec3) -> Self {
        Self {
            emit: Box::new(SolidColor::new(color)),
        }
    }
}
impl Material for DiffuseLight {
    fn scatter(&self, _ray_direction: &Vec3, _hit_record: &HitRecord) -> Option<ScatteredRay> {
        None // no reflection/scattering
    }
	fn scattering_pdf(&self, _ray_in: &Ray, _hit_record: &HitRecord, _scattered: &Ray) -> f32 {
		0.0 // no scattering
	}
    fn emitted(&self, hit_record: &HitRecord) -> Vec3 {
		if hit_record.front_face() {
			return self.emit.color(hit_record.uv(), &hit_record.point())
		} else {
			return Vec3::zero()
		}
    }
}
