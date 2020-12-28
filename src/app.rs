use super::header::ListHeader;
use super::item::ListItem;
use super::list::List;
use super::{Hovered, WeakComponentLink};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::console;
use ybc::TileCtx::{Ancestor, Child, Parent};

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    console::log_1(&"From Rust: Hello using web-sys".into());
    alert(&format!("Hello, {}!", name));
}

pub enum Msg {
    Hover(Hovered),
}

pub struct App {
    link: ComponentLink<Self>,
    hovered: Hovered,
    list_link: WeakComponentLink<List>,
    sub_list_link: WeakComponentLink<List>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        console::log_1(&"From Rust: Component for App create()".into());
        Self {
            link,
            hovered: Hovered::None,
            list_link: WeakComponentLink::default(),
            sub_list_link: WeakComponentLink::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Hover(hovered) => {
                self.hovered = hovered;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_hover = &self.link.callback(Msg::Hover);
        let onmouseenter = &self.link.callback(|_| Msg::Hover(Hovered::None));
        let list_link = &self.list_link;
        let sub_list_link = &self.sub_list_link;

        // note the use of `html_nested!` instead of `html!`.
        let letters = ('A'..='C')
            .map(|letter| html_nested! { <ListItem name=letter.to_string() on_hover=on_hover /> });

        console::log_1(&"From Rust: Component for App view()".into());

        html! {
            <>
            <ybc::Navbar
                classes="is-success"
                padded=true
                navbrand=html!{
                    <ybc::NavbarItem>
                        <ybc::Title
                            classes="has-text-white"
                            size=ybc::HeaderSize::Is4
                        >{"Nirvana KG/UX"}</ybc::Title>
                    </ybc::NavbarItem>
                }
                navstart=html!{}
                navend=html!{
                    <>
                    <ybc::NavbarItem>
                        <ybc::ButtonAnchor
                            classes="is-black is-outlined"
                            rel="noopener noreferrer"
                            target="_blank"
                            href="https://github.com/agnos-ai/kg-ux-rust">
                            {"Github Repo"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    <ybc::NavbarItem>
                        <ybc::ButtonAnchor
                            classes="is-black is-outlined"
                            rel="noopener noreferrer"
                            target="_blank"
                            href="/login">
                            {"Login"}
                        </ybc::ButtonAnchor>
                    </ybc::NavbarItem>
                    </>
                }
            />
            <ybc::Hero
                classes="is-light"
                size=ybc::HeroSize::FullheightWithNavbar
                body=html!{
                    <div class="main" onmouseenter=onmouseenter>
                        <div id="firebaseui-auth-container"></div>
                        <div id="loader">{ "Loading..." }</div>
                        <h1>{ "Nested List Demo" }</h1>
                        // <MatButton label="Click me!" />
                        <List on_hover=on_hover weak_link=list_link>
                            <ListHeader text="Calling all Rusties!" on_hover=on_hover list_link=list_link />
                            <ListItem name="Rustin" on_hover=on_hover />
                            <ListItem hide=true name="Rustaroo" on_hover=on_hover />
                            <ListItem name="Rustifer" on_hover=on_hover>
                                <div class="sublist">{ "Sublist!" }</div>
                                <List on_hover=on_hover weak_link=sub_list_link>
                                    <ListHeader text="Sub Rusties!" on_hover=on_hover list_link=sub_list_link/>
                                    <ListItem hide=true name="Hidden Sub" on_hover=on_hover />
                                    { for letters }
                                </List>
                            </ListItem>
                        </List>
                        { self.view_last_hovered() }
                    </div>
                }>
            </ybc::Hero>
            </>
        }
    }
}

impl App {
    fn view_last_hovered(&self) -> Html {
        html! {
            <div class="last-hovered">
                { "Last hovered:"}
                <span class="last-hovered-text">
                    { &self.hovered }
                </span>
            </div>
        }
    }
}
