mod emerald;

use gio::{ApplicationExt, ApplicationExtManual, Resource};
use gtk::*;
use pango::{AttrList, Attribute, Weight};
use std::env;

fn main() {
    gtk::init().expect("Failed to initialize GTK");

    let resources = Resource::load("PokeRNGTool.gresource").expect("Failed to load resources");
    gio::resources_register(&resources);

    let app = Application::new(Some("com.jms.PokeRNGTool"), Default::default())
        .expect("Failed to create GTK application");
    app.connect_activate(move |app| {
        let emerald_view_stack = emerald::view();

        let view_stack = Stack::new();
        let header_bar_stack = Stack::new();

        view_stack.add_named(&view(&view_stack, &header_bar_stack), "main");
        view_stack.add_named(&emerald_view_stack, "emerald");

        header_bar_stack.add_named(&header_bar(), "main");
        header_bar_stack.add_named(
            &emerald::header_bar(&view_stack, &header_bar_stack, &emerald_view_stack),
            "emerald",
        );

        let window = ApplicationWindowBuilder::new()
            .application(app)
            .default_width(750)
            .default_height(500)
            .border_width(18)
            .child(&view_stack.upcast::<Widget>())
            .build();
        window.set_titlebar(Some(&header_bar_stack));
        app.add_window(&window);
        window.show_all();
        window.present();
    });

    let args: Vec<String> = env::args().collect();
    app.run(&args);
}

fn view(view_stack: &Stack, header_bar_stack: &Stack) -> ScrolledWindow {
    let gen_text_attributes = &AttrList::new();
    gen_text_attributes.insert(Attribute::new_family("Monospace").unwrap());
    gen_text_attributes.insert(Attribute::new_weight(Weight::Bold).unwrap());
    gen_text_attributes.insert(Attribute::new_scale(3.2).unwrap());
    let gen3_label = LabelBuilder::new()
        .label("3")
        .attributes(&gen_text_attributes)
        .margin_end(6)
        .build();
    let gen4_label = LabelBuilder::new()
        .label("4")
        .attributes(&gen_text_attributes)
        .margin_end(6)
        .build();
    let gen5_label = LabelBuilder::new()
        .label("5")
        .attributes(&gen_text_attributes)
        .margin_end(6)
        .build();
    let gen6_label = LabelBuilder::new()
        .label("6")
        .attributes(&gen_text_attributes)
        .margin_end(6)
        .build();
    let gen7_label = LabelBuilder::new()
        .label("7")
        .attributes(&gen_text_attributes)
        .margin_end(6)
        .build();

    let game_text_attributes = &AttrList::new();
    game_text_attributes.insert(Attribute::new_weight(Weight::Bold).unwrap());
    let ruby_label = LabelBuilder::new()
        .label("Ruby")
        .attributes(&game_text_attributes)
        .build();
    let sapphire_label = LabelBuilder::new()
        .label("Sapphire")
        .attributes(&game_text_attributes)
        .build();
    let emerald_label = LabelBuilder::new()
        .label("Emerald")
        .attributes(&game_text_attributes)
        .build();
    let firered_label = LabelBuilder::new()
        .label("FireRed")
        .attributes(&game_text_attributes)
        .build();
    let leafgreen_label = LabelBuilder::new()
        .label("LeafGreen")
        .attributes(&game_text_attributes)
        .build();
    let diamond_label = LabelBuilder::new()
        .label("Diamond")
        .attributes(&game_text_attributes)
        .build();
    let pearl_label = LabelBuilder::new()
        .label("Pearl")
        .attributes(&game_text_attributes)
        .build();
    let platinum_label = LabelBuilder::new()
        .label("Platinum")
        .attributes(&game_text_attributes)
        .build();
    let heartgold_label = LabelBuilder::new()
        .label("HeartGold")
        .attributes(&game_text_attributes)
        .build();
    let soulsilver_label = LabelBuilder::new()
        .label("SoulSilver")
        .attributes(&game_text_attributes)
        .build();
    let black_label = LabelBuilder::new()
        .label("Black")
        .attributes(&game_text_attributes)
        .build();
    let white_label = LabelBuilder::new()
        .label("White")
        .attributes(&game_text_attributes)
        .build();
    let black2_label = LabelBuilder::new()
        .label("Black 2")
        .attributes(&game_text_attributes)
        .build();
    let white2_label = LabelBuilder::new()
        .label("White 2")
        .attributes(&game_text_attributes)
        .build();
    let x_label = LabelBuilder::new()
        .label("X")
        .attributes(&game_text_attributes)
        .build();
    let y_label = LabelBuilder::new()
        .label("Y")
        .attributes(&game_text_attributes)
        .build();
    let omega_ruby_label = LabelBuilder::new()
        .label("Omega Ruby")
        .attributes(&game_text_attributes)
        .build();
    let alpha_sapphire_label = LabelBuilder::new()
        .label("Alpha Sapphire")
        .attributes(&game_text_attributes)
        .build();
    let sun_label = LabelBuilder::new()
        .label("Sun")
        .attributes(&game_text_attributes)
        .build();
    let moon_label = LabelBuilder::new()
        .label("Moon")
        .attributes(&game_text_attributes)
        .build();
    let ultra_sun_label = LabelBuilder::new()
        .label("Ultra Sun")
        .attributes(&game_text_attributes)
        .build();
    let ultra_moon_label = LabelBuilder::new()
        .label("Ultra Moon")
        .attributes(&game_text_attributes)
        .build();

    let ruby_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/groudon.png");
    let sapphire_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/kyogre.png");
    let emerald_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/rayquaza.png");
    let firered_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/charizard.png");
    let leafgreen_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/venusaur.png");
    let diamond_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/dialga.png");
    let pearl_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/palkia.png");
    let platinum_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/giratina.png");
    let soulsilver_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/lugia.png");
    let heartgold_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/ho-oh.png");
    let black_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/zekrom.png");
    let white_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/reshiram.png");
    let black2_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/kyurem-black.png");
    let white2_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/kyurem-white.png");
    let x_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/xerneas.png");
    let y_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/yveltal.png");
    let omega_ruby_icon =
        Image::new_from_resource("/com/jms/PokeRNGTool/sprites/primal-groudon.png");
    let alpha_sapphire_icon =
        Image::new_from_resource("/com/jms/PokeRNGTool/sprites/primal-kyogre.png");
    let sun_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/solgaleo.png");
    let moon_icon = Image::new_from_resource("/com/jms/PokeRNGTool/sprites/lunala.png");
    let ultra_sun_icon =
        Image::new_from_resource("/com/jms/PokeRNGTool/sprites/necrozma-dusk-mane.png");
    let ultra_moon_icon =
        Image::new_from_resource("/com/jms/PokeRNGTool/sprites/necrozma-dawn-wings.png");

    let ruby_button = ButtonBuilder::new()
        .child(&ruby_icon.upcast::<Widget>())
        .build();
    let sapphire_button = ButtonBuilder::new()
        .child(&sapphire_icon.upcast::<Widget>())
        .build();
    let emerald_button = ButtonBuilder::new()
        .child(&emerald_icon.upcast::<Widget>())
        .build();
    emerald_button.connect_clicked({
        let view_stack = view_stack.clone();
        let header_bar_stack = header_bar_stack.clone();
        move |_| {
            view_stack.set_visible_child_name("emerald");
            header_bar_stack.set_visible_child_name("emerald");
        }
    });
    let firered_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&firered_icon.upcast::<Widget>())
        .build();
    let leafgreen_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&leafgreen_icon.upcast::<Widget>())
        .build();
    let diamond_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&diamond_icon.upcast::<Widget>())
        .build();
    let pearl_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&pearl_icon.upcast::<Widget>())
        .build();
    let platinum_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&platinum_icon.upcast::<Widget>())
        .build();
    let heartgold_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&heartgold_icon.upcast::<Widget>())
        .build();
    let soulsilver_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&soulsilver_icon.upcast::<Widget>())
        .build();
    let black_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&black_icon.upcast::<Widget>())
        .build();
    let white_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&white_icon.upcast::<Widget>())
        .build();
    let black2_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&black2_icon.upcast::<Widget>())
        .build();
    let white2_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&white2_icon.upcast::<Widget>())
        .build();
    let x_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&x_icon.upcast::<Widget>())
        .build();
    let y_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&y_icon.upcast::<Widget>())
        .build();
    let omega_ruby_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&omega_ruby_icon.upcast::<Widget>())
        .build();
    let alpha_sapphire_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&alpha_sapphire_icon.upcast::<Widget>())
        .build();
    let sun_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&sun_icon.upcast::<Widget>())
        .build();
    let moon_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&moon_icon.upcast::<Widget>())
        .build();
    let ultra_sun_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&ultra_sun_icon.upcast::<Widget>())
        .build();
    let ultra_moon_button = ButtonBuilder::new()
        .margin_top(6)
        .child(&ultra_moon_icon.upcast::<Widget>())
        .build();

    let grid = GridBuilder::new().row_spacing(6).column_spacing(12).build();
    grid.attach(&gen3_label, 0, 0, 1, 4);
    grid.attach(&gen4_label, 0, 4, 1, 4);
    grid.attach(&gen5_label, 0, 8, 1, 2);
    grid.attach(&gen6_label, 0, 10, 1, 2);
    grid.attach(&gen7_label, 0, 12, 1, 2);
    grid.attach(&ruby_button, 1, 0, 1, 1);
    grid.attach(&sapphire_button, 2, 0, 1, 1);
    grid.attach(&emerald_button, 3, 0, 1, 1);
    grid.attach(&ruby_label, 1, 1, 1, 1);
    grid.attach(&sapphire_label, 2, 1, 1, 1);
    grid.attach(&emerald_label, 3, 1, 1, 1);
    grid.attach(&firered_button, 1, 2, 1, 1);
    grid.attach(&leafgreen_button, 2, 2, 1, 1);
    grid.attach(&firered_label, 1, 3, 1, 1);
    grid.attach(&leafgreen_label, 2, 3, 1, 1);
    grid.attach(&diamond_button, 1, 4, 1, 1);
    grid.attach(&pearl_button, 2, 4, 1, 1);
    grid.attach(&platinum_button, 3, 4, 1, 1);
    grid.attach(&diamond_label, 1, 5, 1, 1);
    grid.attach(&pearl_label, 2, 5, 1, 1);
    grid.attach(&platinum_label, 3, 5, 1, 1);
    grid.attach(&heartgold_button, 1, 6, 1, 1);
    grid.attach(&soulsilver_button, 2, 6, 1, 1);
    grid.attach(&heartgold_label, 1, 7, 1, 1);
    grid.attach(&soulsilver_label, 2, 7, 1, 1);
    grid.attach(&black_button, 1, 8, 1, 1);
    grid.attach(&white_button, 2, 8, 1, 1);
    grid.attach(&black2_button, 3, 8, 1, 1);
    grid.attach(&white2_button, 4, 8, 1, 1);
    grid.attach(&black_label, 1, 9, 1, 1);
    grid.attach(&white_label, 2, 9, 1, 1);
    grid.attach(&black2_label, 3, 9, 1, 1);
    grid.attach(&white2_label, 4, 9, 1, 1);
    grid.attach(&x_button, 1, 10, 1, 1);
    grid.attach(&y_button, 2, 10, 1, 1);
    grid.attach(&omega_ruby_button, 3, 10, 1, 1);
    grid.attach(&alpha_sapphire_button, 4, 10, 1, 1);
    grid.attach(&x_label, 1, 11, 1, 1);
    grid.attach(&y_label, 2, 11, 1, 1);
    grid.attach(&omega_ruby_label, 3, 11, 1, 1);
    grid.attach(&alpha_sapphire_label, 4, 11, 1, 1);
    grid.attach(&sun_button, 1, 12, 1, 1);
    grid.attach(&moon_button, 2, 12, 1, 1);
    grid.attach(&ultra_sun_button, 3, 12, 1, 1);
    grid.attach(&ultra_moon_button, 4, 12, 1, 1);
    grid.attach(&sun_label, 1, 13, 1, 1);
    grid.attach(&moon_label, 2, 13, 1, 1);
    grid.attach(&ultra_sun_label, 3, 13, 1, 1);
    grid.attach(&ultra_moon_label, 4, 13, 1, 1);

    ScrolledWindowBuilder::new()
        .child(&grid.upcast::<Widget>())
        .build()
}

fn header_bar() -> HeaderBar {
    HeaderBarBuilder::new()
        .title("PokeRNGTool")
        .show_close_button(true)
        .build()
}
