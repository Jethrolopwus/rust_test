use integration_test::{Category, Customer, Order, Product};
mod helpers;

#[test]

fn test_total_bill_without_discount() {
    helpers::common_setup();
    let product = Product::new(1, String::from("Laptop"), 19.9, Category::Electronics);
    let customer = Customer::new(1, String::from("Alice"), String::from("alice@gmail.com"));
    let order = Order::new(2, product, customer, 3);
    assert_eq!(format!("{:.2}", order.total_bill()), "65.67");
}


#[test]
fn test_total_bill_with_discount() {
    let product = Product::new(2, String::from("Smartphone"), 19.99, Category::Electronics);
    let customer = Customer::new(2, String::from("Bob"), String::from("bob@gmail.com"));
    let order = Order::new(3, product, customer, 10);
    assert_eq!(format!("{:.2}", order.total_bill()), "217.69");
   
}
