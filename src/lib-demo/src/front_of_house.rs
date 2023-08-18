// pub mod front_of_house {
    fn cook_order() {}

    pub mod hosting {
        pub fn add_to_waitlist() {
            super::cook_order(); // parent
            self::seat_at_table(); // self
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
// }

             