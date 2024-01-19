use leptos::{ReadSignal, SignalWith, SignalWithUntracked};
//use serde::{Deserialize, Serialize};
use super::{
    consts::Opts,
    owner::{self, Owner},
};
pub const STD_MONTHLY_WITHDRAWAL_RATE: f32 = 0.04 / 12.0;

#[derive(Clone)]
pub enum SaverType {
    HomeOwner,
    Renter,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]

#[derive(Clone)]
pub struct Saver {
    pub current_age: u8,
    pub retirement_age: u8,
    pub total_savings: f32,
    pub monthly_income: f32,
    pub monthly_expenses: f32,
    pub home_value: f32,
    pub monthly_rent: f32,
    pub mortgage_debt: f32,
    pub mortgage_rate: f32,
    pub mortgage_term: u8,
    pub min_baseline_retirement_income: f32,
    pub max_baseline_retirement_income: f32,
    pub interest_rates: ReadSignal<Vec<Opts>>,
    pub inflation_rates: ReadSignal<Vec<Opts>>,
    pub home_savings: Vec<f32>,
    pub rental_savings: Vec<f32>,
    pub active_retirement: bool,
    pub home_owned_age: Option<u8>,
    pub cached_mortgage_installment: Option<f32>,
    pub home_expenses: f32,
}

impl Saver {
    // monthly inflation rate
    pub fn monthly_inflation(&self) -> impl Fn() -> f32 + '_ {
        move || {
            self.inflation_rates.with_untracked(|inflation_rates| {
                inflation_rates[self.current_age as usize].get_float_ref() / 12.0
            })
        }
    }
    // monthly interest rate
    pub fn monthly_interest(&self) -> impl Fn() -> f32 + '_ {
        move || {
            self.interest_rates.with_untracked(|interest_rates| {
                interest_rates[self.current_age as usize].get_float_ref() / 12.0
            })
        }
    }

    // calculate liquid assets (total savings - (home value - mortgage debt))
    pub fn liquid_assets(&self) -> f32 {
        if self.total_savings - (self.home_value - self.mortgage_debt) < 0.0 {
            0.0
        } else {
            self.total_savings - (self.home_value - self.mortgage_debt)
        }
    }

    // calculate monthly withdrawal rate (4% of total savings or min/max baseline retirement income)
    pub fn monthly_withdrawal(&self) -> f32 {
        if self.liquid_assets() <= 0.0 {
            0.0
        } else if self.min_baseline_retirement_income
            <= self.liquid_assets() * STD_MONTHLY_WITHDRAWAL_RATE
            && self.max_baseline_retirement_income
                >= self.liquid_assets() * STD_MONTHLY_WITHDRAWAL_RATE
        {
            -1.0 * STD_MONTHLY_WITHDRAWAL_RATE * self.liquid_assets()
        // if the min baseline retirement income is greater than the standard monthly withdrawal rate use that (we always need more than the min)
        } else if self.min_baseline_retirement_income
            > self.liquid_assets() * STD_MONTHLY_WITHDRAWAL_RATE
        {
            -1.0 * (self.min_baseline_retirement_income)
        // if the max baseline retirement income is less than the standard monthly withdrawal rate use that (we never need more than the max)
        } else if self.max_baseline_retirement_income
            < self.liquid_assets() * STD_MONTHLY_WITHDRAWAL_RATE
        {
            -1.0 * (self.max_baseline_retirement_income)
        } else {
            unreachable!("shouldn't be here")
        }
    }
    // calculate monthly interest earnings
    pub fn interest_earnings(&self) -> f32 {
        self.liquid_assets() * self.monthly_interest()()
    }
    // income is monthly income + interest earnings
    pub fn income(&mut self) -> f32 {
        if self.active_retirement {
            self.monthly_expenses = 0.0;
            self.monthly_withdrawal()
        } else {
            self.monthly_income
        }
    }
    // end of month expenses for a renter and owner (if renter -> owner is zeroed out, if owner -> renter is zeroed out)
    pub fn expenses(&mut self) -> f32 {
        self.monthly_expenses + owner::Owner::expenses(self) + self.monthly_rent
    }
    // c
    pub fn apply_monthly_changes(&mut self) -> f32 {
        let monthly_inflation = self.monthly_inflation()();
        let month_end = self.total_savings + self.income() - self.expenses();
        self.monthly_income *= 1.0 + monthly_inflation;
        self.monthly_expenses *= 1.0 + monthly_inflation;
        self.monthly_rent *= 1.0 + monthly_inflation;
        self.home_expenses *= 1.0 + monthly_inflation;
        self.min_baseline_retirement_income *= 1.0 + monthly_inflation;
        self.max_baseline_retirement_income *= 1.0 + monthly_inflation;
        month_end
    }
    // run through months then apply the total savings to show only the end of year savings
    pub fn apply_annual_changes(&mut self, st: &SaverType) {
        for _ in 0..12 {
            // apply interest on the savings from the month prior
            let interest = self.interest_earnings();
            match self.apply_monthly_changes() {
                // you can not spend continue if you have no more than your home
                num if num > self.home_value - self.mortgage_debt => {
                    self.total_savings = num + interest;
                }
                _ => {
                    self.total_savings = 0.0;
                    break;
                }
            }
        }
        match st {
            SaverType::HomeOwner => {
                self.home_savings[self.current_age as usize] = self.total_savings;
            }
            SaverType::Renter => {
                self.rental_savings[self.current_age as usize] = self.total_savings;
            }
        }
    }

    // end of month income adjusted for inflation
    pub fn calculate_savings(&mut self, st: SaverType, death_age: u8) -> Vec<f32> {
        match st {
            SaverType::HomeOwner => {
                self.cached_mortgage_installment = Some(Owner::mortgage_installments(self));
                self.home_savings.fill(0.0);
                if self.mortgage_debt == 0.0 {
                    self.home_owned_age = Some(self.current_age);
                } else {
                    self.home_owned_age = None;
                }
                self.home_savings[self.current_age as usize] = self.total_savings;
            }
            SaverType::Renter => {
                self.rental_savings.fill(0.0);
                self.rental_savings[self.current_age as usize] = self.total_savings;
            }
        }
        self.current_age += 1;
        while self.current_age < death_age && self.total_savings > 0.0 {
            self.active_retirement = self.current_age >= self.retirement_age;
            self.apply_annual_changes(&st);
            self.current_age += 1;
        }

        match st {
            SaverType::HomeOwner => {
                self.cached_mortgage_installment = None;
                self.home_savings.clone()
            }
            SaverType::Renter => self.rental_savings.clone(),
        }
    }
}
