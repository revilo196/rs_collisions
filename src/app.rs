
use graphics::types::Vec2d;
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};


#[derive(Debug)]
pub struct ColBox {
pub bottom : f64,
pub top: f64, 
pub left : f64,
pub right : f64
}

#[derive(Debug)]
pub struct Particle {
    pub position : Vec2d,
    pub velocity : Vec2d,
    pub acceration : Vec2d,
    pub radius : f64,
}

 impl Particle {
    pub fn update(&mut self,dt:f64, colbox: &ColBox) {
          self.position[0] += self.velocity[0] * dt;
          self.velocity[0] += self.acceration[0] * dt;
          self.position[1] += self.velocity[1] * dt;
          self.velocity[1] += self.acceration[1] * dt;
    

         
          if self.position[0]+self.radius >= colbox.right {  //reflect right
             self.velocity[0] = -self.velocity[0];
          }
          if self.position[0]-self.radius <= colbox.left { // reflect left
             self.velocity[0] = -self.velocity[0];
          }
          if self.position[1]+self.radius >= colbox.top { // reflect top 
             self.velocity[1] = -self.velocity[1]
          }
          if self.position[1]-self.radius <= colbox.bottom { // reflect bottom 
             self.velocity[1] = -self.velocity[1]
          }



   }
}

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub rotation: f64,
    pub test: i32,
}


///
///TODO: setup with init
///TODO: impl particle
/// - frame 
/// - particle motion with collison with frame 
/// - 
///
impl App {
    pub fn update_app(&mut self, args: &UpdateArgs) {
        self.test += 1;
        self.rotation += 2.0 * args.dt;
    }

    pub fn render_app(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }
}
