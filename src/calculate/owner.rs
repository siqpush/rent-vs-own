use super::saver::Saver;

pub const PROPERTY_TAX: f32 = 0.01 / 12.0;

pub trait Owner<Saver> {
    fn monthly_mortgage_rate(&self) -> f32;
    fn mortgage_term_months(&self) -> f32;
    fn mortgage_installments(&self) -> f32;
    fn monthly_mortgage_interest_payment(&self) -> f32;
    fn monthly_home_expenses(&self) -> f32;
    fn expenses(&mut self) -> f32;
}

impl Owner<Saver> for super::saver::Saver {
    // calculate monthly mortgage rate
    fn monthly_mortgage_rate(&self) -> f32 {
        self.mortgage_rate / 12.0
    }
    // calculate mortgage term in months
    fn mortgage_term_months(&self) -> f32 {
        self.mortgage_term as f32 * 12.0
    }
    // calculate monthly mortgage payment
    fn mortgage_installments(&self) -> f32 {
        if self.monthly_mortgage_rate() == 0.0 {
            self.mortgage_debt / self.mortgage_term_months()
        } else {
            self.mortgage_debt
                * ((self.monthly_mortgage_rate()
                    * (1.0 + self.monthly_mortgage_rate())
                        .powi(self.mortgage_term_months() as i32))
                    / ((1.0 + self.monthly_mortgage_rate())
                        .powi(self.mortgage_term_months() as i32)
                        - 1.0))
        }
    }
    // subtract monthly mortgage payment from mortgage debt and add monthly interest payment
    fn monthly_mortgage_interest_payment(&self) -> f32 {
        self.mortgage_debt * self.monthly_mortgage_rate()
    }

    fn monthly_home_expenses(&self) -> f32 {
        self.home_value * (self.home_expenses / 12.0)
    }

    // calculate monthly expenses for a homeowner (mortgage, property tax, home expenses) + other
    fn expenses(&mut self) -> f32 {
        let mortgage_interest = self.monthly_mortgage_interest_payment();
        let monthly_principle = self.cached_mortgage_installment.unwrap_or(0.0) - mortgage_interest;
        let monthly_expenses =
            self.home_value * PROPERTY_TAX + self.monthly_home_expenses() + mortgage_interest;
        // make a mortgage payment if you have a mortgage
        if self.mortgage_debt > 0.0 {
            self.mortgage_debt -= monthly_principle;
        } else if self.home_owned_age.is_none() {
            self.home_owned_age = Some(self.current_age);
        }
        monthly_expenses
    }
}
