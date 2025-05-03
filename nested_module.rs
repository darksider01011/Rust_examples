// This file is inside "src/lib.rs"


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //relative path
    front_of_house::hosting::add_to_waitlist();
}

