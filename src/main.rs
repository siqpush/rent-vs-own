mod calculate;
use crate::calculate::consts::*;
use crate::calculate::rates::new_rates;
use crate::calculate::saver::{Saver, SaverType};

use leptos::view;
use leptos::*;
use leptos::{component, create_signal, CollectView, IntoView, SignalGet};
use leptos_use::utils::Pausable;
use leptos_use::*;
use num_format::{Locale, ToFormattedString};
use plotly::color::NamedColor;
use plotly::common::{Title, Font};
use plotly::layout::Axis;
use plotly::Plot;
use plotly::Scatter;

#[derive(Clone, Debug)]
pub struct OptionMeta {
    pub numtype: OptType,
    pub name: String,
    pub info: String,
    pub default_val: Opts,
    pub optarr: &'static [Opts; 100],
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
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

    leptos::window_event_listener(ev::resize, move |_| {
        fetch_width();
        fetch_height();
    });
    let (expand_methodology, set_expand_methodology) = create_signal(false);
    let (pause_resume, set_pause_resume) = create_signal(false);
    let (interval, _) = create_signal(1000_u64);

    let (age, set_age) = create_signal(Opts::Int(40));
    let age_opts = || OptionMeta {
        numtype: OptType::Int,
        name: "Age".to_string(),
        info: "this is your current age".to_string(),
        default_val: Opts::Int(45),
        optarr: &AGE_RANGE,
    };
    let (networth, set_networth) = create_signal(Opts::Float(200000.0));
    let networth_opts = || OptionMeta {
        numtype: OptType::Float,
        name: "Net Worth".to_string(),
        info: "this is your current net worth (savings + home value)".to_string(),
        default_val: Opts::Float(100000.0),
        optarr: &NETWORTH_RANGE,
    };
    let (retirement_age, set_retirement_age) = create_signal(Opts::Int(65));
    let retirement_age_opts = || OptionMeta {
        numtype: OptType::Int,
        name: "Retirement Age".to_string(),
        info: "this is the age you want to retire at".to_string(),
        default_val: Opts::Int(65),
        optarr: &AGE_RANGE,
    };
    let (monthly_income, set_monthly_income) = create_signal(Opts::Float(6000.0));
    let monthly_income_opts = || OptionMeta {
        numtype: OptType::Float,
        name: "Monthly Income".to_string(),
        info: "this is your current monthly income".to_string(),
        default_val: Opts::Float(5000.0),
        optarr: &INCEXP_RANGE,
    };
    let (monthly_expenses, set_monthly_expenses) = create_signal(Opts::Float(3000.0));
    let monthly_expenses_opts = || OptionMeta {
        numtype: OptType::Float,
        name: "Monthly Expenses".to_string(),
        info:
            "this is your current monthly expenses NOT including rent or mortgage related expenses"
                .to_string(),
        default_val: Opts::Float(3000.0),
        optarr: &INCEXP_RANGE,
    };
    let (rent, set_rent) = create_signal(Opts::Float(3000.0));
    let rent_opts = || {
        OptionMeta {
            numtype: OptType::Float,
            name: "Rent".to_string(),
            info: "this is your current monthly rent or amount you wish to compare against your home value".to_string(),
            default_val: Opts::Float(1000.0),
            optarr: &INCEXP_RANGE,
        }
    };
    let (home_value, set_home_value) = create_signal(Opts::Float(1000000.0));
    let home_value_opts = || {
        OptionMeta {
            numtype: OptType::Float,
            name: "Home Value".to_string(),
            info: "this is the current value of your home or the value of a home you wish to compare against your rent".to_string(),
            default_val: Opts::Float(200000.0),
            optarr: &NETWORTH_RANGE,
        }
    };
    let (mortgage, set_mortgage) = create_signal(Opts::Float(500000.0));
    let mortgage_opts = || OptionMeta {
        numtype: OptType::Float,
        name: "Mortgage".to_string(),
        info: "current/total mortgage amount on home".to_string(),
        default_val: Opts::Float(500000.0),
        optarr: &NETWORTH_RANGE,
    };
    let (mortgage_rate, set_mortgage_rate) = create_signal(Opts::Float(0.03));
    let mortgage_rate_opts = || OptionMeta {
        numtype: OptType::Float,
        name: "Mortgage Rate".to_string(),
        info: "current/total mortgage rate".to_string(),
        default_val: Opts::Float(0.03),
        optarr: &MORTGAGE_RATES,
    };
    let (mortgage_term, set_mortgage_term) = create_signal(Opts::Int(30));
    let mortgage_term_opts = || OptionMeta {
        numtype: OptType::Int,
        name: "Mortgage Term".to_string(),
        info: "duration of your mortgage in years".to_string(),
        default_val: Opts::Int(30),
        optarr: &AGE_RANGE,
    };
    let (min_retirement_income, set_min_retirement_income) = create_signal(Opts::Float(3000.0));
    let min_retirement_income_opts = || {
        OptionMeta {
            numtype: OptType::Float,
            name: "Min Monthly Retirement Income".to_string(),
            info: "this is the minimum amount of monthly income you want to have in retirement (in today's dollars)".to_string(),
            default_val: Opts::Float(2000.0),
            optarr: &INCEXP_RANGE,
        }
    };
    let (max_retirement_income, set_max_retirement_income) = create_signal(Opts::Float(5000.0));
    let max_retirement_income_opts = || {
        OptionMeta {
            numtype: OptType::Float,
            name: "Max Monthly Retirement Income".to_string(),
            info: "this is the maximum amount of monthly income you want to have in retirement (in today's dollars)".to_string(),
            default_val: Opts::Float(4000.0),
            optarr: &INCEXP_RANGE,
        }
    };

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

    let savers_derived = create_memo(move |_| {
        set_renter_savings_arr.set(renter_savings());
        set_owner_savings_arr.set(owner_savings());
    });

    let plot_resource = create_local_resource(
        move || (width, height),
        move |_| async move {
            let mut plot = Plot::new();
            let start_x_value = age.get_untracked().get_int() as usize;
            let x_values = &AGE_RANGE_FLOATS[start_x_value..DEATH];

            let y_axis = || {
                let owner_max = owner_savings_arr.with_untracked(|owner_savings_arr| {
                    *owner_savings_arr[start_x_value..DEATH]
                        .iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                });

                let renter_max = renter_savings_arr.with_untracked(|renter_savings_arr| {
                    *renter_savings_arr[start_x_value..DEATH]
                        .iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                });

                Axis::new()
                    .title("Savings".into())
                    .range(vec![
                        0.0,
                        if owner_max > renter_max {
                            YAXIS_BUCKETS
                                .iter()
                                .find(|x| x > &&owner_max)
                                .unwrap_or(&10000000000.0)
                        } else {
                            YAXIS_BUCKETS
                                .iter()
                                .find(|x| x > &&renter_max)
                                .unwrap_or(&10000000000.0)
                        }
                        .to_owned(),
                    ])
                    .auto_range(false)
            };

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
                    .x_axis(Axis::new().title("Age".into()).range(x_values.to_vec()))
                    .y_axis(y_axis())
                    .auto_size(true)
            };

            let mut sizing = || {
                if let (Some(width), Some(height)) = (
                    width.get_untracked().map(|width| width * 0.75),
                    height.get_untracked().map(|height| height * 0.5),
                ) {
                    plot.set_layout(
                        plotly::Layout::new()
                            .font(Font::new().family("Courier New, monospace"))
                            .title(Title::new("Renting vs Owning"))
                            .show_legend(true)
                            .width(width as usize)
                            .height(height as usize)
                            .x_axis(Axis::new().title("Age".into()).range(x_values.to_vec()))
                            .y_axis(y_axis()),
                    );
                } else {
                    plot.set_layout(err_sizing())
                }
            };

            sizing();

            //plot.set_layout(layout);
            plotly::bindings::new_plot("plot", &plot).await;
        },
    );

    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn_with_options(
        move || {
            let rates = new_rates();
            set_interest_rates.update(|i| *i = rates.0);
            set_inflation_rates.update(|i| *i = rates.1);
            set_owner_savings_arr.update(|i| *i = owner_savings());
            set_renter_savings_arr.update(|i| *i = renter_savings());
        },
        interval,
        UseIntervalFnOptions {
            immediate: true,
            immediate_callback: true,
        },
    );

    create_effect(move |_| {
        expand_methodology.get();
        if pause_resume.get() && is_active.get() {
            pause();
        } else {
            resume();
        }
    });

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
        savers_derived.get();
        inflation_rates.get();
        interest_rates.get();
        width.get();
        height.get();
        plot_resource.refetch();
    });

    view! {
        <div id="container">
            <div id="plot-container">
                <div id="plot-container-top-row">
                    <div id="plot-container-chart">
                        <div id="plot"></div>
                        
                    </div>
                    <div id="plot-container-methodology">
                        <Show when={move || expand_methodology.get()}>
                                <h3>Methodology</h3>
                                <p>
                                    "This calculator compares the savings of a renter vs a home owner.
                                    A Monte Carlo simulation is used to calculate stock market returns
                                    and inflation rates. The simulation run 1000 times would produce 
                                    interest rates starting close to the following: 6.25% and falls to
                                    4.5% as you age. Additionally the std deviation of the interest rates
                                    also decreases as you age to assume less risk is introduced the less
                                    time you have to recover from a market crash. Both interest and inflation
                                    are compounded monthly using an annual interest rate / 12.0. Inflation is 
                                    impacts rent, home expenses (1% annually), monthly expenses, and monthly income.
                                    Interest is only applied to liquid assets (home value is not interest bearing).
                                    We assume you continue to live in the same home for the duration of the simulation.
                                    "
                                </p>
                        </Show>
                    </div>
                </div>
                <div id="plot-container-action-button">
                    <button on:click=move |_| {
                            set_pause_resume.set(!pause_resume.get());
                    }
                    >
                    {move || { if pause_resume.get() { "Resume" } else { "Pause" } }}
                    </button>
                    <button id="methodology-button" on:click=move |_| {
                        set_pause_resume.set(!pause_resume.get());
                        set_expand_methodology.set(!expand_methodology.get());
                    }>
                        {move || {
                            if expand_methodology.get() { 
                                "Hide Methodology"
                            } else { 
                                "Show Methodology"
                            }
                        }}
                    </button>
                </div>
            </div>
            <div id="opts-container">
                <DisplayOptions set_val=set_age fn_meta=age_opts/>
                <DisplayOptions set_val=set_networth fn_meta=networth_opts/>
                <DisplayOptions set_val=set_retirement_age fn_meta=retirement_age_opts/>
                <DisplayOptions set_val=set_monthly_income fn_meta=monthly_income_opts/>
                <DisplayOptions set_val=set_monthly_expenses fn_meta=monthly_expenses_opts/>
                <DisplayOptions set_val=set_rent fn_meta=rent_opts/>
                <DisplayOptions set_val=set_home_value fn_meta=home_value_opts/>
                <DisplayOptions set_val=set_mortgage fn_meta=mortgage_opts/>
                <DisplayOptions set_val=set_mortgage_rate fn_meta=mortgage_rate_opts/>
                <DisplayOptions set_val=set_mortgage_term fn_meta=mortgage_term_opts/>
                <DisplayOptions
                    set_val=set_min_retirement_income
                    fn_meta=min_retirement_income_opts
                />
                <DisplayOptions
                    set_val=set_max_retirement_income
                    fn_meta=max_retirement_income_opts
                />
            </div>
        </div>
    }
}

#[component]
fn DisplayOptions<FnMeta>(set_val: WriteSignal<Opts>, fn_meta: FnMeta) -> impl IntoView
where
    FnMeta: Fn() -> OptionMeta + 'static,
{
    let OptionMeta {
        numtype,
        name,
        info,
        default_val,
        optarr,
    } = fn_meta();
    view! {
        <div id="select-container">
            <div id="select-container-options">
                <label for=name.clone()>{name.clone()}</label>
                <select id=name.clone() on:change=move |ev| {
                    match numtype {
                        OptType::Int => set_val.set(Opts::opt_from_u8_str(&event_target_value(&ev))),
                        OptType::Float => {
                            set_val.set(Opts::opt_from_f32_str(&event_target_value(&ev)))
                        }
                    }
                }>

                    <SelectOpts options=move || (optarr, default_val)/>
                </select>
            </div>
            <div id="select-container-footnotes">
                <p>{info.clone()}</p>
            </div>
        </div>
    }
}

#[component]
fn SelectOpts<FnDefaults>(options: FnDefaults) -> impl IntoView
where
    FnDefaults: Fn() -> (&'static [Opts; 100], Opts) + 'static,
{
    move || {
        let (options, default_val) = options();
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
        (*options)
            .into_iter()
            .map(|opt| match opt {
                Opts::Int(opt) => {
                    if opt == default_val.get_int() {
                        view! {
                            <option selected="selected" value=opt>
                                {opt}
                            </option>
                        }
                    } else {
                        view! { <option value=opt>{opt}</option> }
                    }
                }
                Opts::Float(opt) => {
                    if opt == default_val.get_float() {
                        view! {
                            <option selected="selected" value=opt>

                                {if format_percent {
                                    let mut val = ((opt * 1000.0).trunc() as i32)
                                        .to_formatted_string(&Locale::en) + "%";
                                    val.insert(val.len() - 2, '.');
                                    if val == ".0%" {
                                        val = "0.0%".to_string();
                                    }
                                    val
                                } else {
                                    let mut val = (opt.trunc() as i32)
                                        .to_formatted_string(&Locale::en);
                                    if val == ".0" {
                                        val = "0".to_string();
                                    }
                                    val
                                }}

                            </option>
                        }
                    } else {
                        view! {
                            <option value=opt>

                                {if format_percent {
                                    let mut val = ((opt * 1000.0).trunc() as i32)
                                        .to_formatted_string(&Locale::en) + "%";
                                    val.insert(val.len() - 2, '.');
                                    if val == ".0%" {
                                        val = "0.0%".to_string();
                                    }
                                    val
                                } else {
                                    let mut val = (opt.trunc() as i32)
                                        .to_formatted_string(&Locale::en);
                                    if val == ".0" {
                                        val = "0".to_string();
                                    }
                                    val
                                }}

                            </option>
                        }
                    }
                }
            })
            .collect_view()
    }
}
