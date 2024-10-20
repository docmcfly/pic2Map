// use gdk::gio::null_settings_backend_new;
use gdk4::Display;
use gtk4::prelude::*;

use gtk4::{
    glib, Application, ApplicationWindow, Box, Builder, Button, CssProvider, Label, Orientation, GestureClick
};

use libadwaita::Clamp as AdwClamp;

use libshumate::{MapSourceRegistry, SimpleMap, MAP_SOURCE_OSM_MAPNIK};

const APP_ID: &str = "net.cylancer.pic2map";

fn main() -> glib::ExitCode {
    // Create a new application
    let app: Application = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    // Connect to "activate" signal of `app`
    app.connect_activate(|app| {
        let glade_src = include_str!("ui/pic2map.ui");
        let builder: Builder = Builder::from_string(glade_src);
        //   let builder_file = File::create("ui/pic2map.ui").expect("msg");
        //  let builder: Builder = Builder::from_file("ui/pic2map.ui");

        let map_widget: SimpleMap = builder.object("map_widget").unwrap();
        let registry = MapSourceRegistry::with_defaults();

        // Use OpenStreetMap as the source
        let map_source = registry.by_id(MAP_SOURCE_OSM_MAPNIK);
        let viewport = map_widget.viewport().unwrap();
        let map = map_widget.map().unwrap();

        map_widget.set_map_source(map_source.as_ref());
        map.center_on(0., 0.);

        // Reference map source used by MarkerLayer
        viewport.set_reference_map_source(map_source.as_ref());
        viewport.set_zoom_level(5.);

     
        let gesture = GestureClick::new();
        map_widget.add_controller(gesture.clone());




        let window: ApplicationWindow = ApplicationWindow::new(app);
        // builder.object("window").expect("msg");
        window.set_application(Some(app));

        let label: Label = builder.object("label").expect("msg");
        label.set_text("hallo welt");

        window.set_decorated(true);
        // let window = ApplicationWindow::builder()
        //     .application(app)
        window.set_default_height(200);
        window.set_default_width(400);
        window.set_title(Some("Pic2Map - Verorte Deine Bilder..."));
        //     .build();
        // Present windo w
        // Erstelle einen Button
        let button = Button::with_label("Klick mich!");
        button.connect_clicked(|_| {
            println!("Button wurde geklickt!");
        });

        let main_box: Box = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .build();
        // let content_box: Box = builder.object("content").expect("msg");

        let adw_clamp: AdwClamp = builder.object("adwClamp").expect("msg");
        main_box.append(&adw_clamp);

        window.set_child(Some(&main_box));
        window.present();
    });

    // Create a window and set the title

    // Run the application
    app.run()
}

fn load_css() {
    let css = include_str!("ui/pic2map.css");
    // let css_file: gio::File = File::

    let provider: CssProvider = CssProvider::new();
    provider.load_from_data(css);

    // provider.lo();

    // Add the provider to the default screen
    gtk4::style_context_add_provider_for_display(
        // gdk::Display.get_default(),
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_SETTINGS,
    );
}
