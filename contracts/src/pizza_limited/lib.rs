#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod pizza_limited {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use ink::prelude::format;
    use ink::scale::{Decode, Encode};
    use ink::H160;

    #[ink(storage)]
    pub struct PizzaLimited {
        daily_supply: u32,
        orders_per_user: Mapping<H160, u32>,
        remaining_supply: u32,
        max_order_per_user: u32,
        last_reset: Timestamp,
        total_orders: u32,
    }
      #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum PizzaError {
        DailySupplyReached, 
        MaxOrderPerUserReached,
        CannotOrderZeroPizzas,
    }
    
    #[ink(event)]
    pub struct PizzaOrdered {
        caller: Option<H160>,
        quantity_ordered: u32,
    }

    // #[ink(event)]
    // pub struct Transferred {
    //     #[ink(topic)]
    //     from: Option<H160>,
    //     #[ink(topic)]
    //     to: Option<H160>,
    //     value: Balance,
    // }

 pub type Result<T> = core::result::Result<T, PizzaError>;

    impl PizzaLimited {
        #[ink(constructor)]
        pub fn new(
            
            max_order_per_user: u32,
             
            ) -> Self {
                let daily_supply = 50;
            Self {
                daily_supply,
                max_order_per_user,
                orders_per_user: Mapping::default(),
                remaining_supply: daily_supply,
                last_reset: Self::env().block_timestamp(),
                total_orders: 0,
        }
    }

        #[ink(message)]
        pub fn get_remaining_supply(&self) -> u32 {
            self.daily_supply
        }
      
        #[ink(message)]
        pub fn get_daily_supply(&self) -> u32 {
            self.remaining_supply
        }

        #[ink(message)]
        pub fn get_last_reset(&self) -> Timestamp{
            self.last_reset
        }

        #[ink(message)]
pub fn order_pizza(
    &mut self,
    quantity_ordered: u32,
    
) -> Result<u32> {
    let caller: H160 = self.env().caller();
   // let transferred_balance = self.env().transferred_value();
    //let price_per_pizza: Balance = 2_000_000_000_000; 
    let max_order_per_user: u32 = 5;
    if quantity_ordered == 0 {
        return Err(PizzaError::CannotOrderZeroPizzas);
    }

    // Check daily supply
    if self.daily_supply < quantity_ordered {
        return Err(PizzaError::DailySupplyReached);
    }

    // Check current orders by user
    let user_orders = self.orders_per_user.get(&caller).unwrap_or(0);
    if user_orders + quantity_ordered > max_order_per_user {
        return Err(PizzaError::MaxOrderPerUserReached);
    }

    // Compute required payment
//    let total_price_for_orders: Balance = price_per_pizza * (quantity_ordered as Balance);

    // Update state
    self.daily_supply -= quantity_ordered;
    self.orders_per_user.insert(caller, &(user_orders + quantity_ordered));
    self.total_orders += quantity_ordered;
    // Emit events
    self.env().emit_event(PizzaOrdered {
        caller: Some(caller),
        quantity_ordered,
    });

    // self.env().emit_event(Transferred {
    //         from: Some(caller),
    //         to: Some(self.env().address()),
    //         value: total_price_for_orders,
    //     });

    Ok(user_orders + quantity_ordered)
}
    #[ink(message)]
        pub fn reset_supply(&mut self) -> u32 {
    let now = self.env().block_timestamp();
    let one_day: Timestamp = 24 * 60 * 60 * 1000;
    if let Some(next_reset) = self.last_reset.checked_add(one_day) {
        if now >= next_reset {
            self.last_reset = now;
            return self.daily_supply;
        }
    }
    self.remaining_supply
}
    
}

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn it_works() {
            let pizza_limited = PizzaLimited::new(20, 2, 12);
            assert_eq!(pizza_limited.get_daily_supply(), 20);
        }

        #[ink::test]
        fn get_remaining_supply_works() {
            let mut pizza_limited = PizzaLimited::new(20, 2, 15);
            assert_eq!(pizza_limited.get_remaining_supply(), 15);
        }

        #[ink::test]
        fn get_daily_supply_works() {
            let pizza_limited = PizzaLimited::new(20, 2, 10);
            assert_eq!(pizza_limited.get_daily_supply(), 20);
        }

        #[ink::test]
        fn get_last_reset_works() {
            let mut pizza_limited = PizzaLimited::new(20, 2, 10);
            assert_eq!(pizza_limited.get_last_reset(), pizza_limited.last_reset);
        }


     #[ink::test]
fn reset_supply_works() {
    let mut pizza_limited = PizzaLimited::new(20, 2, 5);
    // Simulate passage of time by setting last_reset so that one day has passed
    let one_day: Timestamp = 24 * 60 * 60 * 1000;
    pizza_limited.last_reset = pizza_limited.last_reset.checked_add(one_day).unwrap_or(0);
    let reset_result = pizza_limited.reset_supply();
   assert_eq!(reset_result, 5);
    assert_eq!(pizza_limited.remaining_supply, 5);
}
          
        }


   
}

