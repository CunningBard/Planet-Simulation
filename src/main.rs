use std::i128;
use evalexpr::*;
use libm::{atan2, cos, sin, sqrt};
use macroquad::prelude::*;
use macroquad::time;
use crate::miniquad::conf::Icon;

fn substring(str: String, start: i32, end: i32) ->  Option<String>
{
    if end <= start
    {
        return None;
    }
    let ss = (&str[(start as usize)..(end as usize)]).to_string();
    Option::from(ss)

}

const AU: i128 = 149600000000;
const G: f64 = 0.000000000066744;
const SCALE: f64 = 0.00000000167112299;  // 1AU = 100 pixels
const TIMESTEP: i128 = 3600 * 24; // 1 day
const WIDTH: i32 = 700;
const HEIGHT: i32 = 700;

#[derive(Clone)]
struct Planet
{
    x: i128,
    y: i128,
    id: u16,
    mass: i128,
    radius: i32,
    color: Vec<u8>,
    x_velocity: i128,
    y_velocity: i128,
    orbit: Vec<Vec<f32>>
}
// def attraction(self, other):
// 		other_x, other_y = other.x, other.y
// 		distance_x = other_x - self.x
// 		distance_y = other_y - self.y
// 		distance = math.sqrt(distance_x ** 2 + distance_y ** 2)
//
// 		if other.sun:
// 			self.distance_to_sun = distance
//
// 		force = self.G * self.mass * other.mass / distance**2
// 		theta = math.atan2(distance_y, distance_x)
// 		force_x = math.cos(theta) * force
// 		force_y = math.sin(theta) * force
// 		return force_x, force_y

impl Planet
{
    pub fn planet(x: i128, y: i128, mass: i128, radius: i32, color: Vec<u8>, id: u16) -> Planet{
        Planet {
            x,
            y,
            id,
            mass,
            radius,
            color,
            x_velocity: 0,
            y_velocity: 0,
            orbit: vec![],
        }
    }
    pub fn attraction(&self, other: &Planet) -> Vec<i128> {
        let distance_x = other.x - self.x;
        let distance_y = other.y - self.y;
        let distance = sqrt((distance_x.pow(2) + distance_y.pow(2)) as f64) as i64;

         let force = (G * self.mass as f64 * other.mass as f64 / (distance as i128).pow(2) as f64 )as f64; // KILL ME
        // let res = eval(&format!("{} * {}", G, self.mass)).unwrap().to_string();
        // let force = res.parse::<f64>().unwrap() / (distance as i128).pow(2) as f64;
        let theta = atan2(distance_y as f64, distance_x as f64);
        let force_x = cos(theta) * force as f64;
        let force_y = sin(theta) * force as f64;

        vec![force_x as i128, force_y as i128]
    }
        // def draw(self, win):
        // x = self.x * self.SCALE + WIDTH / 2
        // y = self.y * self.SCALE + HEIGHT / 2
        //
        // if len(self.orbit) > 2:
        //     updated_points = []
        //     for point in self.orbit:
        //         x, y = point
        //         x = x * self.SCALE + WIDTH / 2
        //         y = y * self.SCALE + HEIGHT / 2
        //         updated_points.append((x, y))
        //
        //     pygame.draw.lines(win, self.color, False, updated_points, 2)
        //
        // pygame.draw.circle(win, self.color, (x, y), self.radius)
    pub fn draw(&mut self) -> (f32, f32, f32, Color){
            let x = (eval(&format!(" 0.00000000167112299 * {}", self.x)).unwrap().to_string().parse::<f64>().unwrap() as i128 + WIDTH  as i128 / 2) as f32;
            let y = (eval(&format!(" 0.00000000167112299 * {}", self.y)).unwrap().to_string().parse::<f64>().unwrap() as i128 + WIDTH  as i128 / 2) as f32;
            let r = self.radius as f32;
            let c = Color::from_rgba(
                            self.color[0],
                            self.color[1],
                            self.color[2], 255);

            self.orbit.push(vec![x, y]);
            return (x,y, r,c);
        }

    pub fn update_position(&mut self, planets: &Vec<Planet>) {
        let mut total_fx = 0;
        let mut total_fy = 0;
        for planet in planets {
            if planet.id != self.id {
                let res = self.attraction(&planet);
                total_fx += res[0];
                total_fy += res[1];
            }
        }
        self.x_velocity += eval(&format!("{}.0 / {} * {} * {}", total_fx, self.mass, TIMESTEP, TIMESTEP)).unwrap().to_string().parse::<f64>().unwrap() as i128;
        self.y_velocity += eval(&format!("{}.0 / {} * {} * {}", total_fy, self.mass, TIMESTEP, TIMESTEP)).unwrap().to_string().parse::<f64>().unwrap() as i128;
        self.x += self.x_velocity;
        self.y += self.y_velocity;
        }

}
fn window_conf() -> Conf {
    Conf {
            window_title: "".to_owned(),
            window_width: WIDTH,
            window_height: HEIGHT,
            high_dpi: false,
            fullscreen: false,
            sample_count: 1,
            window_resizable: true,
            icon: Some(Icon::miniquad_logo()),
        }
}

#[macroquad::main(window_conf())]
async fn main() {
    let now = time::get_fps();
    let mut planets = vec![];
    // sun = Planet(0, 0, 30, YELLOW, 1988920000000000000000000000000)
    let mut sun = Planet::planet(0, 0, 198892 * 10_i128.pow(24), 30, vec![255, 255, 0], 1);
    planets.push(sun);
    let mut earth = Planet::planet(-1 * AU, 0, 59722 * 10_i128.pow(18), 16, vec![90, 25, 55], 2);
    earth.y_velocity = 807830000;
    planets.push(earth);

    loop {
        let i_hate_my_self = planets.clone();
        // let now = std::time::SystemTime::now();
        for planet in &mut planets {
            planet.update_position(&i_hate_my_self);
        }
        // println!("{:?}", now.elapsed());
        for planet in &mut planets {
            for dot in &planet.orbit {
                draw_circle(dot[0], dot[1], 1f32, Color::from_rgba(255, 255, 255, 255))
            }
            let res = planet.draw();
            draw_circle(res.0, res.1 ,res.2 ,res.3);
        }
        next_frame().await;
    }
}
