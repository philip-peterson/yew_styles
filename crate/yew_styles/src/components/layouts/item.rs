use crate::utils::create_style;
use yew::prelude::*;

#[derive(Clone)]
pub enum ItemLayout {
    ItXs(i8),
    ItS(i8),
    ItM(i8),
    ItL(i8),
    ItXl(i8),
}

#[derive(Clone)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

pub enum Msg {
    Clicked,
}

pub struct Item {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone)]
struct ItemProps {
    layouts_classes: String,
    name: String,
    index: i16,
    class_name: String,
}

#[derive(Clone, Copy)]
struct ItemModel;

#[derive(Clone, Properties)]
pub struct Props {
    pub layouts: Vec<ItemLayout>,
    #[prop_or(AlignSelf::Auto)]
    pub align_self: AlignSelf,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or_default]
    pub index: i16,
    #[prop_or(Callback::noop())]
    pub onsignal: Callback<()>,
    pub children: Children,
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Item { link, props }
    }

    fn mounted(&mut self) -> ShouldRender {
        ItemModel.init(self.props.clone());

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onsignal.emit(());
            }
        };

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let item_props = ItemProps::from(self.props.clone());

        html! {
            <div
                class=if item_props.name == "" {
                    format!("item item-{} {} {}", item_props.index, item_props.layouts_classes, item_props.class_name)
                } else {
                    format!("item item-{}-{} {} {}", item_props.name, item_props.index, item_props.layouts_classes, item_props.class_name)
                }

                onclick=self.link.callback(|_| Msg::Clicked)
            >
                {self.props.children.render()}
            </div>
        }
    }
}

impl From<Props> for ItemProps {
    fn from(props: Props) -> Self {
        ItemProps {
            layouts_classes: ItemModel.get_layout_classes(props.layouts),
            name: props.name,
            index: props.index,
            class_name: props.class_name,
        }
    }
}

impl ItemModel {
    fn init(self, props: Props) {
        self.get_item_align(props.align_self, props.index, props.name);
    }

    fn get_layout_classes(self, layouts_prop: Vec<ItemLayout>) -> String {
        let mut layouts = layouts_prop
            .into_iter()
            .map(|layout| self.get_layout(layout))
            .collect::<String>();

        layouts.truncate(layouts.len() - 1);
        layouts
    }

    fn get_layout(self, item_layout: ItemLayout) -> String {
        match item_layout {
            ItemLayout::ItXs(size) => format!("it-xs-{} ", size),
            ItemLayout::ItS(size) => format!("it-s-{} ", size),
            ItemLayout::ItM(size) => format!("it-m-{} ", size),
            ItemLayout::ItL(size) => format!("it-l-{} ", size),
            ItemLayout::ItXl(size) => format!("it-xl-{} ", size),
        }
    }

    fn get_item_align(self, align: AlignSelf, index: i16, name: String) {
        let value = match align {
            AlignSelf::Auto => "auto".to_string(),
            AlignSelf::Baseline => "baseline".to_string(),
            AlignSelf::Center => "center".to_string(),
            AlignSelf::FlexStart => "flex-start".to_string(),
            AlignSelf::FlexEnd => "flex-end".to_string(),
            AlignSelf::Stretch => "stretch".to_string(),
        };

        create_style(
            String::from("align-self"),
            value,
            if name == "" {
                format!("item-{}", index)
            } else {
                format!("item-{}-{}", name, index)
            },
        );
    }
}