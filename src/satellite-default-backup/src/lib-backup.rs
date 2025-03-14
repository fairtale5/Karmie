use junobuild_macros::{
    assert_delete_asset, assert_delete_doc, assert_set_doc, assert_upload_asset, on_delete_asset,
    on_delete_doc, on_delete_filtered_assets, on_delete_filtered_docs, on_delete_many_assets,
    on_delete_many_docs, on_set_doc, on_set_many_docs, on_upload_asset,
};
use junobuild_satellite::{
    include_satellite, AssertDeleteAssetContext, AssertDeleteDocContext, AssertSetDocContext,
    AssertUploadAssetContext, OnDeleteAssetContext, OnDeleteDocContext,
    OnDeleteFilteredAssetsContext, OnDeleteFilteredDocsContext, OnDeleteManyAssetsContext,
    OnDeleteManyDocsContext, OnSetDocContext, OnSetManyDocsContext, OnUploadAssetContext,
};

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


#[on_set_doc(collections = ["users"])]
async fn on_set_doc(context: OnSetDocContext) -> Result<(), String> {
    // Decode the document data to access the username
    let user_data: UserData = decode_doc_data(&context.data.data.after.data)?;

    // Create a normalized username for searching
    let normalized_username = normalize_username(&user_data.username);

    // Format the description with the search key prefix
    // Using a format that allows for future expansion with additional search keys
    // Format: "username:normalized_value;key2:value2;key3:value3"
    let description = format!("username:{}", normalized_username);

    // Update the document with the new description
    let doc = SetDoc {
        data: context.data.data.after.data,
        description: Some(description),
        version: context.data.data.after.version,
    };

    // Save the document with the updated description
    set_doc_store(
        context.caller,
        context.data.collection,
        context.data.key,
        doc,
    )?;

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


#[assert_set_doc(collections = ["users"])]
fn assert_set_doc(context: AssertSetDocContext) -> Result<(), String> {
    // Define our custom data structure that matches the payload
    #[derive(Deserialize)]
    struct UserData {
        username: String,
        displayName: String,
    }

    // Decode just the data payload from the document
    let user_data: UserData = decode_doc_data(&context.data.data)?;

    // Validate username
    if user_data.username.trim().is_empty() {
        return Err("Username cannot be empty".to_string());
    }

    if user_data.username.len() < 3 {
        return Err("Username must be at least 3 characters long".to_string());
    }

    if user_data.username.len() > 50 {
        return Err("Username cannot be longer than 50 characters".to_string());
    }

    // Username can only contain alphanumeric characters, underscores, and hyphens
    if !user_data.username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err("Username can only contain letters, numbers, underscores, and hyphens".to_string());
    }

    // Validate displayName
    if user_data.displayName.trim().is_empty() {
        return Err("Display name cannot be empty".to_string());
    }

    if user_data.displayName.len() > 100 {
        return Err("Display name cannot be longer than 100 characters".to_string());
    }

    // Check for username uniqueness using the description field
    let normalized_username = normalize_username(&user_data.username);
    let username_search = format!("username:{}", normalized_username);

    let matcher = ListMatcher {
        description: Some(username_search),
        ..Default::default()
    };

    let params = ListParams {
        matcher: Some(matcher),
        ..Default::default()
    };

    let existing_users = list_docs("users", params)?;

    // If we found any documents with this normalized username (except for our own document in case of updates)
    for doc in existing_users.items {
        if doc.0 != context.data.key {
            return Err("Username already exists (ignoring case and accents)".to_string());
        }
    }

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
