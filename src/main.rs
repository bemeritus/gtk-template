use adw::prelude::*;
use relm4::prelude::*;

#[derive(Default)]
struct App;

#[derive(Debug)]
enum Msg {}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();

    view! {
        main_window = gtk::ApplicationWindow {
            set_title: Some("Action example"),
            set_default_size: (600, 600),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 5,
                set_spacing: 5,

                adw::PreferencesPage {
                    add = &adw::PreferencesGroup {
                        set_title: "E-IMZO Keys",

                        #[name(expander)]
                        add = &adw::ExpanderRow {
                            set_title: "NiggerLicious",
                            set_subtitle: "niggerlicious@gmail.com",
                        }
                    }
                }
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App;
        let widgets = view_output!();

        widgets.expander.add_row(
            &adw::ActionRow::builder()
                .title("Description")
                .subtitle(
                    "Lorem Ipsum is simply dummy text of the printing and typesetting industry.",
                )
                .build(),
        );

        widgets.expander.add_row(
            &adw::ActionRow::builder()
                .title("Age")
                .subtitle("33")
                .build(),
        );

        let actions_row = adw::ActionRow::builder().build();
        let remove_btn = gtk::Button::with_label("Remove…");
        remove_btn.add_css_class("destructive-action");
        actions_row.add_suffix(&remove_btn);
        widgets.expander.add_row(&actions_row);

        ComponentParts { model, widgets }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.actions");
    app.run::<App>(());
}
