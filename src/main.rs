mod calculate;
use crate::calculate::consts::*;
use crate::calculate::rates::new_rates;
use crate::calculate::saver::{Saver, SaverType};

use leptos::view;
use leptos::*;
use leptos::{component, create_signal, CollectView, IntoView, SignalGet};
use num_format::{Locale, ToFormattedString};

use plotly::color::NamedColor;
use plotly::common::Title;
use plotly::Plot;
use plotly::Scatter;

use std::str::FromStr;

fn main() {
    leptos::mount_to_body(|| view! { <Savings/> })
}

#[component]
fn SelectOpts(options: ReadSignal<[Opts; 100]>) -> impl IntoView {
    let l = move |_| logging::error!("changing");
    move || {
        let options = options.get();
        let mut format_percent = false;
        if let Some(last_val) = options.last() {
            match last_val {
                Opts::Int(_) => {}
                Opts::Float(x) => {
                    if x < &1.0 {
                        format_percent = true;
                    }
                }
            }
        }
        options
            .into_iter()
            .enumerate()
            .map(|(i, opt)| match opt {
                Opts::Int(opt) => {
                    if i == 50 {
                        view! {
                            <option type="number" selected="selected" value=opt>
                                {opt}
                            </option>
                        }
                    } else {
                        view! {
                            <option type="number" on:change=l value=opt>
                                {opt}
                            </option>
                        }
                    }
                }
                Opts::Float(opt) => {
                    if i == 30 {
                        view! {
                            <option type="number" selected="selected" value=opt>

                                {if format_percent {
                                    let mut val = ((opt * 1000.0).trunc() as i32)
                                        .to_formatted_string(&Locale::en) + "%";
                                    val.insert(val.len() - 2, '.');
                                    val
                                } else {
                                    (opt.trunc() as i32).to_formatted_string(&Locale::en)
                                }}

                            </option>
                        }
                    } else {
                        view! {
                            <option type="number" value=opt>

                                {if format_percent {
                                    let mut val = ((opt * 1000.0).trunc() as i32)
                                        .to_formatted_string(&Locale::en) + "%";
                                    val.insert(val.len() - 2, '.');
                                    val
                                } else {
                                    (opt.trunc() as i32).to_formatted_string(&Locale::en)
                                }}

                            </option>
                        }
                    }
                }
            })
            .collect_view()
    }
}

#[component]
fn Savings() -> impl IntoView {
    let (width, set_width) = create_signal(None::<f64>);
    let fetch_width = move || {
        match leptos::window().inner_width() {
            Ok(val) => {
                set_width.set(val.as_f64());
            }
            Err(e) => {
                logging::error!("Error getting window width: {:?}", e);
                set_width.set(None);
            }
        };
    };

    let (height, set_height) = create_signal(None::<f64>);
    let fetch_height = move || {
        match leptos::window().inner_height() {
            Ok(val) => {
                set_height.set(val.as_f64());
            }
            Err(e) => {
                logging::error!("Error getting window height: {:?}", e);
                set_height.set(None);
            }
        };
    };

    let plot_width = move || width.get().map(|width| width * 0.75);

    let plot_height = move || height.get().map(|height| height * 0.5);

    leptos::window_event_listener(ev::resize, move |_| {
        fetch_width();
        fetch_height();
    });

    let (age_range, _) = create_signal(AGE_RANGE);
    let (mortgage_range, _) = create_signal(MORTGAGE_RATES);
    let (networth_range, _) = create_signal(NETWORTH_RANGE);
    let (incexp_range, _) = create_signal(INCEXP_RANGE);

    let (age, set_age) = create_signal(Opts::Int(45));
    let (networth, set_networth) = create_signal(Opts::Float(100000.0));
    let (retirement_age, set_retirement_age) = create_signal(Opts::Int(55));
    let (monthly_income, set_monthly_income) = create_signal(Opts::Float(5000.0));
    let (monthly_expenses, set_monthly_expenses) = create_signal(Opts::Float(3000.0));
    let (rent, set_rent) = create_signal(Opts::Float(1000.0));
    let (home_value, set_home_value) = create_signal(Opts::Float(200000.0));
    let (mortgage, set_mortgage) = create_signal(Opts::Float(100000.0));
    let (mortgage_rate, set_mortgage_rate) = create_signal(Opts::Float(0.05));
    let (mortgage_term, set_mortgage_term) = create_signal(Opts::Int(30));
    let (min_retirement_income, set_min_retirement_income) = create_signal(Opts::Float(3000.0));
    let (max_retirement_income, set_max_retirement_income) = create_signal(Opts::Float(5000.0));

    let (inflation_rates, set_inflation_rates) = create_signal(vec![Opts::Float(0.0); 100]);
    let (interest_rates, set_interest_rates) = create_signal(vec![Opts::Float(0.0); 100]);

    let (owner_savings_arr, set_owner_savings_arr) = create_signal(vec![0.0; 100]);
    let (renter_savings_arr, set_renter_savings_arr) = create_signal(vec![0.0; 100]);

    let owner_savings = move || {
        Saver {
            monthly_rent: 0.0,
            current_age: age.get().get_int(),
            retirement_age: retirement_age.get().get_int(),
            total_savings: networth.get().get_float(),
            monthly_income: monthly_income.get().get_float(),
            monthly_expenses: monthly_expenses.get().get_float(),
            home_value: home_value.get().get_float(),
            mortgage_debt: mortgage.get().get_float(),
            mortgage_rate: mortgage_rate.get().get_float(),
            mortgage_term: mortgage_term.get().get_int(),
            min_baseline_retirement_income: min_retirement_income.get().get_float(),
            max_baseline_retirement_income: max_retirement_income.get().get_float(),
            home_expenses: 0.0,
            home_savings: vec![0.0; DEATH],
            rental_savings: vec![0.0; DEATH],
            active_retirement: false,
            home_owned_age: None::<u8>,
            cached_mortgage_installment: None::<f32>,
            interest_rates,
            inflation_rates,
        }
        .calculate_savings(SaverType::HomeOwner, DEATH as u8)
    };
    let owner_savings_derived = create_memo(move |_| {
        set_owner_savings_arr.set(owner_savings());
    });

    let renter_savings = move || {
        Saver {
            monthly_rent: rent.get().get_float(),
            current_age: age.get().get_int(),
            retirement_age: retirement_age.get().get_int(),
            total_savings: networth.get().get_float(),
            monthly_income: monthly_income.get().get_float(),
            monthly_expenses: monthly_expenses.get().get_float(),
            home_value: 0.0,
            mortgage_debt: 0.0,
            mortgage_rate: 0.0,
            mortgage_term: 0,
            min_baseline_retirement_income: min_retirement_income.get().get_float(),
            max_baseline_retirement_income: max_retirement_income.get().get_float(),
            home_expenses: 0.0,
            home_savings: vec![0.0; DEATH],
            rental_savings: vec![0.0; DEATH],
            active_retirement: false,
            home_owned_age: None::<u8>,
            cached_mortgage_installment: None::<f32>,
            interest_rates,
            inflation_rates,
        }
        .calculate_savings(SaverType::Renter, DEATH as u8)
    };

    let renter_savings_derived = create_memo(move |_| {
        set_renter_savings_arr.set(renter_savings());
    });

    let plot_resource = create_local_resource(
        move || (width, height),
        move |_| async move {
            let mut plot = Plot::new();
            let start_x_value = age.get_untracked().get_int() as usize;
            let x_values = &AGE_RANGE_FLOATS[start_x_value..DEATH];

            let owner_trace = Scatter::new(
                x_values.to_vec(),
                owner_savings_arr.get_untracked()[start_x_value..DEATH].to_vec(),
            )
            //.mode(Mode::Lines)
            .visible(plotly::common::Visible::True)
            .fill_color(NamedColor::PaleTurquoise)
            .name("Owner");

            let renter_trace = Scatter::new(
                x_values.to_vec(),
                renter_savings_arr.get_untracked()[start_x_value..DEATH].to_vec(),
            )
            .visible(plotly::common::Visible::True)
            .fill_color(NamedColor::LightSalmon)
            .name("Renter");

            plot.add_trace(owner_trace);
            plot.add_trace(renter_trace);

            // sizing plot
            let err_sizing = || {
                plotly::Layout::new()
                    .title(Title::new("Renting vs Owning"))
                    .show_legend(true)
                    .auto_size(true)
            };
            if let (Some(width), Some(height)) = (plot_width(), plot_height()) {
                plot.set_layout(
                    plotly::Layout::new()
                        .title(Title::new("Renting vs Owning"))
                        .show_legend(true)
                        .width(width as usize)
                        .height(height as usize),
                );
            } else {
                plot.set_layout(err_sizing())
            }

            //plot.set_layout(layout);
            plotly::bindings::new_plot("plot", &plot).await;
        },
    );

    create_effect(move |_| {
        age.get();
        networth.get();
        retirement_age.get();
        monthly_income.get();
        monthly_expenses.get();
        rent.get();
        home_value.get();
        mortgage.get();
        mortgage_rate.get();
        mortgage_term.get();
        min_retirement_income.get();
        max_retirement_income.get();
        owner_savings_derived.get();
        renter_savings_derived.get();
        inflation_rates.get();
        interest_rates.get();
        width.get();
        height.get();
        plot_resource.refetch();
    });

    view! {
        <div id="container">
            <div id="plot-container">
                <div id="plot"></div>
            </div>
            <div id="container">
                <div>
                    <button on:click=move |_| {
                        let rates = new_rates();
                        set_interest_rates.set(rates.0);
                        set_inflation_rates.set(rates.1);
                    }>

                        "Randomize Rates"

                    </button>
                </div>
                <div>
                    <div>
                        <label for="Age">"Age"</label>
                        <select
                            id="Age"
                            on:change=move |ev| {
                                set_age.set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=age_range/>
                        </select>
                    </div>
                    <div>
                        <label for="Retirement Age">"Retirement Age"</label>
                        <select
                            id="Retirement Age"
                            on:change=move |ev| {
                                set_retirement_age
                                    .set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=age_range/>
                        </select>
                    </div>
                    <div>
                        <label for="Net Worth">"Net Worth"</label>
                        <select
                            id="Net Worth"
                            on:change=move |ev| {
                                set_networth.set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=networth_range/>
                        </select>
                    </div>
                    <div>
                        <label for="Monthly Income">"Monthly Income"</label>
                        <select
                            id="Monthly Income"
                            on:change=move |ev| {
                                set_monthly_income
                                    .set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=incexp_range/>
                        </select>
                    </div>
                    <div>
                        <label for="Monthly Expenses">"Monthly Expenses"</label>
                        <select
                            id="Monthly Expenses"
                            on:change=move |ev| {
                                set_monthly_expenses
                                    .set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=incexp_range/>
                        </select>
                    </div>
                    <div>
                        <label for="Rent">"Rent"</label>
                        <select
                            id="Rent"
                            on:change=move |ev| {
                                set_rent.set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=incexp_range/>
                        </select>
                    </div>
                    <div>
                        <label for="Home Value">"Home Value"</label>
                        <select
                            id="Home Value"
                            on:change=move |ev| {
                                set_home_value
                                    .set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=networth_range/>
                        </select>
                    </div>
                    <div>
                        <label for="Mortgage">"Mortgage"</label>
                        <select
                            id="Mortgage"
                            on:change=move |ev| {
                                set_mortgage.set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=networth_range/>
                        </select>
                    </div>
                    <div>
                        <label for="Mortgage Rate">"Mortgage Rate"</label>
                        <select
                            id="Mortgage Rate"
                            on:change=move |ev| {
                                set_mortgage_rate
                                    .set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=mortgage_range/>
                        </select>
                    </div>
                    <div>
                        <label for="Mortgage Term">"Mortgage Term"</label>
                        <select
                            id="Mortgage Term"
                            on:change=move |ev| {
                                set_mortgage_term
                                    .set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=age_range/>
                        </select>
                    </div>
                    <div>
                        <label for="Min Monthly Retirement Income">
                            "Min Monthly Retirement Income"
                        </label>
                        <select
                            id="Min Monthly Retirement Income"
                            on:change=move |ev| {
                                set_min_retirement_income
                                    .set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=incexp_range/>
                        </select>
                    </div>
                    <div>
                        <label for="Max Monthly Retirement Income">
                            "Max Monthly Retirement Income"
                        </label>
                        <select
                            id="Max Monthly Retirement Income"
                            on:change=move |ev| {
                                set_max_retirement_income
                                    .set(Opts::from_str(&event_target_value(&ev)).unwrap())
                            }
                        >
                            <SelectOpts options=incexp_range/>
                        </select>
                    </div>
                </div>
            </div>
        </div>
    }
}
