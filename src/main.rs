use yew::prelude::*;
use wasm_bindgen::{JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use gloo_timers::callback::{Timeout};
use wasm_logger;
use log;

mod boids;

const TIMEOUT: u32 = 1000 / 60;

enum Msg {
    Init,
    Update,
    Run,
    Stop
}

struct Model {
    canvas: NodeRef,
    sim: boids::Simulation,
    running: bool
}

fn step(canvas: &NodeRef, sim: &mut boids::Simulation, update: bool) {
    let _canvas = canvas.cast::<HtmlCanvasElement>().unwrap();
    let ctx = _canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    if update {
        sim.update();
    }
    sim.render(&ctx, 5.0);
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let sim = boids::Simulation::new();
        let canvas = NodeRef::default();

        Self {
            canvas,
            sim,
            running: false
        }
    }


    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Init =>{
                log::info!("Init");
                self.sim = boids::Simulation::initialize(10);

                step(&self.canvas, &mut self.sim, false);

                true
            },
            Msg::Run => {
                log::info!("Run");
                // let interval = Interval::new(1000 / 60, move || {
                //     step(&self.canvas, &mut self.sim, true);
                // });
                self.running = true;

                step(&self.canvas, &mut self.sim, true);

                let link = _ctx.link().clone();
                let timeout = Timeout::new(TIMEOUT, move || {
                    link.send_message(Msg::Update);
                });

                timeout.forget();

                true
            },
            Msg::Update =>{
                log::info!("Update");
                step(&self.canvas, &mut self.sim, true);

                if self.running {
                    let link = _ctx.link().clone();
                    let timeout = Timeout::new(TIMEOUT, move || {
                        link.send_message(Msg::Update);
                    });

                    timeout.forget();
                }

                true
            },
            Msg::Stop => {
                self.running = false;
                false
            }

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        // let text = format!("{:?}", self.sim);
        html! {
            <div>
            <div>
                <button onclick={link.callback(|_| Msg::Init)}>{ "Init" }</button>
                <button onclick={link.callback(|_| Msg::Update)}>{ "Step" }</button>
                <button onclick={link.callback(|_| Msg::Run)}>{ "Run" }</button>
                <button onclick={link.callback(|_| Msg::Stop)}>{ "Stop" }</button>
            </div>
            <div>

                // <p>{text}</p>
                <canvas ref={self.canvas.clone()} width="500" height="500" style="outline: black 3px solid;"></canvas>
            </div>
            </div>

        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}