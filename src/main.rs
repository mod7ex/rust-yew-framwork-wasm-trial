use yew::{ 
    prelude::Component, 
    Context, 
    Renderer,
    html, 
};

enum Event {
    Add,
    Update(String),
    Remove(usize),
    RemoveAll
}

struct Model {
    input: String,
    todos: Vec<String>
}

impl Component for Model {
    type Message = Event;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            input: String::new(),
            todos: vec![]
        }
    }

    fn update(&mut self, _: &Context<Self>, ev: Self::Message) -> bool {
        match ev {
            Event::Add => { true },
            Event::Update(v) => { true },
            Event::Remove(i) => { true },
            Event::RemoveAll => { true }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let link = ctx.link();

        html! {
            <div class="container">
                <input placeholder="write something" />
                <p>{ self.count }</p>
                <button onclick={ link.callback(|_| Msg::Increment) }>{ "increment" }</button>
            </div>
        }
    }
}

fn main() {
    Renderer::<Model>::new().render();
}