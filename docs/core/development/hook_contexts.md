# Hook Contexts

The hook context is the main object passed to hooks. It varies depending on the hook type.

For the on_set_doc hook, the context is as follows:
```rust
{
    "caller": "Principal",  // The Principal ID of who called the function
    "data": {
        "collection": "String",  // Name of the collection (e.g., "users")
        "key": "String",        // Document key
        "data": {
            "before": {         // State of document before update (null if new doc)
                "data": "Blob",      // The actual document data (encoded)
                "owner": "Principal", // Who owns this document
                "description": "String?",  // Optional description
                "created_at": "u64",  // Creation timestamp
                "updated_at": "u64",  // Last update timestamp
                "version": "u64?"     // Optional version number
            },
            "after": {          // State of document after update
                "data": "Blob",      // The actual document data (encoded)
                "owner": "Principal", // Who owns this document
                "description": "String?",  // Optional description
                "created_at": "u64",  // Creation timestamp
                "updated_at": "u64",  // Last update timestamp
                "version": "u64?"     // Optional version number
            }
        }
    }
}
```