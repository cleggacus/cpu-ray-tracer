use std::{fs::File, io::BufReader};

use glium::glutin::event::VirtualKeyCode;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator};
use rfd::FileDialog;
use serde::{Serialize, Deserialize};

use crate::{vector::Vector3, event_manager::EventManager, quaternion::Quaternion};

pub type RGB = [f32; 3];

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Object {
  Sphere(Sphere),
  Plane(Plane),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Light {
  Ambient(AmbientLight),
  Directional(DirectionalLight),
  Point(PointLight),
}

impl ToString for Light {
  fn to_string(&self) -> String {
    match *self {
      Light::Ambient(_) => String::from("Ambient Light"),
      Light::Directional(_) => String::from("Directional Light"),
      Light::Point(_) => String::from("Point Light"),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AmbientLight {
  pub color: RGB,
}

impl AmbientLight {
  pub fn new() -> AmbientLight {
    AmbientLight { 
      color: [0.1, 0.05, 0.05]
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PointLight {
  pub color: RGB,
  pub position: Vector3,
}

impl PointLight {
  pub fn new() -> PointLight {
    PointLight {
      color: [1.0, 1.0, 1.0],
      position: Vector3::new(80.0, 60.0, -40.0),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DirectionalLight {
  pub color: RGB,
  pub direction: Vector3,
}

impl DirectionalLight {
  pub fn new() -> DirectionalLight {
    DirectionalLight {
      color: [1.0, 1.0, 1.0],
      direction: Vector3::new(0.5, 0.5, -10.0),
    }
  }
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Material {
  pub ambient_reflection: f64,
  pub has_specular: bool,
  pub specular_reflection: f64,
  pub diffuse_reflection: f64,
  pub reflectivity: f64,
  pub transparency: f64,
  pub refractive_index: f64,
  pub color: RGB,
}

impl Material {
  pub fn new() -> Material {
    Material {
      ambient_reflection: 1.0,
      has_specular: true,
      specular_reflection: 10.0,
      diffuse_reflection: 1.0,
      reflectivity: 0.0,
      transparency: 0.0,
      color: [1.0, 0.8, 0.5],
      refractive_index: 1.0,
    }
  }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Plane {
  pub position: Vector3,
  pub width: f64,
  pub height: f64,
  pub material: Material,
}

impl Plane {
  pub fn new() -> Plane {
    Plane {
      position: Vector3::new(0.0, -2.0, 10.0),
      width: 100.0,
      height: 100.0,
      material: Material::new()
    }
  }

  pub fn trace_ray(&self, ray: &Ray) -> Option<f64> {
    let origin = self.position - ray.position;
    let up = Vector3::new(0.0, 1.0, 0.0);

    let t = origin.dot(&up) / ray.direction.dot(&up);
    let position = ray.position_from_distance(t);

    if 
      position.z > self.position.z + self.height / 2.0 ||
      position.z < self.position.z - self.height / 2.0 ||
      position.x > self.position.x + self.width / 2.0 ||
      position.x < self.position.x - self.width / 2.0
    {
      return None
    }

    if t < 0.01 {
      return None;
    }

    Some(t)
  }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Sphere {
  pub position: Vector3,
  pub radius: f64,
  pub material: Material,
}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere {
      position: Vector3::new(0.0, 0.0, 10.0),
      radius: 2.0,
      material: Material::new()
    }
  }

  pub fn trace_ray(&self, ray: &Ray) -> [Option<f64>; 2] {
    let origin = ray.position - self.position;

    let a = ray.direction.dot(&ray.direction);
    let b = origin.dot(&ray.direction) * 2.0;
    let c = origin.dot(&origin) - self.radius.powi(2);

    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0_f64 {
      return [None, None];
    }

    let t1 = (-b + discriminant.sqrt()) / (2_f64 * a);
    let t2 = (-b - discriminant.sqrt()) / (2_f64 * a);

    if t1 < 0.01_f64 {
      return [None, None];
    }

    if t2 < 0.01_f64 {
      return [Some(t1), None];
    }

    [Some(t1), Some(t2)]
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct World {
  objects: Vec<Object>,
  lights: Vec<Light>,
  camera: Camera,
}

impl World {
  pub fn new() -> World {
    World { 
      objects: vec![
        Object::Sphere(Sphere::new()),
        Object::Plane(Plane::new())
      ],
      lights: vec![
        Light::Point(PointLight::new()),
        Light::Ambient(AmbientLight::new())
      ],
      camera: Camera::new(),
    }
  }

  pub fn update(&mut self, event_manager: &EventManager) {
    self.camera.update(event_manager);
  }

  pub fn lights_mut(&mut self) -> &mut Vec<Light> {
    &mut self.lights
  }

  pub fn lights(&self) -> &Vec<Light> {
    &self.lights
  }

  pub fn objects_mut(&mut self) -> &mut Vec<Object> {
    &mut self.objects
  }

  pub fn objects(&self) -> &Vec<Object> {
    &self.objects
  }

  pub fn camera(&self) -> &Camera {
    &self.camera
  }

  pub fn camera_mut(&mut self) -> &mut Camera {
    &mut self.camera
  }

  pub fn save_world(&mut self) {
    self.camera.rays.clear();

    let file = FileDialog::new()
      .add_filter("json", &["json"])
      .set_directory("/")
      .save_file();

    if file.is_none() {
      return;
    }

    let file = file.unwrap();

    serde_json::to_writer(&File::create(file.as_path()).unwrap(), self).unwrap();

    self.camera.calc_rays();
  }

  pub fn open_world(&mut self) {
    let file = FileDialog::new()
      .add_filter("json", &["json"])
      .set_directory("/")
      .pick_file();

    if file.is_none() {
      return;
    }

    let file = file.unwrap();

    let file = File::open(file.as_path());
    let reader = BufReader::new(file.unwrap());

    let world: Result<World, serde_json::Error> = serde_json::from_reader(reader);

    match world {
      Ok(data) => {
        self.camera = data.camera;
        self.lights = data.lights;
        self.objects = data.objects;
        self.camera.calc_rays();
      },
      Err(err) => {
        println!("{}", err);
      }
    }
  }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CameraType {
  Orthographic, Perspective
}

impl ToString for CameraType {
  fn to_string(&self) -> String {
    if *self == CameraType::Orthographic {
      String::from("Orthergraphic")
    } else {
      String::from("Perspective")
    }
  }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct CameraInfo {
  pub camera_height: f64,
  pub vertical_fov: f64,
  pub camera_type: CameraType,
  pub viewport_width: u32,
  pub viewport_height: u32,
  pub position: Vector3,
  pub miss_color: RGB,
  pub forward: Vector3,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Camera {
  camera_info_old: CameraInfo,
  camera_info: CameraInfo,
  rays: Vec<Ray>,
  speed: f64,
  depth: u32,
  moved: bool,
}

impl Camera {
  pub fn new() -> Camera {
    let camera_info = CameraInfo {
      camera_height: 1.0,
      vertical_fov: 1.0,
      camera_type: CameraType::Perspective,
      position: Vector3::new(0.0, 0.0, 0.0),
      miss_color: [0.6, 0.8, 0.9],
      forward: Vector3::new(0.0, 0.0, 1.0),
      viewport_width: 720,
      viewport_height: 480,
    };

    let mut cam = Camera {
      camera_info_old: camera_info,
      camera_info,
      depth: 5,
      rays: Vec::new(),
      speed: 0.1,
      moved: false,
    };

    cam.calc_rays();

    cam
  }

  pub fn speed_mut(&mut self) -> &mut f64 {
    &mut self.speed
  }

  pub fn depth(&self) -> &u32 {
    &self.depth
  }

  pub fn depth_mut(&mut self) -> &mut u32 {
    &mut self.depth
  }

  pub fn camera_info_mut(&mut self) -> &mut CameraInfo {
    &mut self.camera_info
  }

  pub fn camera_info(&self) -> &CameraInfo {
    &self.camera_info
  }

  pub fn update(&mut self, event_manager: &EventManager) {
    self.moved = false;

    if self.camera_info != self.camera_info_old {
      self.moved = true;
    }

    self.camera_info_old = self.camera_info;

    if event_manager.is_left_mouse_down() {
      let (yaw, pitch) = event_manager.mouse_move();

      if yaw != 0.0 || pitch != 0.0 {
        let pitch = pitch * 0.05 * self.speed;
        let yaw = yaw * 0.05 * self.speed;

        let forward = self.camera_info.forward;
        let right = self.right();
        let up = self.up();

        let q_pitch = Quaternion::from_angle_axis(pitch, right);
        let q_yaw = Quaternion::from_angle_axis(yaw, up);
        let p = Quaternion::from_vector_3(forward);

        let q = q_pitch * q_yaw;

        self.camera_info.forward = (q * p * q.inverse()).to_vector_3();

        self.moved = true;
      }
    }

    if event_manager.is_key_down(VirtualKeyCode::W) {
      self.camera_info.position = self.camera_info.position + self.camera_info.forward * self.speed;
      self.moved = true;
    } else if event_manager.is_key_down(VirtualKeyCode::S) {
      self.camera_info.position = self.camera_info.position - self.camera_info.forward * self.speed;
      self.moved = true;
    }

    if event_manager.is_key_down(VirtualKeyCode::D) {
      self.camera_info.position = self.camera_info.position + self.right() * self.speed;
      self.moved = true;
    } else if event_manager.is_key_down(VirtualKeyCode::A) {
      self.camera_info.position = self.camera_info.position - self.right() * self.speed;
      self.moved = true;
    }

    if event_manager.is_key_down(VirtualKeyCode::Q) {
      self.camera_info.position = self.camera_info.position + self.up() * self.speed;
      self.moved = true;
    } else if event_manager.is_key_down(VirtualKeyCode::E) {
      self.camera_info.position = self.camera_info.position - self.up() * self.speed;
      self.moved = true;
    }

    if self.moved {
      self.calc_rays()
    }
  }

  pub fn rays(&self) -> &Vec<Ray> {
    &self.rays
  }

  fn up(&self) -> Vector3 {
    self.camera_info.forward.cross(&self.right()).normalise()
  }

  fn right(&self) -> Vector3 {
    let plane_up = Vector3::new(0.0, -1.0, 0.0);
    self.camera_info.forward.cross(&plane_up).normalise()
  }

  fn horizontal_fov(&self) -> f64 {
    self.camera_info.viewport_width as f64 * self.camera_info.vertical_fov / self.camera_info.viewport_height as f64
  }

  fn camera_width(&self) -> f64 {
    self.camera_info.viewport_width as f64 * self.camera_info.camera_height / self.camera_info.viewport_height as f64
  }

  pub fn calc_rays(&mut self) {
    match self.camera_info.camera_type {
      CameraType::Orthographic => self.calc_orthographic_rays(),
      CameraType::Perspective => self.calc_perspective_rays(),
    }
  }

  fn calc_perspective_rays(&mut self) {
    let ray_count = self.camera_info.viewport_width as usize * self.camera_info.viewport_height as usize;

    if ray_count > self.rays.len() {
      self.rays.resize(ray_count, Ray::default());
    }

    let v_fov = self.camera_info.vertical_fov as f64;
    let h_fov = self.horizontal_fov() as f64;

    let right = self.right();
    let up = self.up();

    self.rays.par_iter_mut().enumerate().for_each(|(i, ray)| {
      let sample_width = self.camera_info.viewport_width;

      let y = i as u32 / sample_width;
      let x = i as u32 % sample_width;

      let x = x as f64 / self.camera_info.viewport_width as f64;
      let y = y as f64 / self.camera_info.viewport_height as f64;

      let view_y = v_fov * (2.0 * y - 1.0);
      let view_x = h_fov * (2.0 * x - 1.0);

      ray.direction = (right * view_x + up * view_y + self.camera_info.forward).normalise();
      ray.position = self.camera_info.position;
    });
  }

  fn calc_orthographic_rays(&mut self) {
    let ray_count = (self.camera_info.viewport_width * self.camera_info.viewport_height) as usize;

    if ray_count > self.rays.len() {
      self.rays.resize(ray_count, Ray::default());
    }

    let camera_height = self.camera_info.camera_height;
    let camera_width = self.camera_width();

    let right = self.right();
    let up = self.up();

    for y in 0..self.camera_info.viewport_height {
      let view_y = (y as f64 / self.camera_info.viewport_height as f64) * 2.0 - 1.0;

      for x in 0..self.camera_info.viewport_width {
        let i = (y * self.camera_info.viewport_width + x) as usize;

        let view_x = (x as f64 / self.camera_info.viewport_width as f64) * 2.0 - 1.0;

        self.rays[i].direction = self.camera_info.forward.normalise();
        self.rays[i].position = self.camera_info.position + (right * view_x * camera_width + up * view_y * camera_height);
      }
    }
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ray {
  pub position: Vector3,
  pub direction: Vector3,
}

impl Default for Ray {
  fn default() -> Self {
    Self { 
      position: Vector3::new(0.0, 0.0, 0.0),
      direction: Vector3::new(0.0, 0.0, 1.0)
    }
  }
}

impl Ray {
  pub fn position_from_distance(&self, distance: f64) -> Vector3 {
    self.position + self.direction * distance
  }
}

pub struct RayIntersection<'a> {
  ray: &'a Ray,
  material: &'a Material,
  distance: f64,
  normal: Vector3,
  n1: f64,
  n2: f64,
}

impl<'a> RayIntersection <'a>{
  pub fn new(ray: &'a Ray, material: &'a Material, distance: f64, normal: Vector3, n1: f64, n2: f64) -> RayIntersection<'a> {
    RayIntersection { 
      ray,
      material, 
      distance,
      normal,
      n1,
      n2,
    }
  }

  pub fn material(&self) -> &Material {
    &self.material
  }

  pub fn distance(&self) -> &f64 {
    &self.distance
  }

  pub fn normal(&self) -> &Vector3 {
    &self.normal
  }

  pub fn n1(&self) -> &f64 {
    &self.n1
  }

  pub fn n2(&self) -> &f64 {
    &self.n2
  }

  pub fn position(&self) -> Vector3 {
    self.ray.position_from_distance(self.distance)
  }
}
