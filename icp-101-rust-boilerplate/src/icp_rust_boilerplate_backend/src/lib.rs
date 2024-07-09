#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};


type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Product {
    id: u64,
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Supplier {
    id: u64,
    name: String,
    contact_info: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Order {
    id: u64,
    product_id: u64,
    quantity: u32,
    order_date: u64,
    delivery_date: Option<u64>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Shipment {
    id: u64,
    order_id: u64,
    shipping_details: String,
    status: ShipmentStatus,
    created_at: u64,
    updated_at: Option<u64>,
    location_proofs: Vec<LocationProof>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default, Debug)]
struct LocationProof {
    timestamp: u64,
    location_data: String,
    verifier: String,
}

impl Storable for Product {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Product {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Supplier {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Supplier {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Order {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Order {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Shipment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Shipment {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static PRODUCT_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static SUPPLIER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create a counter")
    );

    static ORDER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0)
            .expect("Cannot create a counter")
    );

    static SHIPMENT_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))), 0)
            .expect("Cannot create a counter")
    );

    static PRODUCT_STORAGE: RefCell<StableBTreeMap<u64, Product, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static SUPPLIER_STORAGE: RefCell<StableBTreeMap<u64, Supplier, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static ORDER_STORAGE: RefCell<StableBTreeMap<u64, Order, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));

    static SHIPMENT_STORAGE: RefCell<StableBTreeMap<u64, Shipment, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));
    static USER_ID_COUNTER: std::cell::RefCell<IdCell> = std::cell::RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(8))), 0)
            .expect("Cannot create a user ID counter")
    );

    static USER_STORAGE: std::cell::RefCell<StableBTreeMap<u64, User, Memory>> =
        std::cell::RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(9)))
    ));
}
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    username: String,
    email: String,
    role: UserRole,
}
#[derive(Clone)]
#[derive(candid::CandidType, Deserialize, Serialize)]
enum UserRole {
    Admin,
    Supplier,
    Customer,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Customer
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ProductPayload {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct SupplierPayload {
    name: String,
    contact_info: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct OrderPayload {
    product_id: u64,
    quantity: u32,
    order_date: u64,
    delivery_date: Option<u64>,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ShipmentPayload {
    order_id: u64,
    shipped_date: u64,
    expected_arrival_date: u64,
    shipping_details: String,
}
#[derive(candid::CandidType, Deserialize, Serialize, Clone, Debug, Default)]
enum ShipmentStatus {
    #[default]
    Pending,
    Shipped,
    InTransit,
    Delivered,
    Canceled,
}



#[ic_cdk::query]
fn get_user(id: u64) -> Result<User, String> {
    match _get_user(&id) {
        Some(user) => Ok(user),
        None => Err(format!("User with id={} not found", id)),
    }
}

fn _get_user(id: &u64) -> Option<User> {
    USER_STORAGE.with(|s| s.borrow().get(id))
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct UserPayload {
    username: String,
    email: String,
    role: UserRole,
}

#[ic_cdk::update]
fn add_user(payload: UserPayload) -> Result<User, String> {
    let id = USER_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1).expect("Cannot increment user ID counter")
        });
    let user = User {
        id,
        username: payload.username,
        email: payload.email,
        role: payload.role,
    };
    do_insert_user(&user);
    Ok(user)
}

fn do_insert_user(user: &User) {
    USER_STORAGE.with(|service| service.borrow_mut().insert(user.id, user.clone()));
}

#[ic_cdk::update]
fn update_user(id: u64, payload: UserPayload) -> Result<User, String> {
    match USER_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut user) => {
            user.username = payload.username;
            user.email = payload.email;
            user.role = payload.role;
            do_insert_user(&user);
            Ok(user)
        }
        None => Err(format!("Couldn't update user with id={}. User not found", id)),
    }
}

#[ic_cdk::update]
fn delete_user(id: u64) -> Result<User, String> {
    match USER_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(user) => Ok(user),
        None => Err(format!("Couldn't delete user with id={}. User not found.", id)),
    }
}


#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

#[ic_cdk::query]
fn get_product(id: u64) -> Result<Product, String> {
    match _get_product(&id) {
        Some(product) => Ok(product),
        None => Err(format!("Product with id={} not found", id)),
    }
}

fn _get_product(id: &u64) -> Option<Product> {
    PRODUCT_STORAGE.with(|s| s.borrow().get(id))
}



#[ic_cdk::update]
fn add_product(payload: ProductPayload) -> Result<Product, String> {
    let id = PRODUCT_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1).expect("Cannot increment product ID counter")
        });
    let product = Product {
        id,
        name: payload.name,
        description: payload.description,
        price: payload.price,
        quantity: payload.quantity,
    };
    do_insert_product(&product);
    Ok(product)
}

fn do_insert_product(product: &Product) {
    PRODUCT_STORAGE.with(|service| service.borrow_mut().insert(product.id, product.clone()));
}
#[ic_cdk::update]
fn update_product(id: u64, payload: ProductPayload) -> Result<Product, String> {
    match PRODUCT_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut product) => {
            product.name = payload.name;
            product.description = payload.description;
            product.price = payload.price;
            product.quantity = payload.quantity;
            do_insert_product(&product);
            Ok(product)
        }
        None => Err(format!("Couldn't update product with id={}. Product not found", id)),
    }
}
#[ic_cdk::update]
fn delete_product(id: u64) -> Result<Product, String> {
    match PRODUCT_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(product) => Ok(product),
        None => Err(format!("Couldn't delete product with id={}. Product not found.", id)),
    }
}
#[ic_cdk::query]
fn get_order(id: u64) -> Result<Order, String> {
    match _get_order(&id) {
        Some(order) => Ok(order),
        None => Err(format!("Order with id={} not found", id)),
    }
}

fn _get_order(id: &u64) -> Option<Order> {
    ORDER_STORAGE.with(|s| s.borrow().get(id))
}


#[ic_cdk::update]
fn add_order(payload: OrderPayload) -> Result<Order, String> {
    let id = ORDER_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1).expect("Cannot increment order ID counter")
        });
    let order = Order {
        id,
        product_id: payload.product_id,
        quantity: payload.quantity,
        order_date: payload.order_date,
        delivery_date: payload.delivery_date,
    };
    do_insert_order(&order);
    Ok(order)
}

fn do_insert_order(order: &Order) {
    ORDER_STORAGE.with(|service| service.borrow_mut().insert(order.id, order.clone()));
}
#[ic_cdk::update]
fn update_order(id: u64, payload: OrderPayload) -> Result<Order, String> {
    match ORDER_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut order) => {
            order.product_id = payload.product_id;
            order.quantity = payload.quantity;
            order.order_date = payload.order_date;
            order.delivery_date = payload.delivery_date;
            do_insert_order(&order);
            Ok(order)
        }
        None => Err(format!("Couldn't update order with id={}. Order not found", id)),
    }
}
#[ic_cdk::update]
fn delete_order(id: u64) -> Result<Order, String> {
    match ORDER_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(order) => Ok(order),
        None => Err(format!("Couldn't delete order with id={}. Order not found.", id)),
    }
}
#[ic_cdk::query]
fn get_shipment(id: u64) -> Result<Shipment, String> {
    match _get_shipment(&id) {
        Some(shipment) => Ok(shipment),
        None => Err(format!("Shipment with id={} not found", id)),
    }
}

fn _get_shipment(id: &u64) -> Option<Shipment> {
    SHIPMENT_STORAGE.with(|s| s.borrow().get(id))
}


#[ic_cdk::update]
fn add_shipment(payload: ShipmentPayload) -> Result<Shipment, String> {
    let id = SHIPMENT_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1).expect("Cannot increment shipment ID counter")
        });
        let shipment = Shipment {
            id,
            order_id: payload.order_id,
            shipping_details: payload.shipping_details,
            status: ShipmentStatus::Pending,
            created_at: time(),
            updated_at: None,
            location_proofs: vec![],
        };
    do_insert_shipment(&shipment);
    Ok(shipment)
}

fn do_insert_shipment(shipment: &Shipment) {
    SHIPMENT_STORAGE.with(|service| service.borrow_mut().insert(shipment.id, shipment.clone()));
}



#[ic_cdk::update]
fn update_shipment_status(id: u64, status: ShipmentStatus, proof: LocationProof) -> Result<Shipment, Error> {
    match _get_shipment(&id) {
        Some(mut shipment) => {
            shipment.status = status;
            shipment.updated_at = Some(time());
            shipment.location_proofs.push(proof);
            do_insert_shipment(&shipment);
            Ok(shipment)
        }
        None => Err(Error::NotFound {
            msg: format!("A shipment with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_shipment_status(id: u64) -> Result<ShipmentStatus, Error> {
    match _get_shipment(&id) {
        Some(shipment) => Ok(shipment.status),
        None => Err(Error::NotFound {
            msg: format!("A shipment with id={} not found", id),
        }),
    }
}
#[ic_cdk::update]
fn delete_shipment(id: u64) -> Result<Shipment, String> {
    match SHIPMENT_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(shipment) => Ok(shipment),
        None => Err(format!("Couldn't delete shipment with id={}. Shipment not found.", id)),
    }
}

#[ic_cdk::query]
fn get_shipment_location_proofs(id: u64) -> Result<Vec<LocationProof>, Error> {
    match _get_shipment(&id) {
        Some(shipment) => Ok(shipment.location_proofs),
        None => Err(Error::NotFound {
            msg: format!("A shipment with id={} not found", id),
        }),
    }
}
ic_cdk::export_candid!();



