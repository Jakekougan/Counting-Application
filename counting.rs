use std::ops::IndexMut;
use druid::{widget::{Button, Flex, Label, Axis, CrossAxisAlignment, MainAxisAlignment, RadioGroup, Split, TabInfo, Tabs, TabsEdge, TabsPolicy, TabsTransition, TextBox, ViewSwitcher}, AppLauncher, Data, Env, Widget, WindowDesc };
use druid::im::Vector;
use druid::{theme, Lens, Color,WidgetExt };
use dynamic_tab_data_derived_lenses::highest;
use instant::Duration;




#[derive(Clone, Data)]
struct Count {
    name: String,
    count: i32,
}

impl Count {
    fn increment(&mut self) {
        self.count += 1;
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    fn decrement(&mut self) {
        self.count -= 1;
    }
}

fn ui_builder() -> impl Widget<Count> {
    //Counter: _
    // + -
    let label = Label::new(|data: &Count, _: &Env| format!("Counter: {}", data.count));
    let increment = Button::new("+")
    .on_click(|_ctx, data: &mut Count, _env| data.increment());
    let decrement = Button::new("-")
    .on_click(|_ctx, data: &mut Count, _env| data.decrement());
    let reset = Button::new("Reset")
    .on_click(|_ctx, data: &mut Count, _env| data.reset());

    Flex::column().with_child(label).with_child(Flex::row().with_child(increment).with_child(decrement).with_child(reset))
}

struct Counts {
    counts: Vec<Count>
}

impl Counts {
    fn add_count_obj(&mut self, obj : Count) {
        self.counts.push(obj);
    }

    fn remove_count_obj(&mut self, obj : Count) {
        for i in 0..self.counts.len() {
            if self.counts.index_mut(i).name == obj.name {
                self.counts.remove(i);

            }
        }
    }

    fn clear(&mut self) {
        self.counts.clear();
    }
}

#[derive(Data, Clone, Lens)]
struct DynamicTabData {
    highest: usize,
    removed: usize,
    labels: Vector<usize>,
}

impl DynamicTabData {
    fn new(highest: usize) -> Self {
        DynamicTabData {
            highest,
            removed: 0,
            labels: (1..=highest).collect(),
        }
    }

    fn add(&mut self) {
        self.highest += 1;
        self.labels.push_back(self.highest);
    }

    fn remove(&mut self, indx: usize) {
        if >= self.labels.len() {
            tracing::warn!("Attempt to remove non existing tab tab at index {}", indx)

        } else {
            self.removed += 1;
            self.labels.remove(indx);
        }

    }

    fn tabs_key(&self) -> (usize, usize) {
        (self.highest, self.removed)
    }
}


fn main() {
    //Window Descripter
    // Launch
    let main_window = WindowDesc::new(ui_builder());
    AppLauncher::with_window(main_window)
    .log_to_console()
    .launch(Count{name: String::from("Default"), count: 0}).unwrap();

    println!("Hello World");
}




