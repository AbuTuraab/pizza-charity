#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod pizza_limited {
    use ink::prelude::string::String;
    use ink::scale::{Decode, Encode};
    use ink::storage::Mapping;
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

    pub type Result<T> = core::result::Result<T, PizzaError>;

    impl PizzaLimited {
        #[ink(constructor)]
        pub fn new(max_order_per_user: u32) -> Self {
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
            self.remaining_supply
        }

        #[ink(message)]
        pub fn get_daily_supply(&self) -> u32 {
            self.daily_supply
        }

        #[ink(message)]
        pub fn get_last_reset(&self) -> Timestamp {
            self.last_reset
        }

        #[ink(message)]
        pub fn order_pizza(&mut self, quantity_ordered: u32) -> Result<u32> {
            let caller: H160 = self.env().caller();
            if quantity_ordered == 0 {
                return Err(PizzaError::CannotOrderZeroPizzas);
            }

            if self.remaining_supply < quantity_ordered {
                return Err(PizzaError::DailySupplyReached);
            }

            let user_orders = self.orders_per_user.get(&caller).unwrap_or(0);
            if user_orders + quantity_ordered > self.max_order_per_user {
                return Err(PizzaError::MaxOrderPerUserReached);
            }

            self.remaining_supply -= quantity_ordered;
            self.orders_per_user
                .insert(caller, &(user_orders + quantity_ordered));
            self.total_orders += quantity_ordered;

            self.env().emit_event(PizzaOrdered {
                caller: Some(caller),
                quantity_ordered,
            });

            Ok(user_orders + quantity_ordered)
        }

        #[ink(message)]
        pub fn reset_supply(&mut self) -> u32 {
            self.remaining_supply = self.daily_supply;
            self.max_order_per_user = self.max_order_per_user;
            self.remaining_supply
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn it_works() {
            let pizza_limited = PizzaLimited::new(5);
            assert_eq!(pizza_limited.get_daily_supply(), 50);
        }

        #[ink::test]
        fn get_remaining_supply_works() {
            let pizza_limited = PizzaLimited::new(5);
            assert_eq!(pizza_limited.get_remaining_supply(), 50);
        }

        #[ink::test]
        fn order_pizza_works() {
            let mut contract = PizzaLimited::new(5);
            let result = contract.order_pizza(3);
            assert_eq!(result, Ok(3));
            assert_eq!(contract.remaining_supply, 47);
            assert_eq!(contract.total_orders, 3);

            let result = contract.order_pizza(0);
            assert_eq!(result, Err(PizzaError::CannotOrderZeroPizzas));

            let mut contract = PizzaLimited::new(5);
            let _ = contract.order_pizza(3);
            let result = contract.order_pizza(3);
            assert_eq!(result, Err(PizzaError::MaxOrderPerUserReached));

            let mut contract = PizzaLimited::new(5);
        }
    }
}
