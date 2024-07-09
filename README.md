# Supply Chain Management Canister

This Rust-based Internet Computer (ICP) canister implements a decentralized supply chain management system. It allows for managing products, suppliers, orders, and shipments, and incorporates decentralized proof of location (PoL) techniques to enhance the authenticity and integrity of location data within the supply chain.

## Features

- **Product Management**: CRUD operations for managing product information.
- **Supplier Management**: CRUD operations for managing supplier details.
- **Order Management**: CRUD operations for managing customer orders.
- **Shipment Management**: CRUD operations for managing shipments, including status updates and location proofs.
- **Decentralized Proof of Location**: Incorporates techniques for decentralized location verification to ensure the integrity of shipment tracking.

## Architecture

The canister uses thread-local storage to manage the state and generate unique IDs for various entities. The data is persisted using `MemoryManager` and `StableBTreeMap`.

## Data Structures

### Product
```rust
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Product {
    id: u64,
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}
```

### Supplier
```rust
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Supplier {
    id: u64,
    name: String,
    contact_info: String,
}
```

### Order
```rust
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Order {
    id: u64,
    product_id: u64,
    quantity: u32,
    total_price: f64,
    status: OrderStatus,
    created_at: u64,
    updated_at: Option<u64>,
}
```

### Shipment
```rust
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
```

### User
```rust
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    username: String,
    password: String,
    role: UserRole,
}
```

### Enums

#### OrderStatus
```rust
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Debug)]
enum OrderStatus {
    Pending,
    Confirmed,
    Canceled,
}
```

#### ShipmentStatus
```rust
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Debug)]
enum ShipmentStatus {
    Pending,
    Shipped,
    InTransit,
    Delivered,
    Canceled,
}
```

#### UserRole
```rust
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Debug)]
enum UserRole {
    Admin,
    Supplier,
    Customer,
}
```

### LocationProof
```rust
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Debug)]
struct LocationProof {
    timestamp: u64,
    location_data: String,
    verifier: String,
}
```

### MessagePayload
```rust
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct MessagePayload {
    title: String,
    body: String,
    attachment_url: String,
}
```

## CRUD Operations

### Product Management
- **Add Product**
- **Get Product**
- **Update Product**
- **Delete Product**

### Supplier Management
- **Add Supplier**
- **Get Supplier**
- **Update Supplier**
- **Delete Supplier**

### Order Management
- **Add Order**
- **Get Order**
- **Update Order**
- **Delete Order**

### Shipment Management
- **Add Shipment**
- **Get Shipment**
- **Update Shipment Status**
- **Delete Shipment**
- **Get Shipment Location Proofs**

## Example Code Snippets

### Adding a Product
```rust
#[ic_cdk::update]
fn add_product(product: Product) -> Result<Product, Error> {
    let id = ID_COUNTER.with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1);
        current_value + 1
    })?;
    let new_product = Product { id, ..product };
    do_insert_product(&new_product);
    Ok(new_product)
}

fn do_insert_product(product: &Product) {
    STORAGE.with(|s| s.borrow_mut().insert(product.id, product.clone()));
}
```

### Updating Shipment Status
```rust
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

fn do_insert_shipment(shipment: &Shipment) {
    STORAGE.with(|s| s.borrow_mut().insert(shipment.id, shipment.clone()));
}
```

### Retrieving Shipment Location Proofs
```rust
#[ic_cdk::query]
fn get_shipment_location_proofs(id: u64) -> Result<Vec<LocationProof>, Error> {
    match _get_shipment(&id) {
        Some(shipment) => Ok(shipment.location_proofs),
        None => Err(Error::NotFound {
            msg: format!("A shipment with id={} not found", id),
        }),
    }
}
```

## Installation and Deployment

### Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background

## Running the project locally

If you want to test the project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```


   ```sh
./did.sh && dfx generate && dfx deploy -y
   ```



## Contributing

Contributions are welcome! Please submit a pull request or open an issue to discuss any changes or improvements.

