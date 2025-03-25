// AVAILABLE IMPORTS FROM junobuild_macros:
// These are the only macro decorators available:
use junobuild_macros::{
    assert_delete_asset,        // For asserting asset deletion
    assert_delete_doc,          // For asserting document deletion
    assert_set_doc,             // For asserting document creation/update
    assert_upload_asset,        // For asserting asset upload
    on_delete_asset,            // For handling asset deletion
    on_delete_doc,              // For handling document deletion
    on_delete_filtered_assets,  // For handling filtered asset deletion
    on_delete_filtered_docs,    // For handling filtered document deletion
    on_delete_many_assets,      // For handling batch asset deletion
    on_delete_many_docs,        // For handling batch document deletion
    on_set_doc,                 // For handling document creation/update
    on_set_many_docs,           // For handling batch document creation/update
    on_upload_asset,            // For handling asset upload
};

// AVAILABLE IMPORTS FROM junobuild_satellite:
// These are the only context types and utilities available:
use junobuild_satellite::{
    include_satellite,              // Required macro for Juno integration
    AssertDeleteAssetContext,       // Context for asset deletion assertion
    AssertDeleteDocContext,         // Context for document deletion assertion
    AssertSetDocContext,            // Context for document creation/update assertion
    AssertUploadAssetContext,       // Context for asset upload assertion
    OnDeleteAssetContext,           // Context for asset deletion handler
    OnDeleteDocContext,             // Context for document deletion handler
    OnDeleteFilteredAssetsContext,  // Context for filtered asset deletion
    OnDeleteFilteredDocsContext,    // Context for filtered document deletion
    OnDeleteManyAssetsContext,      // Context for batch asset deletion
    OnDeleteManyDocsContext,        // Context for batch document deletion
    OnSetDocContext,                // Context for document creation/update
    OnSetManyDocsContext,           // Context for batch document creation/update
    OnUploadAssetContext,           // Context for asset upload handler
};

// IMPORTANT NOTE:
// Any additional functionality needed (like data serialization, string manipulation, etc.)
// must be either:
// 1. Imported from external crates (e.g., serde for serialization)
// 2. Implemented manually in our codebase
// 3. Or imported from junobuild_utils if available (needs to be verified)

// For example, we'll need to add:
// - serde for data serialization
// - Our own utility functions for things like username normalization
// - Any additional helper functions we need for our business logic

// All the available hooks and assertions for your Datastore and Storage are scaffolded by default in this `lib.rs` module.
// However, if you don't have to implement all of them, for example to improve readability or reduce unnecessary logic,
// you can selectively enable only the features you need.
//
// To do this, disable the default features in your `Cargo.toml` and explicitly specify only the ones you want to use.
//
// For example, if you only need `on_set_doc`, configure your `Cargo.toml` like this:
//
// [dependencies]
// junobuild-satellite = { version = "0.0.22", default-features = false, features = ["on_set_doc"] }
//
// With this setup, only `on_set_doc` must be implemented with custom logic,
// and other hooks and assertions can be removed. They will not be included in your Satellite.

#[on_set_doc]
async fn on_set_doc(_context: OnSetDocContext) -> Result<(), String> {
    Ok(())
}

#[on_set_many_docs]
async fn on_set_many_docs(_context: OnSetManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_doc]
async fn on_delete_doc(_context: OnDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_docs]
async fn on_delete_many_docs(_context: OnDeleteManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_filtered_docs]
async fn on_delete_filtered_docs(_context: OnDeleteFilteredDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_upload_asset]
async fn on_upload_asset(_context: OnUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_asset]
async fn on_delete_asset(_context: OnDeleteAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_assets]
async fn on_delete_many_assets(_context: OnDeleteManyAssetsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_filtered_assets]
async fn on_delete_filtered_assets(_context: OnDeleteFilteredAssetsContext) -> Result<(), String> {
    Ok(())
}
#[assert_set_doc]
fn assert_set_doc(_context: AssertSetDocContext) -> Result<(), String> {
    Ok(())
}

#[assert_delete_doc]
fn assert_delete_doc(_context: AssertDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[assert_upload_asset]
fn assert_upload_asset(_context: AssertUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[assert_delete_asset]
fn assert_delete_asset(_context: AssertDeleteAssetContext) -> Result<(), String> {
    Ok(())
}

include_satellite!();