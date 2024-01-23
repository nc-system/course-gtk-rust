
// use gtk4 as gtk;
use adw::prelude::*;
use adw::{ ActionRow, Application, ApplcationWindow, HeaderBar };
use gtk::{ Box, ListBox, Orientation, SelectionMode };


fn main() {
    
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_activate(|app| {
        
        // ActionRows are only available in Adawaita
        let row = ActionRow::builder()
            .activatable(true)
            .title("Cleck me")
            .build();

        row.connect_activate(|_| {
            eprintln!("Clicked !");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            // makes the list look nicer
            .css_classes(vec![String::from("boxed-list")])
            .build();
        list.append(&row);


        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(&HeaderBar::new());
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Theme Adwaita")
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.present();

    });

    application.run();

}
