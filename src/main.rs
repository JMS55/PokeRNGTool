use gio::{ApplicationExt, ApplicationExtManual, Resource};
use gtk::{
    Application, ApplicationWindow, Builder, BuilderExtManual, GtkApplicationExt, GtkWindowExt,
    GtkWindowExtManual,
};
use std::env;

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let resources = Resource::load("PokeRNGTool.gresource").expect("Failed to load resources");
    gio::resources_register(&resources);

    let app = Application::new(Some("com.jms.PokeRNGTool"), Default::default())
        .expect("Failed to create GTK application");
    app.connect_activate(move |app| {
        let window: ApplicationWindow =
            Builder::new_from_resource("/com/jms/PokeRNGTool/ui/main.ui")
                .get_object("window")
                .expect("Failed to find the window widget");
        window.set_application(Some(app));
        app.add_window(&window);
        window.present();
    });

    let args: Vec<String> = env::args().collect();
    app.run(&args);
}
