use yew::prelude::{NodeRef, Component, Context, Html, html};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Event, HtmlFormElement, FocusEvent, HtmlInputElement};
use gloo_timers::callback::{Timeout};
use log;
use stdweb::web::{FormData, Element, EventTarget, HtmlElement};
use stdweb::web::event::{InputEvent, IEvent};

use crate::boids;

const TIMEOUT: u32 = 1000 / 60;


pub enum Msg {
    Init,
    Update(bool),
    Run,
    Stop,
    Submit(HtmlFormElement),
}

pub struct Model {
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

                if self.running {
                    return false;
                }
                self.running = true;

                step(&self.canvas, &mut self.sim, true);

                let link = _ctx.link().clone();
                let timeout = Timeout::new(TIMEOUT, move || {
                    link.send_message(Msg::Update(false));
                });

                timeout.forget();

                true
            },
            Msg::Update(manual) =>{
                log::info!("Update");

                step(&self.canvas, &mut self.sim, true);

                if self.running && !manual {
                    let link = _ctx.link().clone();
                    let timeout = Timeout::new(TIMEOUT, move || {
                        link.send_message(Msg::Update(false));
                    });

                    timeout.forget();
                }

                true
            },
            Msg::Stop => {
                self.running = false;
                false
            },
            Msg::Submit(data) => {
                log::info!("Submit");
                log::info!("{:?}", data);

                let num_children = data.length();

                for i in 0..num_children-1 {
                    let child = data.children().item(i as u32).unwrap();
                    let child = child.dyn_into::<HtmlInputElement>().unwrap();
                    let name = child.name();
                    let value = child.value();

                    log::info!("{}: {}", name, value.parse::<f32>().unwrap());
                }
                log::info!("num_children: {}", num_children);


                // log::info!("{:?}", data.children().item(0).unwrap().dyn_ref::<HtmlInputElement>().unwrap().value());

                // let num = data.get("num").unwrap().parse::<i32>().unwrap();
                // self.sim = boids::Simulation::initialize(num);
                // step(&self.canvas, &mut self.sim, false);
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
                <button onclick={link.callback(|_| Msg::Update(true))}>{ "Step" }</button>
                <button onclick={link.callback(|_| Msg::Run)}>{ "Run" }</button>
                <button onclick={link.callback(|_| Msg::Stop)}>{ "Stop" }</button>
            </div>

            <div>

                // <p>{text}</p>
                <canvas ref={self.canvas.clone()} width="500" height="500" style="outline: black 3px solid;"></canvas>
            </div>

            <div>
            <form onsubmit={link.callback(|e: FocusEvent| {
                e.prevent_default();
                let form_element = e.target().unwrap().dyn_ref::<HtmlFormElement>().unwrap().clone();

                Msg::Submit(form_element)
                }
            )}>

            <input type="text" name="num" placeholder="Number of boids" value="100" />
            <input type="text" name="turn" placeholder="Turn factor" value="10" />
            <input type="submit" value="Submit" />

            </form>
            </div>

            </div>

        }
    }
}