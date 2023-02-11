mod custom_widgets;

use custom_widgets::{CustomButton, IntegerObject, Window};
use gtk4::CustomFilter;
use gtk4::CustomSorter;
use gtk4::FilterListModel;
use gtk4::ListItem;
use gtk4::ListView;
use gtk4::ScrolledWindow;
use gtk4::SelectionModel;
use gtk4::SignalListItemFactory;
use gtk4::SingleSelection;
use gtk4::SortListModel;
use gtk4::StringList;
use gtk4::{
    gio::{ListStore, Settings, SettingsBindFlags},
    glib,
    glib::{
        clone, closure_local, signal::Inhibit, BindingFlags, GString, MainContext, Object,
        PRIORITY_DEFAULT,
    },
    prelude::*,
    subclass::prelude::*,
    Application, ApplicationWindow, Button, Label, Orientation, Switch, Widget,
};
use std::{cell::Cell, rc::Rc, thread, time::Duration};

const APP_ID: &str = "org.gtk_rs.HelloWorld";

fn main() {
    // Create a new application
    let app: Application = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    let mut my_double: f32 = 5.0;
    let reference1: &mut f32 = &mut my_double;
    *reference1 += 2.5;
    let reference2: &mut f32 = &mut my_double;
    *reference2 += 2.5;
    assert_eq!(my_double, 10.0);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // let settings: Settings = Settings::new(APP_ID);
    let settings: Settings = Settings::with_path(APP_ID, "/org/gtk_rs/HelloWorld/");
    // // Get the last switch state from the settings
    // let is_switch_enabled: bool = settings.boolean("is-switch-enabled");

    // Create a `Vec<IntegerObject>` with numbers from 0 to 100_000
    let vector: Vec<IntegerObject> = (0..=1_000_000)
        .into_iter()
        .map(IntegerObject::new)
        .collect();

    // Create a new model
    let model: ListStore = ListStore::new(IntegerObject::static_type());

    // Add the vector to the model
    model.extend_from_slice(&vector);

    // Create a new model
    // let model: StringList = (0..=1_000_000)
    //     .into_iter()
    //     .map(|number: i32| number.to_string())
    //     .collect();

    let factory: SignalListItemFactory = SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item: &ListItem| {
        let label: Label = Label::new(None);
        let list_item = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem");
        list_item.set_child(Some(&label));
        // Bind `list_item->item->number` to `label->label`
        list_item
            .property_expression("item")
            .chain_property::<IntegerObject>("number")
            // .chain_property::<IntegerObject>("string")
            .bind(&label, "label", Widget::NONE);
    });

    let filter: CustomFilter = CustomFilter::new(move |obj| {
        // Get `IntegerObject` from `glib::Object`
        let integer_object: &IntegerObject = obj
            .downcast_ref::<IntegerObject>()
            .expect("The object needs to be of type `IntegerObject`.");

        // Get property "number" from `IntegerObject`
        let number: i32 = integer_object.property::<i32>("number");

        // Only allow even numbers
        number % 2 == 0
    });
    let filter_model: FilterListModel = FilterListModel::new(Some(&model), Some(&filter));

    let sorter: CustomSorter = CustomSorter::new(move |obj1, obj2| {
        // Get `IntegerObject` from `glib::Object`
        let integer_object_1 = obj1
            .downcast_ref::<IntegerObject>()
            .expect("The object needs to be of type `IntegerObject`.");
        let integer_object_2 = obj2
            .downcast_ref::<IntegerObject>()
            .expect("The object needs to be of type `IntegerObject`.");

        // Get property "number" from `IntegerObject`
        let number_1 = integer_object_1.property::<i32>("number");
        let number_2 = integer_object_2.property::<i32>("number");

        // Reverse sorting order -> large numbers come first
        number_2.cmp(&number_1).into()
    });
    let sort_model: SortListModel = SortListModel::new(Some(&filter_model), Some(&sorter));

    // factory.connect_bind(move |_, list_item: &ListItem| {
    //     // Get `IntegerObject` from `ListItem`
    //     let integer_object = list_item
    //         .downcast_ref::<ListItem>()
    //         .expect("Needs to be ListItem")
    //         .item()
    //         .and_downcast::<IntegerObject>()
    //         .expect("The item has to be an `IntegerObject`.");

    //     // Get `i32` from `IntegerObject`
    //     let number: i32 = integer_object.property::<i32>("number");

    //     // Get `Label` from `ListItem`
    //     let label: Label = list_item
    //         .downcast_ref::<ListItem>()
    //         .expect("Needs to be ListItem")
    //         .child()
    //         .and_downcast::<Label>()
    //         .expect("The child has to be a `Label`.");

    //     // Set "label" to "number"
    //     label.set_label(&number.to_string());
    // });

    // Create a custom button
    let custom_button: CustomButton = CustomButton::new();
    custom_button.set_margin_top(12);
    custom_button.set_margin_bottom(12);
    custom_button.set_margin_start(12);
    custom_button.set_margin_end(12);
    // Create a custom button
    let custom_button_1: CustomButton = CustomButton::new();
    custom_button_1.set_margin_top(12);
    custom_button_1.set_margin_bottom(12);
    custom_button_1.set_margin_start(12);
    custom_button_1.set_margin_end(12);

    // Create a custom button
    let custom_button_2: CustomButton = CustomButton::new();
    custom_button_2.set_margin_top(12);
    custom_button_2.set_margin_bottom(12);
    custom_button_2.set_margin_start(12);
    custom_button_2.set_margin_end(12);

    custom_button.connect_closure(
        "max-number-reached",
        false,
        closure_local!(move |_: CustomButton, number: i32| {
            println!("The maximum number {} has been reached", number);
        }),
    );

    // Assure that "number" of `button_2` is always 1 higher than "number" of `button_1`
    custom_button_1
        .bind_property("number", &custom_button_2, "number")
        // How to transform "number" from `button_1` to "number" of `button_2`
        .transform_to(|_, number: i32| {
            let incremented_number: i32 = number + 1;
            Some(incremented_number.to_value())
        })
        // How to transform "number" from `button_2` to "number" of `button_1`
        .transform_from(|_, number: i32| {
            let decremented_number: i32 = number - 1;
            Some(decremented_number.to_value())
        })
        .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
        .build();

    // The closure will be called
    // whenever the property "number" of `button_1` gets changed
    custom_button_1.connect_notify_local(Some("number"), move |button, _| {
        let number: i32 = button.property::<i32>("number");
        println!("The current number of `button_1` is {}.", number);
    });

    // Create a switch
    let switch_1: Switch = Switch::new();
    let switch_2: Switch = Switch::builder()
        .margin_top(48)
        .margin_bottom(48)
        .margin_start(48)
        .margin_end(48)
        .build();

    settings
        .bind("is-switch-enabled", &switch_2, "state")
        .flags(SettingsBindFlags::DEFAULT)
        .build();

    // Set and then immediately obtain state
    switch_1
        .bind_property("state", &switch_2, "state")
        .flags(BindingFlags::BIDIRECTIONAL)
        .build();

    // Create a button
    let button_increase: Button = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // Create a button
    let button_decrease: Button = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // Create a label
    let label: Label = Label::builder()
        .label("Counter: 0")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // Create a button
    let button: Button = Button::builder()
        .label("Sleep for 10s")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Reference-counted object with inner-mutability
    let number: Rc<Cell<i32>> = Rc::new(Cell::new(0));

    custom_button_1.connect_notify_local(Some("number"), move |button, _| {
        let number: i32 = button.property::<i32>("number");
        println!("The current number of `button_1` is {}.", number);
    });

    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        let sender = sender.clone();
        // The long running operation runs now in a separate thread
        thread::spawn(move || {
            // Deactivate the button until the operation is done
            sender.send(false).expect("Could not send through channel");
            let ten_seconds = Duration::from_secs(10);
            thread::sleep(ten_seconds);
            // Activate the button again
            sender.send(true).expect("Could not send through channel");
        });
    });
    // The main loop executes the closure as soon as it receives the message
    receiver.attach(
        None,
        clone!(@weak button => @default-return Continue(false),
                    move |enable_button| {
                        button.set_sensitive(enable_button);
                        Continue(true)
                    }
        ),
    );

    // // Connect to "clicked" signal of `button`
    // button.connect_clicked(move |button| {
    //     let main_context = MainContext::default();
    //     // The main loop executes the asynchronous block
    //     main_context.spawn_local(clone!(@weak button => async move {
    //         // Deactivate the button until the operation is done
    //         button.set_sensitive(false);
    //         timeout_future_seconds(5).await;
    //         // Activate the button again
    //         button.set_sensitive(true);
    //     }));
    // });

    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(clone!(@weak label, @weak number =>
        move |_| {
            number.set(number.get() + 1);
            label.set_label(format!("Counter: {}", &number.get()).as_str());
    }));
    button_decrease.connect_clicked(clone!(@weak label =>
        move |_| {
            number.set(number.get() - 1);
            label.set_label(format!("Counter: {}", &number.get()).as_str());
    }));

    let list_box: gtk4::ListBox = gtk4::ListBox::new();
    for number in 0..=100_000 {
        let number_label: Label = Label::new(Some(&number.to_string()));
        list_box.append(&number_label);
    }

    // Add buttons to `gtk_box`
    let gtk_box: gtk4::Box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&switch_1);
    gtk_box.append(&switch_2);
    gtk_box.append(&custom_button);
    gtk_box.append(&custom_button_1);
    gtk_box.append(&custom_button_2);
    gtk_box.append(&button);
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    gtk_box.append(&label);

    // // Create custom window
    // let window = Window::new(application);
    // window.set_title(Some("My GTK App"));
    // // Add button
    // window.set_child(Some(&gtk_box));

    let selection_model: SingleSelection = SingleSelection::new(Some(&sort_model));
    let list_view: ListView = ListView::new(Some(&selection_model), Some(&factory));

    list_view.connect_activate(move |list_view: &ListView, position: u32| {
        // Get `IntegerObject` from model
        let model = list_view.model().expect("The model has to exist.");
        let integer_object = model
            .item(position)
            .and_downcast::<IntegerObject>()
            .expect("The item has to be an `IntegerObject`.");

        // Increase "number" of `IntegerObject`
        integer_object.increase_number();

        // Notify that the filter and sorter have been changed
        filter.changed(gtk4::FilterChange::Different);
        sorter.changed(gtk4::SorterChange::Different);
    });

    // Create a window
    let scrolled_window: ScrolledWindow = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .min_content_width(360)
        .child(&list_view)
        .build();

    // Create a window
    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();

    // Present window
    window.present();
}
