
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button };

fn main() {
    
    println!("Hello, world!");

    // Builder App
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    // Connect, Active App - Builder & Show Window
    app.connect_activate(|app| {
        
        // Builder Main Window
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(480)
            .default_height(320)
            .title("Hello World")
            .build();

        
        // Buutons
        let button = Button::with_label("Click me");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));
        
        // Show the window
        window.present();

    });

    // Run App
    app.run();
}
