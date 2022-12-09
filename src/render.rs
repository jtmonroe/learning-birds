use std::cell::RefCell;
use std::rc::Rc;

use simulation_wasm::Simulation;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL};
use yew::{html, Component, NodeRef};

pub struct SimulationElement {
    node_ref: NodeRef,
    simulation: Rc<RefCell<Simulation>>,
}

fn into_refc<T>(value: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(value))
}

impl Component for SimulationElement {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let simulation = Simulation::new("generation_id".into(), "fitness_id".into()); 
        Self {
            node_ref: NodeRef::default(),
            simulation: into_refc(simulation),
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
        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        self.render(gl);
    }
}

impl SimulationElement {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn render(&mut self, gl: GL) {
        let cb = Rc::new(RefCell::new(None)); 
        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb = cb.clone();
            let sim = self.simulation.clone();
            move || {
                let mut borrowed_sim = sim.borrow_mut(); 
                borrowed_sim.step();
                let world = borrowed_sim.raw_world();
                let animals = world.animals;
                let food = world.food;
                // TODO: App Logic
                Self::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        })as Box<dyn FnMut()> ));
        Self::request_animation_frame(cb.borrow().as_ref().unwrap());
    }
}
