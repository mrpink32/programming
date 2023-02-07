mod custom_widgets;

use custom_widgets::CustomButton;
use gtk4::{
    glib,
    glib::{clone, closure_local, BindingFlags, GString, MainContext, Object, PRIORITY_DEFAULT},
    prelude::*,
    subclass::prelude::*,
    Application, ApplicationWindow, Button, Label, Orientation, Switch,
};
use std::{cell::Cell, rc::Rc, thread, time::Duration};

const APP_ID: &str = "org.gtk_rs.HelloWorld";

fn main() {
    // Create a new application
    let app: Application = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    let mut my_int: i32 = 5;
    let reference1: &mut i32 = &mut my_int;
    *reference1 += 1;
    let reference2: &mut i32 = &mut my_int;
    *reference2 += 2;
    assert_eq!(my_int, 7);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
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
    let switch_2: Switch = Switch::new();

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

    // Create a window
    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present window
    window.present();
}
