use cgmath::{Vector3, Point3, InnerSpace};

struct Camera {
    theta: f64,
    phi: f64,
    radius: f64,
    target: Point3<f64>
}

impl Camera {

    fn to_cartesian_coords(&self) -> Vector3<f64> {
        let x = self.radius * self.phi.sin() * self.theta.sin();
        let y = self.radius * self.phi.cos();
        let z = self.radius * self.phi.sin() * self.theta.cos();
        return Vector3{x, y, z};
    }

    fn rotate(&mut self, dtheta : f64, dPhi : f64) {
        self.theta += dtheta;
        self.phi += dPhi;
    }

    fn zoom(&mut self, distance : f64){
        self.radius -= distance;
    }

    fn pan(&mut self, dx : f64, dy: f64){
        let look = self.to_cartesian_coords().normalize();
        let world_up = Vector3{x: 0., y: 1., z: 0.};
        let right = look.cross(world_up);
        let up = look.cross(right);

        self.target += right * dx + up * dy;

    }
}