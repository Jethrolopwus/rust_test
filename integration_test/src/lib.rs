pub use customer::Customer;
pub use order::Order;
pub use product::{Product, Category};



mod product {

    pub use category::Category;

  pub struct Product {
    pub id: u64,           
    pub name: String,      
    pub price: f64,
    pub category: Category, 
}

    impl Product {
        pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
            Product {
                id,
                name,
                price,
                category,
            }
        }

        fn calculate_tax (&self) -> f64 {
           self.price * 0.1
        }


        pub fn product_price(&self) -> f64 {
            self.price + self.calculate_tax()
        }


       
    } 
    
    mod category {
        #[derive(Debug, PartialEq)]
        pub enum Category {
            Electronics,
            Clothing,
            Home,
        }
    }  


}


mod customer {
    pub struct Customer {
        pub id: u64,
        pub name: String,
        pub email: String,
    }

    impl Customer {
        pub fn new(id: u64, name: String, email: String) -> Customer {
            Customer { id, name, email }
        }
    }
}


mod order {
    use super::product::Product;
    use super::customer::Customer;

    pub struct Order {
        pub id: u64,
        pub product: Product,
        pub customer: Customer,
        pub quantity: u32,
    }

    impl Order {
        pub fn new(id: u64, product: Product, customer: Customer, quantity: u32) -> Order {
            Order {
                id,
                product,
                customer,
                quantity,
            }
        }

        fn calculate_discount(&self) -> f64 {
            if self.quantity >= 5 {
                0.01
            } else {
                0.0
            }
        }

        pub fn total_bill(&self) -> f64 {
            let discount = self.calculate_discount();
            let total_before_discount = self.product.product_price() * self.quantity as f64;
            total_before_discount - (total_before_discount * discount)
            }
        }
    }



