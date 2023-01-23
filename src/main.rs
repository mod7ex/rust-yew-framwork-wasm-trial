use yew::{
    prelude::{ Context, Component, Html, html },
    Renderer
};

enum Msg {
    Increment,
}

struct Counter {
    count: i64,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            count: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.count += 1;
                true // re-render component
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>{ self.count }</p>
                <button onclick={link.callback(|_| Msg::Increment)}>{ "Increment" }</button>
            </div>
        }
    }
}

fn main() {
    Renderer::<Counter>::new().render();
}