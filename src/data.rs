use fake::Fake;
use fake::faker::address::en::{CityName, CountryName, StateAbbr, StreetName, ZipCode};
use fake::faker::internet::en::FreeEmail;
use fake::faker::name::en::{LastName, Name};
use fake::uuid::UUIDv5;
use juniper::FieldResult;

#[derive(Clone, Default)]
pub struct Ctx{}
impl Ctx {
    pub fn new() -> Ctx {
        Ctx{}
    }
}
impl juniper::Context for Ctx {}


pub struct Query;

#[juniper::graphql_object(context=Ctx)]
impl Query {
    pub fn customers() -> FieldResult<Vec<Customer>> {
        let customers = Customer::fake_many();
        Ok(customers)
    }
    pub fn customer_by_email(email: String) -> FieldResult<Customer> {
        let customer = Customer::fake();
        Ok(Customer{email, ..customer})
    }

    pub fn customer_by_id(id: String) -> FieldResult<Customer> {
        let customer = Customer::fake();
        Ok(Customer{id, ..customer})
    }
}


#[derive(GraphQLObject, Clone)]
#[graphql(description="Ecommerce customer")]
pub struct Customer {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub addresses: Vec<DeliveryAddress>
}

impl Customer {
    pub fn fake() -> Customer {
        let id = UUIDv5{}.fake();
        let email = FreeEmail().fake();
        let first_name = Name().fake();
        let last_name = LastName().fake();
        let full_name = format!("{} {}", first_name, last_name);
        let count = (1..5).fake();
        let addresses: Vec<DeliveryAddress> = (0..count).into_iter().map(|_| DeliveryAddress::fake(&full_name)).collect();
        Customer {
            id, email, first_name, last_name, addresses
        }
    }

    pub fn fake_many() -> Vec<Customer> {
        let count = (0..10).fake();
        (0..count).into_iter().map(|_| Customer::fake()).collect()
    }
}

#[derive(GraphQLObject, Clone)]
#[graphql(description="Customer physical address")]
pub struct DeliveryAddress {
    pub id: String,
    address: String,
    city: String,
    state: String,
    postal_code: String,
    recipient: String,
    country: String
}

impl DeliveryAddress {
    pub fn fake(recipientPtr: &String) -> DeliveryAddress {
        let id = UUIDv5{}.fake();
        let address = StreetName().fake();
        let city = CityName().fake();
        let state = StateAbbr().fake();
        let postal_code = ZipCode().fake();
        let country = CountryName().fake();
        let recipient = recipientPtr.clone();
        DeliveryAddress {
            id, address, city, state, postal_code, recipient, country
        }
    }
}