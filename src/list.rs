use crate::*;
use gtk::prelude::{BoxExt, ButtonExt};
use gtk::Orientation;
use relm4::{ComponentUpdate, Model, Sender, WidgetPlus, Widgets};
use tokio::sync::mpsc;

pub struct ListModel {
    pub async_handler: mpsc::Sender<(AsyncHandlerMsg, Sender<AppMsg>)>,
    pub list: Vec<usize>,
}

#[derive(Debug)]
pub enum ListMsg {
    AddItem(usize),
    GetItems,
}

impl Model for ListModel {
    type Msg = ListMsg;
    type Widgets = ListWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for ListModel {
    fn init_model(parent_model: &AppModel) -> Self {
        ListModel {
            async_handler: parent_model.async_handler.clone(),
            list: Vec::new(),
        }
    }

    fn update(
        &mut self,
        msg: ListMsg,
        _components: &(),
        _sender: Sender<ListMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        use ListMsg::*;

        match msg {
            GetItems => {
                self.list = Vec::new();

                self.async_handler
                    .blocking_send((AsyncHandlerMsg::GetItems, parent_sender.clone()))
                    .unwrap();
            }
            AddItem(item) => {
                println!("{}", item);
                self.list.push(item);
            }
        }
    }
}

pub struct ListWidgets {
    root: gtk::Box,
    list: gtk::ListBox,
}

impl Widgets<ListModel, AppModel> for ListWidgets {
    type Root = gtk::Box;

    fn init_view(
        _model: &ListModel,
        _parent_widgets: &AppWidgets,
        sender: Sender<ListMsg>,
    ) -> Self {
        let root = gtk::Box::new(Orientation::Vertical, 10);

        root.set_margin_all(10);

        let scroll = gtk::ScrolledWindow::builder()
            .valign(gtk::Align::Fill)
            .vexpand(true)
            .build();

        let list = gtk::ListBox::new();

        scroll.set_child(Some(&list));
        root.append(&scroll);

        let populate_button = gtk::Button::builder().label("Populate").build();
        let populate_sender = sender.clone();

        populate_button.connect_clicked(move |_| {
            populate_sender.send(ListMsg::GetItems).unwrap();
        });

        root.append(&populate_button);

        ListWidgets { root, list }
    }

    fn view(&mut self, model: &ListModel, _sender: Sender<ListMsg>) {
        for item in &model.list {
            let list_item = gtk::Label::new(Some(&format!("Item {}", item)));

            self.list.append(&list_item);
        }
    }

    fn root_widget(&self) -> Self::Root {
        self.root.clone()
    }
}
