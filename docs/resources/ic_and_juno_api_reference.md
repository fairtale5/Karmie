# Internet Computer & Juno SDK Complete Reference

> **Purpose**: This document serves as a comprehensive technical reference for the Internet Computer (IC) and Juno SDK APIs. It complements the implementation guides in `/docs/implementation/` and the Juno documentation in `/docs/juno/`.

> **Related Documentation**:
> - Implementation guides: `/docs/implementation/`
> - Juno documentation: `/docs/juno/`
> - Core specifications: `/docs/core/`

This document provides a comprehensive reference of all capabilities available in both the Internet Computer Rust CDK and Juno SDK.

## IC Rust CDK Complete Reference (`ic-cdk`)

### API Module (`ic_cdk::api`)

#### Call Management
```rust
// Making calls
api::call::call(canister_id: Principal, method_name: &str, args: Candid_Type) -> Result<Response, Error>
api::call::call_with_payment(canister_id: Principal, method_name: &str, args: Candid_Type, cycles: u64) -> Result<Response, Error>
api::call::call_raw(canister_id: Principal, method_name: &str, args: &[u8], payment: u64) -> Result<Response, Error>

// Response handling
api::call::reply(args) -> ()
api::call::reject(message: &str) -> ()
api::call::reject_code() -> RejectionCode
api::call::reject_message() -> String
api::call::performance_counter(counter_type: PerformanceCounterType) -> u64
```

#### Cycles & Balance
```rust
api::call::msg_cycles_available() -> u64
api::call::msg_cycles_available128() -> u128
api::call::msg_cycles_accept(max_amount: u64) -> u64
api::call::msg_cycles_accept128(max_amount: u128) -> u128
api::call::msg_cycles_refunded() -> u64
api::call::msg_cycles_refunded128() -> u128
api::canister_balance() -> u64
api::canister_balance128() -> u128
```

#### Time & System Info
```rust
api::time() -> u64  // Current time in nanoseconds
api::instruction_counter() -> u64  // Instructions executed so far
api::performance_counter(counter_type: PerformanceCounterType) -> u64
```

#### Stable Memory
```rust
api::stable::stable64_size() -> u64
api::stable::stable64_grow(new_pages: u64) -> Result<u64, String>
api::stable::stable64_read(offset: u64, dst: &mut [u8])
api::stable::stable64_write(offset: u64, src: &[u8])

api::stable::stable_size() -> u32  // 32-bit version
api::stable::stable_grow(new_pages: u32) -> Result<u32, String>
api::stable::stable_read(offset: u32, dst: &mut [u8])
api::stable::stable_write(offset: u32, src: &[u8])
```

#### Identity & Authentication
```rust
api::id() -> Principal  // Canister's ID
caller() -> Principal   // Caller's Principal
api::trap(message: &str) -> !  // Trap with message
api::set_certified_data(data: &[u8])
api::data_certificate() -> Option<Vec<u8>>
```

### Storage Module (`ic_cdk::storage`)

```rust
storage::stable_save<T>(t: &T) -> Result<(), String>
storage::stable_restore<T>() -> Result<T, String>
storage::stable_grow(new_pages: u32) -> Result<u32, String>
storage::stable_size() -> u32
```

### Export Macros

```rust
#[init]  // Canister initialization
#[pre_upgrade]  // Before upgrade
#[post_upgrade]  // After upgrade
#[query]  // Read-only functions
#[update]  // State-modifying functions
#[heartbeat]  // Periodic background task
#[inspect_message]  // Message inspection
```

### Stable Structures (`ic-stable-structures`)

```rust
// Memory
Memory::new(memory_id: u8)
Memory::size(&self) -> u64
Memory::grow(&mut self, pages: u64) -> Result<u64>

// BTreeMap
StableBTreeMap::new(memory: Memory)
StableBTreeMap::insert(&mut self, key: K, value: V) -> Option<V>
StableBTreeMap::get(&self, key: &K) -> Option<&V>
StableBTreeMap::remove(&mut self, key: &K) -> Option<V>

// Vec
StableVec::new(memory: Memory)
StableVec::push(&mut self, value: T)
StableVec::pop(&mut self) -> Option<T>
StableVec::get(&self, index: u64) -> Option<&T>
```

## Juno SDK Complete Reference

### Authentication (`@junobuild/core`)

```typescript
// Sign In
signIn(options?: SignInOptions): Promise<User | undefined>
signOut(): Promise<void>

// Auth State
authSubscribe(callback: (user: User | null) => void): Promise<void>
getUser(): Promise<User | null>

// Providers
class InternetIdentityProvider implements AuthProvider
class NFIDProvider implements AuthProvider
```

### Collections

```typescript
// Initialize
const collection = new Collection({
    collection: string,
    database: "db" | "memory"
});

// Operations
collection.insert<T>({ data: T }): Promise<Doc<T>>
collection.get<T>({ key: string }): Promise<Doc<T> | undefined>
collection.list<T>(options?: ListOptions): Promise<Doc<T>[]>
collection.remove({ key: string }): Promise<void>
collection.count(options?: CountOptions): Promise<number>

// Batch Operations
collection.bulkInsert<T>(docs: T[]): Promise<string[]>
collection.bulkRemove(keys: string[]): Promise<void>

// Subscriptions
collection.onChange<T>(callback: (doc: Doc<T>) => void): Unsubscribe
```

### Storage

```typescript
// Initialize
const storage = new Storage({
    container: string
});

// Operations
storage.upload(options: UploadOptions): Promise<Asset>
storage.downloadUrl(key: string): Promise<string>
storage.list(options?: ListOptions): Promise<Asset[]>
storage.remove(key: string): Promise<void>

// Batch Operations
storage.bulkUpload(files: File[]): Promise<Asset[]>
storage.bulkRemove(keys: string[]): Promise<void>
```

### Satellite Management

```typescript
// Initialize
initJuno(options: {
    satelliteId: string,
    workers?: boolean
}): Promise<void>

// Configuration
setDoc(options: SetDocOptions): Promise<void>
getDoc(options: GetDocOptions): Promise<Doc | undefined>
```

### Types

```typescript
interface User {
    key: string;  // Principal ID
    created: bigint;
    updated: bigint;
}

interface Doc<T> {
    key: string;
    data: T;
    owner: string;  // Principal ID
    created: bigint;
    updated: bigint;
}

interface Asset {
    key: string;
    url: string;
    downloadUrl: string;
    created: bigint;
    updated: bigint;
    size: bigint;
    headers: Record<string, string>;
}
```

## Best Practices & Patterns

[Previous best practices section remains...]

## Important Notes

### IC Limitations
1. **Memory**
   - Heap: 4GB per canister
   - Stable: 48GB per canister
   - Message: 2MB max size
   - Cycles per message: 10T cycles

2. **Performance**
   - Query calls: ~200ms
   - Update calls: ~2s
   - Inter-canister calls: +1-2s per hop
   - Maximum instruction count: 5B per message

3. **Rate Limits**
   - HTTP outcalls: 1000 per day
   - Ingress messages: 10/sec per user
   - Inter-canister calls: 1000/sec

### Juno Limitations
1. **Collections**
   - Maximum document size: 1MB
   - Maximum batch size: 100 documents
   - Rate limits apply per collection

2. **Storage**
   - Maximum file size: 5MB
   - Supported formats: Images, PDFs, common web assets
   - CDN caching: 1 hour default

## Resources

### Official Documentation
- [IC Interface Spec](https://internetcomputer.org/docs/current/references/ic-interface-spec)
- [Rust CDK API Reference](https://docs.rs/ic-cdk)
- [Juno Documentation](https://docs.juno.build)
- [IC Developer Docs](https://internetcomputer.org/docs/current/developer-docs/) 