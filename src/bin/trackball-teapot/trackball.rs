use cgmath::{Vector3, Point3, InnerSpace};

pub struct Camera {
    pub theta: f32,
    pub phi: f32,
    pub radius: f32,
    pub target: Point3<f32>
}

impl Camera {

    pub fn to_cartesian_coords(&self) -> Point3<f32> {
        let x = self.radius * self.phi.sin() * self.theta.sin();
        let y = self.radius * self.phi.cos();
        let z = self.radius * self.phi.sin() * self.theta.cos();
        Point3{x: self.radius * self.phi.sin() * self.theta.sin(), y: self.radius * self.phi.cos(), z: self.radius * self.phi.sin() * self.theta.cos()}
    }

    pub(crate) fn rotate(&mut self, dtheta : f32, dPhi : f32) {
        self.theta += dtheta;
        self.phi += dPhi;
    }

    pub fn zoom(&mut self, distance : f32){
        self.radius -= distance;
    }

    pub fn get_up_vector(&self) -> Vector3<f32> {
        let look = (self.to_cartesian_coords() - Point3{x: 0.,y: 0.,z: 0.}).normalize();
        let world_up = Vector3{x: 0., y: 1., z: 0.};
        let right = look.cross(world_up);
        let up = look.cross(right);
        return up;
    }

    fn pan(&mut self, dx : f32, dy: f32){
        let look = (self.to_cartesian_coords() - Point3{x: 0.,y: 0.,z: 0.}).normalize();
        let world_up = Vector3{x: 0., y: 1., z: 0.};
        let right = look.cross(world_up);
        let up = look.cross(right);

        self.target += right * dx + up * dy;

    }
}