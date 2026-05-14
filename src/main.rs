use wxdragon::prelude::*;
use wxdragon::widgets::list_ctrl::{ListColumnFormat, ListCtrlStyle};

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("SFZ Tool Prototype")
            .with_size(Size::new(1050, 720))
            .build();
        frame.set_name("SFZ Tool Prototype window");

        let root_panel = Panel::builder(&frame).build();
        root_panel.set_name("SFZ Tool main panel");
        let root_sizer = BoxSizer::builder(Orientation::Vertical).build();

        let heading = StaticText::builder(&root_panel)
            .with_label("SFZ instrument editor prototype")
            .build();
        root_sizer.add(
            &heading,
            0,
            SizerFlag::Expand | SizerFlag::All,
            8,
        );

        let body_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        let list_panel = Panel::builder(&root_panel).build();
        list_panel.set_name("Region list panel");
        let list_sizer = BoxSizer::builder(Orientation::Vertical).build();

        let region_list_label = StaticText::builder(&list_panel)
            .with_label("&Regions")
            .build();
        region_list_label.set_name("Regions label");
        list_sizer.add(
            &region_list_label,
            0,
            SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Top,
            8,
        );

        let region_list = create_region_list(&list_panel);
        list_sizer.add(
            &region_list,
            1,
            SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
            8,
        );
        list_panel.set_sizer(list_sizer, true);
        body_sizer.add(&list_panel, 1, SizerFlag::Expand, 0);

        let editor_panel = create_region_editor(&root_panel);
        body_sizer.add(
            &editor_panel,
            0,
            SizerFlag::Expand | SizerFlag::All,
            8,
        );

        root_sizer.add_sizer(&body_sizer, 1, SizerFlag::Expand, 0);

        let validation_label = StaticText::builder(&root_panel)
            .with_label("&Validation results")
            .build();
        validation_label.set_name("Validation results label");
        root_sizer.add(
            &validation_label,
            0,
            SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Top,
            8,
        );

        let validation = create_validation_list(&root_panel);
        root_sizer.add(
            &validation,
            0,
            SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
            8,
        );

        let status = StaticText::builder(&root_panel)
            .with_label("Ready. Prototype data is loaded for keyboard and screen-reader testing.")
            .build();
        root_sizer.add(
            &status,
            0,
            SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
            8,
        );

        root_panel.set_sizer(root_sizer, true);
        frame.show(true);
        frame.centre();
    });
}

fn create_region_list(parent: &dyn WxWidget) -> ListCtrl {
    let list = ListCtrl::builder(parent)
        .with_style(
            ListCtrlStyle::Report
                | ListCtrlStyle::SingleSel
                | ListCtrlStyle::HRules
                | ListCtrlStyle::VRules,
        )
        .build();
    list.set_name("Regions");
    list.set_tooltip("Regions list with sample, key range, velocity, and round robin columns");

    list.insert_column(0, "Sample", ListColumnFormat::Left, 230);
    list.insert_column(1, "Key", ListColumnFormat::Left, 60);
    list.insert_column(2, "Low key", ListColumnFormat::Left, 70);
    list.insert_column(3, "High key", ListColumnFormat::Left, 70);
    list.insert_column(4, "Root key", ListColumnFormat::Left, 70);
    list.insert_column(5, "Velocity", ListColumnFormat::Left, 80);
    list.insert_column(6, "Round robin", ListColumnFormat::Left, 90);

    add_region_row(&list, 0, ["Piano_C4_rr1.wav", "C4", "C4", "C4", "C4", "1-127", "1 of 2"]);
    add_region_row(&list, 1, ["Piano_C4_rr2.wav", "C4", "C4", "C4", "C4", "1-127", "2 of 2"]);
    add_region_row(&list, 2, ["Piano_D4_rr1.wav", "D4", "D4", "D4", "D4", "1-127", "1 of 1"]);

    list
}

fn add_region_row(list: &ListCtrl, row: i64, values: [&str; 7]) {
    list.insert_item(row, values[0], None);
    for (column, value) in values.iter().enumerate().skip(1) {
        list.set_item_text_by_column(row, column as i32, value);
    }
}

fn create_region_editor(parent: &dyn WxWidget) -> Panel {
    let panel = Panel::builder(parent).build();
    panel.set_name("Region editor");
    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    add_text_field(&panel, &sizer, "&Sample path", "samples/Piano_C4_rr1.wav");
    add_text_field(&panel, &sizer, "&Low key", "C4");
    add_text_field(&panel, &sizer, "&High key", "C4");
    add_text_field(&panel, &sizer, "&Root key", "C4");
    add_text_field(&panel, &sizer, "Low &velocity", "1");
    add_text_field(&panel, &sizer, "High v&elocity", "127");
    add_text_field(&panel, &sizer, "&Round robin position", "1");
    add_text_field(&panel, &sizer, "Round robin &count", "2");

    let loop_checkbox = CheckBox::builder(&panel)
        .with_label("Loop sample")
        .build();
    loop_checkbox.set_name("Loop sample");
    loop_checkbox.set_tooltip("Loop sample checkbox");
    sizer.add(&loop_checkbox, 0, SizerFlag::Expand | SizerFlag::All, 4);

    let trigger_label = StaticText::builder(&panel)
        .with_label("&Trigger")
        .build();
    sizer.add(&trigger_label, 0, SizerFlag::Expand | SizerFlag::All, 4);

    let trigger = Choice::builder(&panel)
        .with_choices(vec![
            "attack".to_string(),
            "release".to_string(),
            "first".to_string(),
            "legato".to_string(),
        ])
        .build();
    trigger.set_name("Trigger");
    trigger.set_tooltip("Trigger mode");
    trigger.set_selection(0);
    sizer.add(&trigger, 0, SizerFlag::Expand | SizerFlag::All, 4);

    let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    let add_button = Button::builder(&panel).with_label("&Add").build();
    let duplicate_button = Button::builder(&panel).with_label("&Duplicate").build();
    let delete_button = Button::builder(&panel).with_label("De&lete").build();

    button_sizer.add(&add_button, 1, SizerFlag::Expand | SizerFlag::All, 4);
    button_sizer.add(&duplicate_button, 1, SizerFlag::Expand | SizerFlag::All, 4);
    button_sizer.add(&delete_button, 1, SizerFlag::Expand | SizerFlag::All, 4);
    sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand, 0);

    panel.set_sizer(sizer, true);
    panel
}

fn add_text_field(parent: &Panel, sizer: &BoxSizer, label: &str, value: &str) {
    let static_text = StaticText::builder(parent).with_label(label).build();
    sizer.add(&static_text, 0, SizerFlag::Expand | SizerFlag::All, 4);

    let text = TextCtrl::builder(parent).with_value(value).build();
    sizer.add(&text, 0, SizerFlag::Expand | SizerFlag::All, 4);
}

fn create_validation_list(parent: &dyn WxWidget) -> ListCtrl {
    let list = ListCtrl::builder(parent)
        .with_style(
            ListCtrlStyle::Report
                | ListCtrlStyle::SingleSel
                | ListCtrlStyle::HRules
                | ListCtrlStyle::VRules,
        )
        .with_size(Size::new(1000, 120))
        .build();
    list.set_name("Validation results");
    list.set_tooltip("Validation results list");

    list.insert_column(0, "Severity", ListColumnFormat::Left, 90);
    list.insert_column(1, "Message", ListColumnFormat::Left, 650);
    list.insert_column(2, "Region", ListColumnFormat::Left, 150);

    list.insert_item(0, "Info", None);
    list.set_item_text_by_column(0, 1, "No validation has been run yet.");
    list.set_item_text_by_column(0, 2, "All regions");

    list
}
