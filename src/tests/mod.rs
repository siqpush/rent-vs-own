#[cfg(test)]
mod tests {
    use leptos::create_signal;

    use crate::{DEATH, calculate::saver::{SaverType, Saver}, Opts};

    #[test]
    fn test_saver_with_zero_mortgage() {
        let (inflation_rates, _) = create_signal(vec![Opts::Float(0.0); 100]);
        let (interest_rates, _) = create_signal(vec![Opts::Float(0.0); 100]);
    
        // Saver {
        //     monthly_rent: rent.get().get_float(),
        //     current_age: age.get().get_int(),
        //     retirement_age: retirement_age.get().get_int(),
        //     total_savings: networth.get().get_float(),
        //     monthly_income: monthly_income.get().get_float(),
        //     monthly_expenses: monthly_expenses.get().get_float(),
        //     home_value: 0.0,
        //     mortgage_debt: 0.0,
        //     mortgage_rate: 0.0,
        //     mortgage_term: 0,
        //     min_baseline_retirement_income: min_retirement_income.get().get_float(),
        //     max_baseline_retirement_income: max_retirement_income.get().get_float(),
        //     home_expenses: 0.0,
        //     home_savings: vec![0.0; DEATH],
        //     rental_savings: vec![0.0; DEATH],
        //     active_retirement: false,
        //     home_owned_age: None::<u8>,
        //     cached_mortgage_installment: None::<f32>,
        //     interest_rates,
        //     inflation_rates,
        // }
        let rates = Saver {
            monthly_rent:0.0,
            current_age:10,
            retirement_age:65,
            total_savings:100000.0,
            monthly_income:10000.0,
            monthly_expenses:5000.0,
            home_value:10000.0,
            mortgage_debt:0.0,
            mortgage_rate:0.0,
            mortgage_term:0,
            min_baseline_retirement_income:5000.0,
            max_baseline_retirement_income:10000.0,
            home_expenses:0.0,
            home_savings:vec![0.0;DEATH],
            rental_savings:vec![0.0;DEATH],
            active_retirement:false,
            home_owned_age:None::<u8>,
            cached_mortgage_installment:None::<f32>,
            interest_rates, 
            inflation_rates, 
        }.calculate_savings(SaverType::HomeOwner, DEATH as u8);
        assert_eq!(rates, vec![0.0;DEATH]);
    }
}
