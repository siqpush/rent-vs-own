mod calculate;

use crate::calculate::consts::*;
use crate::calculate::rates::new_rates;
use crate::calculate::saver::{Saver, SaverType};

use leptos::*;
use leptos_use::utils::Pausable;
use leptos_use::*;
use num_format::{Locale, ToFormattedString};
use plotly::color::NamedColor;
use plotly::common::{Anchor, DashType, Font, Line, Title};
use plotly::layout::{Axis, Margin};
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
            Err(_) => {
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
            Err(_) => {
                set_height.set(None);
            }
        };
    };

    leptos::window_event_listener(ev::resize, move |_| {
        fetch_width();
        fetch_height();
    });

    let (expand_methodology, set_expand_methodology) = create_signal(false);
    let (expand_y_axis_settings, set_expand_y_axis_settings) = create_signal(false);
    let (pause_resume, set_pause_resume) = create_signal(false);
    let (find_equivelent_rent, set_find_equivelent_rent) = create_signal(false);

    let default_age = 30;
    let (age, set_age) = create_signal(Opts::Int(default_age));
    let age_opts = move || OptionMeta {
        numtype: OptType::Int,
        name: "Age".to_string(),
        info: "this is your current age".to_string(),
        default_val: Opts::Int(default_age),
        optarr: &AGE_RANGE,
    };

    let (networth, set_networth) = create_signal(Opts::Float(200000.0));
    let networth_opts = move || OptionMeta {
        numtype: OptType::Float,
        name: "Net Worth".to_string(),
        info: "this is your current net worth (*include home principle)".to_string(),
        default_val: Opts::Float(200000.0),
        optarr: &NETWORTH_RANGE,
    };

    let default_retirement_age = 65;
    let (retirement_age, set_retirement_age) = create_signal(Opts::Int(default_retirement_age));
    let retirement_age_opts = move || OptionMeta {
        numtype: OptType::Int,
        name: "Retirement Age".to_string(),
        info: "this is the age you want to retire at".to_string(),
        default_val: Opts::Int(default_retirement_age),
        optarr: &AGE_RANGE,
    };

    let default_monthly_income = 6000.0;
    let (monthly_income, set_monthly_income) = create_signal(Opts::Float(default_monthly_income));
    let monthly_income_opts = move || OptionMeta {
        numtype: OptType::Float,
        name: "Monthly Income".to_string(),
        info: "this is your current monthly income".to_string(),
        default_val: Opts::Float(default_monthly_income),
        optarr: &INCEXP_RANGE,
    };

    let default_monthly_expenses = 5000.0;
    let (monthly_expenses, set_monthly_expenses) = create_signal(Opts::Float(default_monthly_expenses));
    let monthly_expenses_opts = move || OptionMeta {
        numtype: OptType::Float,
        name: "Monthly Expenses".to_string(),
        info:
            "this is your current monthly expenses NOT including rent or mortgage related expenses"
                .to_string(),
        default_val: Opts::Float(default_monthly_expenses),
        optarr: &INCEXP_RANGE,
    };

    let default_rent = 2000.0;
    let (rent, set_rent) = create_signal(Opts::Float(default_rent));
    let rent_opts = move || {
        OptionMeta {
            numtype: OptType::Float,
            name: "Rent".to_string(),
            info: "this is your current monthly rent or amount you wish to compare against your home value".to_string(),
            default_val: Opts::Float(default_rent),
            optarr: &INCEXP_RANGE,
        }
    };

    let default_home_value = 500000.0;
    let (home_value, set_home_value) = create_signal(Opts::Float(default_home_value));
    let home_value_opts = move || {
        OptionMeta {
            numtype: OptType::Float,
            name: "Home Value".to_string(),
            info: "this is the current value of your home or the value of a home you wish to compare against your rent".to_string(),
            default_val: Opts::Float(default_home_value),
            optarr: &NETWORTH_RANGE,
        }
    };

    let default_mortgage = 400000.0;
    let (mortgage, set_mortgage) = create_signal(Opts::Float(default_mortgage));
    let mortgage_opts = move || OptionMeta {
        numtype: OptType::Float,
        name: "Mortgage".to_string(),
        info: "current/total mortgage amount on home".to_string(),
        default_val: Opts::Float(default_mortgage),
        optarr: &NETWORTH_RANGE,
    };

    let default_mortgage_rate = 0.05;
    let (mortgage_rate, set_mortgage_rate) = create_signal(Opts::Float(default_mortgage_rate));
    let mortgage_rate_opts = move || OptionMeta {
        numtype: OptType::Float,
        name: "Mortgage Rate".to_string(),
        info: "current/total mortgage rate".to_string(),
        default_val: Opts::Float(default_mortgage_rate),
        optarr: &MORTGAGE_RATES,
    };

    let default_mortgage_term = 30;
    let (mortgage_term, set_mortgage_term) = create_signal(Opts::Int(default_mortgage_term));
    let mortgage_term_opts = move || OptionMeta {
        numtype: OptType::Int,
        name: "Mortgage Term".to_string(),
        info: "duration of your mortgage in years".to_string(),
        default_val: Opts::Int(default_mortgage_term),
        optarr: &AGE_RANGE,
    };

    let default_min_retirement_income = 2000.0;
    let (min_retirement_income, set_min_retirement_income) = create_signal(Opts::Float(default_min_retirement_income));
    let min_retirement_income_opts = move || {
        OptionMeta {
            numtype: OptType::Float,
            name: "Min Monthly Retirement Income".to_string(),
            info: "this is the minimum amount of monthly income you want to have in retirement (in today's dollars)".to_string(),
            default_val: Opts::Float(default_min_retirement_income),
            optarr: &INCEXP_RANGE,
        }
    };
    let default_max_retirement_income = 3000.0;
    let (max_retirement_income, set_max_retirement_income) = create_signal(Opts::Float(default_max_retirement_income));
    let max_retirement_income_opts = move || {
        OptionMeta {
            numtype: OptType::Float,
            name: "Max Monthly Retirement Income".to_string(),
            info: "this is the maximum amount of monthly income you want to have in retirement (in today's dollars)".to_string(),
            default_val: Opts::Float(default_max_retirement_income),
            optarr: &INCEXP_RANGE,
        }
    };

    let (inflation_rates, set_inflation_rates) = create_signal(vec![Opts::Float(0.0); 100]);
    let (interest_rates, set_interest_rates) = create_signal(vec![Opts::Float(0.0); 100]);

    let (owner_savings_arr, set_owner_savings_arr) = create_signal(vec![0.0; 100]);
    let (renter_savings_arr, set_renter_savings_arr) = create_signal(vec![0.0; 100]);

    let (equivelent_rent, set_equivelent_rent) = create_signal("".to_string());

    let owner_savings = move || {
        Saver {
            monthly_rent: 0.0,
            current_age: age.get_untracked().get_int(),
            retirement_age: retirement_age.get_untracked().get_int(),
            total_savings: networth.get_untracked().get_float(),
            monthly_income: monthly_income.get_untracked().get_float(),
            monthly_expenses: monthly_expenses.get_untracked().get_float(),
            home_value: home_value.get_untracked().get_float(),
            mortgage_debt: mortgage.get_untracked().get_float(),
            mortgage_rate: mortgage_rate.get_untracked().get_float(),
            mortgage_term: mortgage_term.get_untracked().get_int(),
            min_baseline_retirement_income: min_retirement_income.get_untracked().get_float(),
            max_baseline_retirement_income: max_retirement_income.get_untracked().get_float(),
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
            monthly_rent: rent.get_untracked().get_float(),
            current_age: age.get_untracked().get_int(),
            retirement_age: retirement_age.get_untracked().get_int(),
            total_savings: networth.get_untracked().get_float(),
            monthly_income: monthly_income.get_untracked().get_float(),
            monthly_expenses: monthly_expenses.get_untracked().get_float(),
            home_value: 0.0,
            mortgage_debt: 0.0,
            mortgage_rate: 0.0,
            mortgage_term: 0,
            min_baseline_retirement_income: min_retirement_income.get_untracked().get_float(),
            max_baseline_retirement_income: max_retirement_income.get_untracked().get_float(),
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

    let calculate_renter_equivelence = move || {

        let owner_saved = owner_savings_arr.with_untracked(|owner_saved| *owner_saved.last().unwrap_or(&0.0));
        let last_age_owner_saved = owner_savings_arr.with_untracked(|owner_saved| 
            owner_saved.iter().enumerate().find(|(_, x)| **x >= 0.0).map(|(i, _)| i).unwrap_or(0)
        );

        let adjust_rent_directionally = |rent: &f32, sign: &f32| {
            rent + (0.1 * sign)
        };

        let adjust_rent = |rent: &f32, savings: &f32, owner_savings: &f32| {
            if f32::abs(savings - owner_savings) < 100.0 || *rent < 100.0 {
                (*rent, false)
            } else if savings < owner_savings {
                (adjust_rent_directionally(rent, &1.0), true)
            } else {
                (adjust_rent_directionally(rent, &-1.0), true)
            }
        };

        let mut rent = rent.with_untracked(|rent| rent.get_float());
        let current_age = age.with_untracked(|age| age.get_int());
        let retirement_age = retirement_age.with_untracked(|retirement_age| retirement_age.get_int());
        let networth = networth.with_untracked(|networth| networth.get_float());
        let monthly_income = monthly_income.with_untracked(|monthly_income| monthly_income.get_float());
        let monthly_expenses = monthly_expenses.with_untracked(|monthly_expenses| monthly_expenses.get_float());
        let _home_value = home_value.with_untracked(|home_value| home_value.get_float());
        let _mortgage = mortgage.with_untracked(|mortgage| mortgage.get_float());
        let _mortgage_rate = mortgage_rate.with_untracked(|mortgage_rate| mortgage_rate.get_float());
        let _mortgage_term = mortgage_term.with_untracked(|mortgage_term| mortgage_term.get_int());
        let min_retirement_income = min_retirement_income.with_untracked(|min_retirement_income| min_retirement_income.get_float());
        let max_retirement_income = max_retirement_income.with_untracked(|max_retirement_income| max_retirement_income.get_float());
        let mut rs = vec![0.0; DEATH];
        let mut continue_adj = true;
        for _ in 0..1000 {
            rs = Saver {
                monthly_rent: rent,
                current_age,
                retirement_age,
                total_savings: networth,
                monthly_income,
                monthly_expenses,
                home_value: 0.0,
                mortgage_debt: 0.0,
                mortgage_rate: 0.0,
                mortgage_term: 0,
                min_baseline_retirement_income: min_retirement_income,
                max_baseline_retirement_income: max_retirement_income,
                home_expenses: 0.0,
                home_savings: vec![0.0; DEATH],
                rental_savings: vec![0.0; DEATH],
                active_retirement: false,
                home_owned_age: None::<u8>,
                cached_mortgage_installment: None::<f32>,
                interest_rates,
                inflation_rates,
            }.calculate_savings(SaverType::Renter, DEATH as u8);

            (rent, continue_adj) = adjust_rent(&rent, &owner_saved, &rs[last_age_owner_saved]);

            if !continue_adj {
                break;
            }
        }
        set_equivelent_rent.set(format!(
            "Home:{}\nRent: {}", 
            (home_value.get().get_float().trunc() as i32).to_formatted_string(&Locale::en),
            (rent.trunc() as i32).to_formatted_string(&Locale::en)
        ));
        rs
    };

    let default_y_axis_max = 10000000.0;
    let (y_axis_max, set_y_axis_max) = create_signal(Opts::Float(default_y_axis_max));
    let y_axis_opts = move || OptionMeta {
        numtype: OptType::Float,
        name: "YAxis Max".to_string(),
        info: "this is the max value of the y axis".to_string(),
        default_val: Opts::Float(default_y_axis_max),
        optarr: &YAXIS_BUCKETS,
    };

    // plotly chart showing savings
    let plot_resource = create_local_resource(
        move || (width, height),
        move |_| async move {
            let mut plot = Plot::new();

            // x axis data / format
            let start_x_value = age.get_untracked().get_int() as usize;
            let x_values = &AGE_RANGE_FLOATS[start_x_value..DEATH];
            let x_axis = || Axis::new().title("Age".into());
            let y_axis = || {
                Axis::new()
                    .title("Savings".into())
                    .auto_range(false)
                    .range(vec![0.0, y_axis_max.get_untracked().get_float()])
            };

            let annotations = || {
                // function to calculate avg returns for annotation
                let avg_returns = |start: usize, stop: usize| {
                    interest_rates.with_untracked(|interest_rates| {
                        interest_rates[start..stop]
                            .iter()
                            .map(|x| x.get_float_ref())
                            .sum::<f32>()
                            / (stop - start) as f32
                    })
                };

                // function to calculate std dev for annotation
                let std_dev = |start: usize, stop: usize| {
                    interest_rates.with_untracked(|interest_rates| {
                        interest_rates[start..stop]
                            .iter()
                            .map(|x| x.get_float_ref())
                            .fold(0.0, |acc, x| acc + (x - avg_returns(start, stop)).powi(2))
                            .sqrt()
                            / (stop - start) as f32
                    })
                };

                let x_pos = |_: usize, stop: usize| {
                    (stop as f32 - start_x_value as f32) / (DEATH - start_x_value) as f32 - 0.05
                };

                // adding annotations for avg return
                let avg_return_annotate = || {
                    let offset = 0.225;
                    [
                        plotly::layout::Annotation::new()
                            .text(format!("{:.1}%", avg_returns(0, 35) * 100.0,))
                            .x_ref("paper")
                            .x(0.05)
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::DarkSeaGreen)),
                        plotly::layout::Annotation::new()
                            .text(format!("{:.1}%", avg_returns(36, 49) * 100.0,))
                            .x_ref("paper")
                            .x(x_pos(36, 49))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::DarkSeaGreen)),
                        plotly::layout::Annotation::new()
                            .text(format!("{:.1}%", avg_returns(50, 64) * 100.0,))
                            .x_ref("paper")
                            .x(x_pos(50, 64))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::DarkSeaGreen)),
                        plotly::layout::Annotation::new()
                            .text(format!("{:.1}%", avg_returns(65, 80) * 100.0,))
                            .x_ref("paper")
                            .x(x_pos(65, 80))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::DarkSeaGreen)),
                        plotly::layout::Annotation::new()
                            .text(format!("{:.1}%", avg_returns(81, DEATH) * 100.0,))
                            .x_ref("paper")
                            .x(x_pos(81, DEATH))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::DarkSeaGreen)),
                        plotly::layout::Annotation::new()
                            .text("ROI")
                            .x_ref("paper")
                            .x(-0.05)
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(0.0)
                            .background_color(NamedColor::DarkSeaGreen)
                            .opacity(0.5)
                            .font(Font::new().size(10).color(NamedColor::FloralWhite)),
                    ]
                };

                // adding annotations for std dev
                let std_dev_annotate = || {
                    let offset = 0.3;
                    [
                        plotly::layout::Annotation::new()
                            .text(format!("{:.1}%", std_dev(0, 35) * 100.0,))
                            .x_ref("paper")
                            .x(0.05)
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::LightSalmon)),
                        plotly::layout::Annotation::new()
                            .text(format!("{:.1}%", std_dev(36, 49) * 100.0,))
                            .x_ref("paper")
                            .x(x_pos(36, 49))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::LightSalmon)),
                        plotly::layout::Annotation::new()
                            .text(format!("{:.1}%", std_dev(50, 64) * 100.0,))
                            .x_ref("paper")
                            .x(x_pos(50, 64))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::LightSalmon)),
                        plotly::layout::Annotation::new()
                            .text(format!("{:.1}%", std_dev(65, 80) * 100.0,))
                            .x_ref("paper")
                            .x(x_pos(65, 80))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::LightSalmon)),
                        plotly::layout::Annotation::new()
                            .text(format!("{:.1}%", std_dev(81, DEATH) * 100.0,))
                            .x_ref("paper")
                            .x(x_pos(81, DEATH))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::LightSalmon)),
                        plotly::layout::Annotation::new()
                            .text("Beta")
                            .x_ref("paper")
                            .x(-0.05)
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(0.0)
                            .background_color(NamedColor::LightSalmon)
                            .opacity(0.5)
                            .font(Font::new().size(10).color(NamedColor::FloralWhite)),
                    ]
                };

                // adding annotations for std dev
                let age_annotate = || {
                    let offset = 0.15;
                    [
                        plotly::layout::Annotation::new()
                            .text(format!("{}-{}", start_x_value, 35))
                            .x_ref("paper")
                            .x(0.05)
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::BurlyWood)),
                        plotly::layout::Annotation::new()
                            .text(format!("{}-{}", 36, 49))
                            .x_ref("paper")
                            .x(x_pos(36, 49))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::BurlyWood)),
                        plotly::layout::Annotation::new()
                            .text(format!("{}-{}", 50, 64))
                            .x_ref("paper")
                            .x(x_pos(50, 64))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::BurlyWood)),
                        plotly::layout::Annotation::new()
                            .text(format!("{}-{}", 65, 80))
                            .x_ref("paper")
                            .x(x_pos(65, 80))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::BurlyWood)),
                        plotly::layout::Annotation::new()
                            .text(format!("{}-{}", 81, DEATH))
                            .x_ref("paper")
                            .x(x_pos(81, DEATH))
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(27.0)
                            .font(Font::new().size(10).color(NamedColor::BurlyWood)),
                        plotly::layout::Annotation::new()
                            .text("Age")
                            .x_ref("paper")
                            .x(-0.05)
                            .y_ref("paper")
                            .y(0.0 - offset)
                            .show_arrow(false)
                            .text_angle(0.0)
                            .background_color(NamedColor::BurlyWood)
                            .opacity(0.5)
                            .font(Font::new().size(10).color(NamedColor::FloralWhite)),
                    ]
                };
                let trace_annotations = || {
                    [
                        plotly::layout::Annotation::new()
                            .text("Owner")
                            .x_ref("paper")
                            .x(0.9)
                            .y_ref("paper")
                            .y(0.9)
                            .show_arrow(false)
                            .text_angle(0.0)
                            .font(Font::new().size(12).color(NamedColor::DarkSeaGreen)),
                        plotly::layout::Annotation::new()
                            .text("Renter")
                            .x_ref("paper")
                            .x(0.9)
                            .y_ref("paper")
                            .y(0.85)
                            .show_arrow(false)
                            .text_angle(0.0)
                            .font(Font::new().size(12).color(NamedColor::LightSalmon)),
                        plotly::layout::Annotation::new()
                            .text(equivelent_rent.get())
                            .x_ref("paper")
                            .x(0.15)
                            .y_ref("paper")
                            .y(0.15)
                            .show_arrow(false)
                            .text_angle(0.0)
                            .font(Font::new().size(10).color(NamedColor::IndianRed)),
                    ]
                };

                let mut annotation_vec = vec![];
                match age.get_untracked().get_int() {
                    81.. => {
                        annotation_vec.extend_from_slice(&age_annotate()[4..]);
                        annotation_vec.extend_from_slice(&avg_return_annotate()[4..]);
                        annotation_vec.extend_from_slice(&std_dev_annotate()[4..]);
                    }
                    65..=80 => {
                        annotation_vec.extend_from_slice(&age_annotate()[3..]);
                        annotation_vec.extend_from_slice(&avg_return_annotate()[3..]);
                        annotation_vec.extend_from_slice(&std_dev_annotate()[3..]);
                    }
                    50..=64 => {
                        annotation_vec.extend_from_slice(&age_annotate()[2..]);
                        annotation_vec.extend_from_slice(&avg_return_annotate()[2..]);
                        annotation_vec.extend_from_slice(&std_dev_annotate()[2..]);
                    }
                    36..=49 => {
                        annotation_vec.extend_from_slice(&age_annotate()[1..]);
                        annotation_vec.extend_from_slice(&avg_return_annotate()[1..]);
                        annotation_vec.extend_from_slice(&std_dev_annotate()[1..]);
                    }
                    0..=35 => {
                        annotation_vec.extend_from_slice(&age_annotate()[0..]);
                        annotation_vec.extend_from_slice(&avg_return_annotate()[0..]);
                        annotation_vec.extend_from_slice(&std_dev_annotate()[0..]);
                    }
                }
                annotation_vec.extend_from_slice(&trace_annotations());

                annotation_vec
            };

            // savings values to plot (owner and renter)
            let traces = || {
                let owner_trace = Scatter::new(
                    x_values.to_vec(),
                    owner_savings_arr.get_untracked()[start_x_value..DEATH].to_vec(),
                )
                .visible(plotly::common::Visible::True)
                .fill_color(NamedColor::DarkSeaGreen)
                .line(
                    Line::new()
                        .dash(DashType::Solid)
                        .color(NamedColor::DarkSeaGreen),
                )
                .name("Owner");

                let renter_trace = Scatter::new(
                    x_values.to_vec(),
                    renter_savings_arr.get_untracked()[start_x_value..DEATH].to_vec(),
                )
                .visible(plotly::common::Visible::True)
                .fill_color(NamedColor::LightSalmon)
                .line(
                    Line::new()
                        .dash(DashType::Dash)
                        .color(NamedColor::LightSalmon),
                )
                .name("Renter");

                (owner_trace, renter_trace)
            };

            let apply_layout = move || async move {

                let traces = traces();
                plot.add_trace(traces.0);
                plot.add_trace(traces.1);

                let general_layout = || {
                    plotly::Layout::new()
                        .font(Font::new().family("Courier New, monospace"))
                        .title(Title::new("Renting vs Owning").x_anchor(Anchor::Right))
                        .annotations(annotations())
                        .x_axis(x_axis())
                        .y_axis(y_axis())
                        .show_legend(false)
                };

                if let (Some(width), Some(height)) = (
                    width.get().map(|width| width * 1.0),
                    height.get().map(|height| height * 0.5),
                ) {
                    plot.set_layout(
                        general_layout()
                            .width(width as usize)
                            .height(height as usize)
                            .margin(Margin::new().bottom(100)),
                    );
                } else {
                    plot.set_layout(
                        general_layout().auto_size(true)
                        .margin(Margin::new().bottom(100))
                    );
                }
                plotly::bindings::new_plot("plot", &plot).await;
            };

            apply_layout().await;
            
        },
    );

    let savers_derived = create_memo(move |_| {
        set_owner_savings_arr.set(owner_savings());
        if find_equivelent_rent.get() {
            set_renter_savings_arr.set(calculate_renter_equivelence());
            plot_resource.refetch();
        } else {
            set_renter_savings_arr.set(renter_savings());
        }
    });

    let Pausable{
        pause,
        resume,
        is_active: _,
    } = use_interval_fn_with_options(
        move || {
            if !find_equivelent_rent.get() {
                let rates = new_rates();
                set_interest_rates.update(|i| *i = rates.0);
                set_inflation_rates.update(|i| *i = rates.1);
                set_owner_savings_arr.update(|i| *i = owner_savings());
                set_renter_savings_arr.update(|i| *i = renter_savings());
            }
        },
        2000_u64,
        UseIntervalFnOptions {
            immediate: true,
            immediate_callback: true,
        },
    );
    
    create_effect(move |_| {
        y_axis_max.get();
        expand_methodology.get();
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
        find_equivelent_rent.get();
        savers_derived.get();
        inflation_rates.get();
        interest_rates.get();
        plot_resource.refetch();
        if pause_resume.get() {
            pause();
        } else {
            resume();
        }
    });

    view! {
        <div id="container">
            <div id="plot-container">
                <div id="plot-container-chart">
                    <div id="plot"></div>
                </div>
                <div id="plot-container-action-button">
                    <button on:click=move |_| {
                        set_pause_resume.set(!pause_resume.get());
                        set_find_equivelent_rent.set(false);
                    }>

                        {move || { if pause_resume.get() { "Resume" } else { "Pause" } }}
                    </button>
                    <button
                        id="methodology-button"
                        on:click=move |_| {
                            set_expand_methodology.set(!expand_methodology.get());
                        }
                    >

                        {move || {
                            if expand_methodology.get() {
                                "Hide Methodology"
                            } else {
                                "Show Methodology"
                            }
                        }}

                    </button>
                    <button
                        id="y-axis-settings-button"
                        on:click=move |_| {
                            set_expand_y_axis_settings.set(!expand_y_axis_settings.get());
                        }
                    >

                        {move || {
                            if expand_y_axis_settings.get() {
                                "Hide Y Axis Settings"
                            } else {
                                "Show Y Axis Settings"
                            }
                        }}

                    </button>
                    <button on:click={move |_| {
                        set_pause_resume.set(true);
                        set_find_equivelent_rent.set(true);
                    }}>
                        "Find Equivelent Rent"
                    </button>
                </div>
            </div>
            <Show when=move || expand_y_axis_settings.get()>
                <div style="padding-left: 10%; padding-right: 10%; margin: 5%">
                    <DisplayOptions set_val=set_y_axis_max fn_meta=y_axis_opts/>
                </div>
            </Show>
            <Show when=move || expand_methodology.try_get().unwrap_or(true)>
                <div id="methodology-container">
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

                </div>
            </Show>
            <div id="opts-container">
                <DisplayOptions set_val=set_age fn_meta=age_opts/>
                <DisplayOptions set_val=set_networth fn_meta=networth_opts/>
                <DisplayOptions set_val=set_retirement_age fn_meta=retirement_age_opts/>
                <DisplayOptions set_val=set_monthly_income fn_meta=monthly_income_opts/>
                <DisplayOptions set_val=set_monthly_expenses fn_meta=monthly_expenses_opts/>
                <DisplayOptions set_val=set_rent fn_meta=rent_opts/>
                <DisplayOptions set_val=set_home_value fn_meta=home_value_opts/>
                <DisplayOptions set_val=set_mortgage fn_meta=mortgage_opts/>
                <Show when=move || mortgage.get().get_float_ref() != &0.0>
                    <DisplayOptions set_val=set_mortgage_rate fn_meta=mortgage_rate_opts/>
                    <DisplayOptions set_val=set_mortgage_term fn_meta=mortgage_term_opts/>
                </Show>
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
                <select
                    id=name.clone()
                    on:change=move |ev| {
                        match numtype {
                            OptType::Int => {
                                set_val.set(Opts::opt_from_u8_str(&event_target_value(&ev)))
                            }
                            OptType::Float => {
                                set_val.set(Opts::opt_from_f32_str(&event_target_value(&ev)))
                            }
                        }
                    }
                >

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