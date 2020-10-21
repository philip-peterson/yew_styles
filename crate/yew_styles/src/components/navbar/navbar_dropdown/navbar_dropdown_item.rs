use yew::prelude::*;

pub struct NavbarDropdownItem {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or(Callback::noop())]
    /// Click event for dropdown item
    pub onclick_signal: Callback<MouseEvent>,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// show with style when the dropdown item is currrently active
    #[prop_or(false)]
    pub active: bool,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
    pub children: Children,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for NavbarDropdownItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked(mouse_event) => {
                self.props.onclick_signal.emit(mouse_event);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }

        false
    }

    fn view(&self) -> Html {
        html! {
            <li
                class=("navbar-dropdown-item",if self.props.active {
                    "active"
                } else {
                    ""
                }, self.props.class_name.clone())
                id=self.props.id
                onclick=self.link.callback(Msg::Clicked)
            >{self.props.children.clone()}</li>
        }
    }
}
