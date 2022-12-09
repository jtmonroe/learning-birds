use std::cell::RefCell;
use std::rc::Rc;

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
    dim: (f64, f64),
    #[prop_or_default]
    generation_id: String,
    #[prop_or_default]
    previous_fitness_id: String,
}

pub struct SimElement {
    node_ref: NodeRef,
    size: (f64, f64),
    simulation: Rc<RefCell<Simulation>>,
}


impl Component for SimElement {
    type Message = ();
    type Properties = SimCanvasInfo;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let info = ctx.props();
        let simulation =
            Simulation::new(info.generation_id.clone(), info.previous_fitness_id.clone());
        Self {
            node_ref: NodeRef::default(),
            size: info.dim,
            simulation: Rc::new(RefCell::new(simulation)),
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <canvas ref={self.node_ref.clone()} />
        }
    }

    fn rendered(&mut self, _ctx: &yew::Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        let canvas: HtmlCanvasElement = self.node_ref.cast().unwrap();
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
        let cb = Rc::new(RefCell::new(None));
        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb = cb.clone();
            let sim = self.simulation.clone();
            let dim = self.size.clone();
            move || {
                let mut borrowed_sim = sim.borrow_mut();
                borrowed_sim.step();
                let world = borrowed_sim.raw_world();

                Self::draw_frame(&world, &canvas_ctx, dim).expect("unreachable?");

                Self::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));
        Self::request_animation_frame(cb.borrow().as_ref().unwrap());
        Ok(())
    }

    fn draw_frame(world: &World, canvas_ctx: &CanvasContext, (height, width): (f64, f64)) -> Result<(), JsValue> {
        canvas_ctx.clear_rect(0.0, 0.0, width, height);

        for food in &world.food {
            draw_circle(
                canvas_ctx,
                food.x as f64 * width,
                food.y as f64 * height,
                (0.01 / 2.0) * width,
                "black".into(),
            )?;
        }

        Ok(())
    }
}

fn draw_circle(
    canvas_ctx: &CanvasContext,
    center_x: f64,
    center_y: f64,
    radius: f64,
    color: String,
) -> Result<(), JsValue> {
    canvas_ctx.begin_path();
    canvas_ctx.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI)?;
    canvas_ctx.set_fill_style(&color.into());
    canvas_ctx.fill();
    Ok(())
}


