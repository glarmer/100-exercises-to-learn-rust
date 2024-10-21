// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}

impl Order {

    //Constructor
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        if !(Self::is_name_valid(&product_name)) {
            panic!("The name is invalid!")
        }
        if !(Self::is_quantity_valid(&quantity)) {
            panic!("The quantity is invalid!")
        }
        if !(Self::is_price_valid(&unit_price)) {
            panic!("The unit price is invalid!")
        }
        Order {
            product_name,
            quantity,
            unit_price
        }
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, new_name: String) {
        if !(Self::is_name_valid(&new_name)) {
            panic!("Name is invalid!")
        }
        self.product_name = new_name
    }

    pub fn set_quantity(&mut self, new_quantity: u32) {
        if !(Self::is_quantity_valid(&new_quantity)) {
            panic!("Quantity is invalid!")
        }
        self.quantity = new_quantity
    }

    pub fn set_unit_price(&mut self, new_price: u32) {
        if !(Self::is_price_valid(&new_price)) {
            panic!("Price is invalid!")
        }
        self.unit_price = new_price
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    fn is_name_valid(name: &String) -> bool {
        let mut valid: bool = false;
        if !name.is_empty() && name.len() < 300 {
            valid = true
        }
        valid
    }

    fn is_quantity_valid(quantity: &u32) -> bool {
        let mut valid: bool = false;
        if quantity > &0u32 {
            valid = true
        }
        valid
    }

    fn is_price_valid(price: &u32) -> bool {
        let mut valid: bool = false;
        if price > &0u32 {
            valid = true
        }
        valid
    }
}
