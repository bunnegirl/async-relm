use crate::list::*;
use crate::*;
use gtk::prelude::GtkWindowExt;
use relm4::{AppUpdate, Components, Model, RelmComponent, Sender, Widgets};
use tokio::sync::mpsc;

pub struct AppModel {
    pub async_handler: mpsc::Sender<(AsyncHandlerMsg, Sender<AppMsg>)>,
}

#[derive(Debug)]
pub enum AppMsg {
    AddItem(usize),
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        use AppMsg::*;

        match msg {
            AddItem(item) => components.list.send(ListMsg::AddItem(item)).unwrap(),
        }

        true
    }
}

pub struct AppComponents {
    list: RelmComponent<ListModel, AppModel>,
}

impl Components<AppModel> for AppComponents {
    fn init_components(
        parent_model: &AppModel,
        parent_widgets: &AppWidgets,
        parent_sender: Sender<AppMsg>,
    ) -> Self {
        AppComponents {
            list: RelmComponent::new(parent_model, parent_widgets, parent_sender.clone()),
        }
    }
}

pub struct AppWidgets {
    window: gtk::ApplicationWindow,
}

impl Widgets<AppModel, ()> for AppWidgets {
    type Root = gtk::ApplicationWindow;

    fn init_view(_model: &AppModel, _parent_widgets: &(), _sender: Sender<AppMsg>) -> Self {
        let window = gtk::ApplicationWindow::builder()
            .title("async relm")
            .default_width(320)
            .default_height(240)
            .build();

        AppWidgets { window }
    }

    fn connect_components(&self, components: &AppComponents) {
        self.window.set_child(Some(components.list.root_widget()));
    }

    fn view(&mut self, _model: &AppModel, _sender: Sender<AppMsg>) {}

    fn root_widget(&self) -> gtk::ApplicationWindow {
        self.window.clone()
    }
}
