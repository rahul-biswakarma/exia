use crate::utils::lib::use_unique_id;
use dioxus::prelude::*;
use std::fmt;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl CalendarDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    pub fn today() -> Self {


        Self {
            year: 2024,
            month: 5,
            day: 15,
        }
    }

    pub fn format(&self, _format: &str) -> String {

        format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }


    pub fn first_day_of_month(&self) -> u32 {


        ((self.day + 6) % 7) + 1
    }


    pub fn days_in_month(&self) -> u32 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {

                if self.year % 4 == 0 && (self.year % 100 != 0 || self.year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }


    pub fn prev_month(&self) -> Self {
        if self.month == 1 {
            Self {
                year: self.year - 1,
                month: 12,
                day: 1,
            }
        } else {
            Self {
                year: self.year,
                month: self.month - 1,
                day: 1,
            }
        }
    }


    pub fn next_month(&self) -> Self {
        if self.month == 12 {
            Self {
                year: self.year + 1,
                month: 1,
                day: 1,
            }
        } else {
            Self {
                year: self.year,
                month: self.month + 1,
                day: 1,
            }
        }
    }


    pub fn is_same_day(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }


    pub fn is_same_month(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month
    }
}

impl fmt::Display for CalendarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CalendarMode {
    Day,
    Month,
    Year,
}


#[allow(dead_code)]
#[derive(Clone)]
struct CalendarContext {

    selected_date: ReadOnlySignal<Option<CalendarDate>>,
    set_selected_date: Callback<Option<CalendarDate>>,
    view_date: ReadOnlySignal<CalendarDate>,
    set_view_date: Callback<CalendarDate>,
    mode: ReadOnlySignal<CalendarMode>,
    set_mode: Callback<CalendarMode>,


    disabled: ReadOnlySignal<bool>,
    disabled_dates: ReadOnlySignal<Vec<CalendarDate>>,
    min_date: ReadOnlySignal<Option<CalendarDate>>,
    max_date: ReadOnlySignal<Option<CalendarDate>>,


    calendar_id: ReadOnlySignal<String>,
}


#[derive(Props, Clone, PartialEq)]
pub struct CalendarProps {

    #[props(default)]
    selected_date: Option<CalendarDate>,


    #[props(default)]
    on_date_change: Callback<Option<CalendarDate>>,


    view_date: CalendarDate,


    #[props(default)]
    on_view_change: Callback<CalendarDate>,


    #[props(default = CalendarMode::Month)]
    mode: CalendarMode,


    #[props(default)]
    on_mode_change: Callback<CalendarMode>,


    #[props(default)]
    disabled: ReadOnlySignal<bool>,


    #[props(default = ReadOnlySignal::new(Signal::new(Vec::new())))]
    disabled_dates: ReadOnlySignal<Vec<CalendarDate>>,


    #[props(default = ReadOnlySignal::new(Signal::new(None)))]
    min_date: ReadOnlySignal<Option<CalendarDate>>,


    #[props(default = ReadOnlySignal::new(Signal::new(None)))]
    max_date: ReadOnlySignal<Option<CalendarDate>>,


    #[props(default)]
    id: Option<String>,


    #[props(default = 1)]
    first_day_of_week: u32,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,


    children: Element,
}


#[component]
pub fn Calendar(props: CalendarProps) -> Element {

    let mut mode = use_signal(|| props.mode);
    let set_mode = Callback::new(move |new_mode: CalendarMode| {
        mode.set(new_mode);
        props.on_mode_change.call(new_mode);
    });


    let calendar_id = match props.id {
        Some(ref id) => use_signal(|| id.clone()),
        None => use_unique_id(),
    };


    let _ctx = use_context_provider(|| CalendarContext {
        selected_date: ReadOnlySignal::new(Signal::new(props.selected_date.clone())),
        set_selected_date: props.on_date_change.clone(),
        view_date: ReadOnlySignal::new(Signal::new(props.view_date.clone())),
        set_view_date: props.on_view_change.clone(),
        mode: mode.into(),
        set_mode,
        disabled: props.disabled,
        disabled_dates: props.disabled_dates,
        min_date: props.min_date,
        max_date: props.max_date,
        calendar_id: calendar_id.into(),
    });

    rsx! {
        div {
            role: "application",
            "aria-label": "Calendar",
            id: props.id,
            "data-disabled": (props.disabled)(),
            ..props.attributes,

            {props.children}
        }
    }
}


#[derive(Props, Clone, PartialEq)]
pub struct CalendarHeaderProps {

    #[props(default)]
    id: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,


    children: Element,
}


#[component]
pub fn CalendarHeader(props: CalendarHeaderProps) -> Element {
    let _ctx: CalendarContext = use_context();

    rsx! {
        div {
            role: "heading",
            "aria-level": "2",
            id: props.id,
            ..props.attributes,

            {props.children}
        }
    }
}


#[derive(Props, Clone, PartialEq)]
pub struct CalendarNavigationProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,


    #[props(default)]
    children: Element,
}


#[component]
pub fn CalendarNavigation(props: CalendarNavigationProps) -> Element {
    let ctx: CalendarContext = use_context();


    let handle_prev_month = move |e: Event<MouseData>| {
        e.prevent_default();
        let current_view = (ctx.view_date)();
        ctx.set_view_date.call(current_view.prev_month());
    };


    let handle_next_month = move |e: Event<MouseData>| {
        e.prevent_default();
        let current_view = (ctx.view_date)();
        ctx.set_view_date.call(current_view.next_month());
    };


    let month_year = use_memo(move || {
        let view_date = (ctx.view_date)();
        let month_names = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        let month_name = month_names[(view_date.month - 1) as usize];
        format!("{} {}", month_name, view_date.year)
    });

    rsx! {
        div { class: "calendar-navigation", ..props.attributes,


            button {
                class: "calendar-nav-prev",
                aria_label: "Previous month",
                r#type: "button",
                onclick: handle_prev_month,
                disabled: (ctx.disabled)(),
                "←"
            }

            div { class: "calendar-nav-title", {month_year} }

            button {
                class: "calendar-nav-next",
                aria_label: "Next month",
                r#type: "button",
                onclick: handle_next_month,
                disabled: (ctx.disabled)(),
                "→"
            }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
pub struct CalendarGridProps {

    #[props(default)]
    id: Option<String>,


    #[props(default)]
    show_week_numbers: bool,


    #[props(default = vec!["Mo".to_string(), "Tu".to_string(), "We".to_string(), "Th".to_string(), "Fr".to_string(), "Sa".to_string(), "Su".to_string()])]
    day_labels: Vec<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}


#[component]
pub fn CalendarGrid(props: CalendarGridProps) -> Element {
    let ctx: CalendarContext = use_context();





    let days_grid = use_memo(move || {

        let view_date = (ctx.view_date)();
        println!("Generating grid for {}-{}", view_date.year, view_date.month);
        let days_in_month = view_date.days_in_month();




        let first_day_offset = 2;


        let mut grid = Vec::new();


        for _ in 0..first_day_offset {
            grid.push(None);
        }


        for day in 1..=days_in_month {
            grid.push(Some(day));
        }


        let remainder = grid.len() % 7;
        if remainder > 0 {
            for _ in 0..(7 - remainder) {
                grid.push(None);
            }
        }

        grid
    });


    let handle_day_select = move |day: u32| {
        if !(ctx.disabled)() {
            let view_date = (ctx.view_date)();
            let date = CalendarDate::new(view_date.year, view_date.month, day);
            ctx.set_selected_date.call(Some(date));
        }
    };

    rsx! {
        div {
            role: "grid",
            id: props.id,
            class: "calendar-grid",
            ..props.attributes,


            div { role: "row", class: "calendar-grid-header",


                for day_label in &props.day_labels {
                    div {
                        role: "columnheader",
                        class: "calendar-grid-day-header",
                        {day_label.clone()}
                    }
                }
            }


            div { class: "calendar-grid-body",


                div { class: "calendar-grid-days",


                    for day_opt in days_grid() {
                        if let Some(day) = day_opt {
                            button {
                                class: "calendar-grid-cell",
                                onclick: move |e| {
                                    e.prevent_default();
                                    handle_day_select(day);
                                },
                                r#type: "button",
                                "data-today": day == (ctx.view_date)().day,
                                "data-selected": (ctx.selected_date)()
                                    .is_some_and(|d| {
                                        d.day == day && d.month == (ctx.view_date)().month
                                            && d.year == (ctx.view_date)().year
                                    }),
                                {day.to_string()}
                            }
                        } else {

                            div { class: "calendar-grid-cell calendar-grid-cell-empty" }
                        }
                    }
                }
            }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
pub struct CalendarCellProps {

    date: CalendarDate,


    #[props(default)]
    is_selected: bool,


    #[props(default)]
    is_today: bool,


    #[props(default)]
    is_disabled: bool,


    #[props(default)]
    onclick: EventHandler<MouseEvent>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}


#[component]
pub fn CalendarCell(props: CalendarCellProps) -> Element {
    let _ctx: CalendarContext = use_context();


    let state_class = if props.is_selected {
        "calendar-grid-cell-selected"
    } else if props.is_today {
        "calendar-grid-cell-today"
    } else {
        ""
    };

    rsx! {
        button {
            role: "gridcell",
            class: "calendar-grid-cell {state_class}",
            "aria-selected": props.is_selected,
            "aria-disabled": props.is_disabled,
            r#type: "button",
            disabled: props.is_disabled,
            "data-selected": props.is_selected,
            "data-today": props.is_today,
            "data-disabled": props.is_disabled,
            tabindex: if props.is_selected { "0" } else { "-1" },
            onclick: props.onclick,
            ..props.attributes,

            {props.date.day.to_string()}
        }
    }
}
