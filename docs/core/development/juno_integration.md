# Juno Integration Guide

## Core Juno SDK Features

### Collections
```typescript
import { Collection } from '@junobuild/core';

// Initialize collection
const users = new Collection({
    collection: "users",
    database: "db" // "db" or "memory"
});

// Basic operations
async function storeUser(data: UserData) {
    return await users.insert({
        data
    });
}

async function getUser(key: string) {
    return await users.get({ key });
}

async function listUsers(filter?: Filter) {
    return await users.list({
        filter,
        order: {
            desc: true,
            field: "created"
        }
    });
}
```

### Custom Rust Endpoints
```rust
use ic_cdk_macros::update;
use junobuild_macros::collection;

#[collection(name = "users")]
static USERS: Collection = Collection::new();

#[update]
async fn custom_user_operation() -> Result<(), String> {
    // Access collection
    USERS.with(|users| {
        // Your custom logic
    })
}
```

### Authentication Flow
```typescript
import { authSubscribe, signIn } from '@junobuild/core';

// Subscribe to auth changes
authSubscribe((user) => {
    if (user === null) {
        // User signed out
        return;
    }
    
    // User signed in
    console.log('User principal:', user.key);
});

// Initiate sign-in
async function login() {
    try {
        await signIn();
    } catch (err) {
        console.error('Sign-in failed:', err);
    }
}
```

## Memory Management

### Stable Memory
```rust
use ic_cdk::storage;

#[update]
fn store_data(data: Vec<u8>) {
    storage::stable_save((data,)).unwrap();
}

#[query]
fn load_data() -> Vec<u8> {
    let (data,) = storage::stable_restore().unwrap();
    data
}
```

### Cycles Management
```rust
use ic_cdk::api::call::call_with_payment;

#[update]
async fn call_with_cycles<T>(
    canister_id: Principal,
    method: &str,
    args: T,
    cycles: u64
) -> Result<(), String> {
    call_with_payment(
        canister_id,
        method,
        args,
        cycles
    ).await.map_err(|e| e.to_string())
}
```

## Inter-Canister Communication

### Calling Other Canisters
```rust
use ic_cdk::api::call::call;

#[update]
async fn cross_canister_call(
    target: Principal,
    payload: Vec<u8>
) -> Result<(), String> {
    call(
        target,
        "method_name",
        payload
    ).await.map_err(|e| e.to_string())
}
```

## Candid Interface
```candid
type UserData = record {
    id: text;
    name: text;
    created: nat64;
};

service : {
    // Query calls
    get_user: (text) -> (opt UserData) query;
    list_users: () -> (vec UserData) query;
    
    // Update calls
    create_user: (UserData) -> (text);
    update_user: (text, UserData) -> (bool);
}
```

## Error Handling Best Practices
```rust
use ic_cdk::trap;

#[update]
async fn safe_operation() -> Result<(), String> {
    match risky_operation().await {
        Ok(result) => Ok(result),
        Err(e) => {
            // Log error
            ic_cdk::println!("Operation failed: {:?}", e);
            // Return user-friendly message
            Err("Operation failed. Please try again.".to_string())
        }
    }
}

fn assert_caller_is_controller() {
    let caller = ic_cdk::caller();
    if caller != ic_cdk::id() {
        trap("Unauthorized");
    }
}
```

## Related Documentation
- [Juno SDK Documentation](https://docs.juno.build/)
- [Juno Collections API](https://docs.juno.build/build/collections)
- [IC Interface Spec](https://internetcomputer.org/docs/current/references/ic-interface-spec)
- [Rust CDK Documentation](https://docs.rs/ic-cdk) 

## Common Gotchas and Best Practices

### Principal Type Import
```rust
// CORRECT: Always import Principal from candid
use candid::Principal;

// INCORRECT: Do not use these deprecated imports
// use ic_principal::Principal;  // Deprecated
// use ic_types::Principal;      // Deprecated

// The Principal type is fundamental to IC development, representing:
// - User identities
// - Canister IDs
// - Other authenticated entities

// Common usage in structs:
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub owner: Principal,  // Document owner's identity
    // ... other fields
}

// Common usage in functions:
fn get_caller_principal() -> Principal {
    ic_cdk::caller()  // Returns Principal of the calling entity
}
```

### Principal Type Migration Note
The `Principal` type was originally part of the `ic_principal` crate but has been moved to the `candid` crate.
Always use the `candid` import to ensure compatibility with current IC development standards.
This change was made to consolidate core IC types into the `candid` crate, which is the standard serialization
format for the Internet Computer. 