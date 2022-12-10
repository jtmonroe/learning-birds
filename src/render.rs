use std::cell::RefCell;
use std::rc::Rc;

use log::info;
// use simulation_wasm::timer;
use simulation_wasm::Simulation;
use simulation_wasm::World;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::{CanvasRenderingContext2d as CanvasContext, HtmlCanvasElement};
use yew::Properties;
use yew::{html, Component, NodeRef};

#[derive(Properties, PartialEq, Clone)]
pub struct SimCanvasInfo {
    #[prop_or_default]
    pub dim: (f64, f64),
    #[prop_or_default]
    pub bird_colour: String,
    #[prop_or_default]
    pub food_colour: String,
}

pub struct SimElement {
    node_ref: NodeRef,
    size: (f64, f64),
    simulation: Rc<RefCell<Simulation>>,
    bird_colour: Colour,
    food_colour: Colour,
}

#[derive(PartialEq, Clone)]
pub struct Colour(String);

impl Component for SimElement {
    type Message = ();
    type Properties = SimCanvasInfo;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let info = ctx.props();
        let simulation = Simulation::new(40, 60);
        Self {
            node_ref: NodeRef::default(),
            size: info.dim,
            simulation: Rc::new(RefCell::new(simulation)),
            bird_colour: Colour(info.bird_colour.clone()),
            food_colour: Colour(info.food_colour.clone()),
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <canvas height=500 width=500 ref={self.node_ref.clone()} />
        }
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, first_render: bool) {
        info!("rendered");
        if !first_render {
            return;
        }
        let scale = window().unwrap().device_pixel_ratio();
        let canvas: HtmlCanvasElement = self.node_ref.cast().unwrap();

        let new_height = (canvas.client_height() as f64) * scale;
        if new_height > 0.0 {
            canvas.set_height(new_height as u32);
        }

        let new_width = (canvas.client_width() as f64) * scale;
        if new_width > 0.0 {
            canvas.set_width(new_width as u32);
        }

        self.size = (new_height, new_width);

        let canvas_ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.render(canvas_ctx).expect("unreachable?");
    }
}

impl SimElement {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn render(&mut self, canvas_ctx: CanvasContext) -> Result<(), JsValue> {
        Self::draw_frame(
            &self.simulation.borrow().raw_world(),
            &canvas_ctx,
            self.size,
            &self.bird_colour,
            &self.food_colour,
        )?;

        let cb = Rc::new(RefCell::new(None));
        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let b_c = self.bird_colour.clone();
            let f_c = self.food_colour.clone();
            let cb = cb.clone();
            let sim = self.simulation.clone();
            let dim = self.size.clone();
            move || {


                let mut borrowed_sim = sim.borrow_mut();
                borrowed_sim.step();
                let world = borrowed_sim.raw_world();

                Self::draw_frame(&world, &canvas_ctx, dim, &b_c, &f_c)
                    .expect("unreachable?");

                Self::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));
        Self::request_animation_frame(cb.borrow().as_ref().unwrap());
        Ok(())
    }

    fn draw_frame(
        world: &World,
        canvas_ctx: &CanvasContext,
        (height, width): (f64, f64),
        bird_colour: &Colour,
        food_colour: &Colour,
    ) -> Result<(), JsValue> {
        canvas_ctx.clear_rect(0.0, 0.0, width, height);

        for food in &world.food {
            draw_circle(
                canvas_ctx,
                food.x as f64 * width,
                food.y as f64 * height,
                (0.01 / 2.0) * width,
                food_colour,
            )?;
        }

        for animal in &world.animals {
            draw_triangle(
                canvas_ctx,
                animal.x as f64 * width,
                animal.y as f64 * height,
                0.01 * width,
                animal.rotation as f64,
                bird_colour,
            )?;
        }

        Ok(())
    }
}

const TWO_THIRDS_PI: f64 = 2.0 * std::f64::consts::FRAC_PI_3;
const FOUR_THIRDS_PI: f64 = 4.0 * std::f64::consts::FRAC_PI_3;
const FLOAT_PI: f64 = std::f64::consts::PI;

fn draw_circle(
    canvas_ctx: &CanvasContext,
    center_x: f64,
    center_y: f64,
    radius: f64,
    colour: &Colour,
) -> Result<(), JsValue> {
    canvas_ctx.begin_path();
    canvas_ctx.arc(center_x, center_y, radius, 0.0, 2.0 * FLOAT_PI)?;
    canvas_ctx.set_fill_style(&colour.0.clone().into());
    canvas_ctx.fill();
    Ok(())
}

fn draw_triangle(
    canvas_ctx: &CanvasContext,
    tip_x: f64,
    tip_y: f64,
    size: f64,
    rotation: f64,
    colour: &Colour,
) -> Result<(), JsValue> {
    canvas_ctx.begin_path();
    canvas_ctx.move_to(
        tip_x + f64::cos(rotation) * size * 1.5,
        tip_y + f64::sin(rotation) * size * 1.5,
    );

    canvas_ctx.line_to(
        tip_x + f64::cos(rotation + TWO_THIRDS_PI) * size,
        tip_y + f64::sin(rotation + TWO_THIRDS_PI) * size,
    );

    canvas_ctx.line_to(
        tip_x + f64::cos(rotation + FOUR_THIRDS_PI) * size,
        tip_y + f64::sin(rotation + FOUR_THIRDS_PI) * size,
    );

    canvas_ctx.line_to(
        tip_x + f64::cos(rotation) * size * 1.5,
        tip_y + f64::sin(rotation) * size * 1.5,
    );

    canvas_ctx.set_fill_style(&colour.0.clone().into());
    canvas_ctx.fill();
    Ok(())
}
