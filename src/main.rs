use yew::{ 
    prelude::Component, 
    Context, 
    Renderer,
    html, 
    InputEvent,
    Html,
    KeyboardEvent
};

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Nothing
}

struct Model {
    pub input: String,
    pub todos: Vec<String>
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            input: String::new(),
            todos: vec![]
        }
    }

    fn update(&mut self, _: &Context<Self>, ev: Self::Message) -> bool {
        match ev {
            Msg::Add => {
                let s = self.input.clone();
                self.todos.push(s);
                self.input = String::new();
                true
            },
            Msg::Update(v) => {
                self.input = v;
                true
            },
            Msg::Remove(i) => {
                self.todos.remove(i);
                true
            },
            Msg::RemoveAll => {
                self.todos.clear();
                true
            },
            Msg::Nothing => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
 
        let oninput_cb = link.callback(|e: InputEvent| {
            let target = e.target().unwrap();

            let input = target.value_of().as_string().unwrap();

            println!("{}", input);

            Msg::Update(input.clone())
        });
 
        let onkeypress_cb = link.callback(|e: KeyboardEvent| {
            let key = e.key();

            if key.as_str() == "Enter" {
                Msg::Add
            } else {
                Msg::Nothing
            }
            
        });

        html! {
            <div class="container">
                <input 
                    placeholder="write something"
                    type="text"
                    value={self.input.clone()}
                    oninput={oninput_cb}
                    onkeypress={onkeypress_cb}
                />

                <p>{ &self.input }</p>

                <ul>
                    {
                        "todos list"
                    }
                </ul>

                <button onclick={link.callback(|_| { Msg::RemoveAll })} >{ "Remove all" }</button>
            </div>
        }
    }
}

fn main() {
    Renderer::<Model>::new().render();
}