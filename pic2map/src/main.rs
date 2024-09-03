use gtk4::prelude::*;
// use gdk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Builder, Label, CssProvider };

use gdk4::Display;
 use std::path::Path;

fn main() {
    let application: Application = Application::builder()
        .application_id("net.cylancer.pic2Map")
        .build();
    application.connect_startup(|_| load_css());

    application.connect_activate(|app: &Application| {
        let glade_src = include_str!("layout/pic2map.ui");
        let builder: Builder = Builder::from_string(glade_src);
        let label: Label = builder.object("demoLabel").expect("not found");
        label.set_text("label123");
        let window: ApplicationWindow = builder.object("demoWindow").expect("ooo");
        window.set_application(Some(app));
        window.present();
    });
    // load_css();
    application.run();
}


fn load_css() {
    
    let css_path = Path::new("layout/pic2map.css");
    
    let provider = CssProvider::new();
    provider.load_from_path(css_path);

    // Add the provider to the default screen
    gtk4::style_context_add_provider_for_display( 
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
