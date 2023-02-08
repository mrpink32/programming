use gtk4::{
    gio::Settings,
    glib::{
        self,
        once_cell::sync::{Lazy, OnceCell},
        signal::Inhibit,
        subclass::Signal,
        BindingFlags, ParamSpec, ParamSpecInt, Value,
    },
    prelude::*,
    subclass::prelude::*,
    ApplicationWindow,
};
use std::cell::Cell;

// Object holding the state
#[derive(Default)]
pub struct CustomButton {
    number: Cell<i32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk4::Button;
}

static MAX_NUMBER: i32 = 10;

// Trait shared by all GObjects
impl ObjectImpl for CustomButton {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("max-number-reached")
                .param_types([i32::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecInt::builder("number").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "number" => {
                let input_number: i32 = value.get().expect("The value needs to be of type `i32`.");
                self.number.replace(input_number);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "number" => self.number.get().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();
        obj.bind_property("number", obj.as_ref(), "label")
            .flags(BindingFlags::SYNC_CREATE)
            .build();
    }
}

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        let incremented_number: i32 = self.number.get() + 1;
        let obj = self.obj();

        if incremented_number == MAX_NUMBER {
            obj.emit_by_name::<()>("max-number-reached", &[&incremented_number]);
            obj.set_property("number", &0);
        } else {
            obj.set_property("number", &incremented_number);
        }
        // self.obj().set_property("number", &incremented_number);
    }
}

#[derive(Default)]
pub struct Window {
    pub settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = ApplicationWindow;
}
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        // Load latest window state
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}
impl WidgetImpl for Window {}
impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self) -> Inhibit {
        // Save window size
        self.obj()
            .save_window_size()
            .expect("Failed to save window state");

        // Don't inhibit the default handler
        Inhibit(false)
    }
}
impl ApplicationWindowImpl for Window {}
