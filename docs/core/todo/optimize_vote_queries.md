# Vote Query Optimization Plan

## Current Issue

We have two common query patterns for votes:
1. Find all votes BY a user in a specific tag
2. Find all votes FOR a user in a specific tag

Currently, our description format is:
```
owner=authorKey;target=targetKey;tag=tagKey;
```

With this format, we can easily query votes TO a user in a key (by matching `target=targetKey;tag=tagKey;`), but to find votes FOR a user we have issues. `owner=authorKey;target=targetKey;tag=tagKey;` between the owner and tag is the target, and that changes for each vote.
So currently as a workaround we:
1. Query a larger set of votes (or all votes)
2. Then filter them for the target user in our code

This is inefficient when the dataset grows large.

## Simple Solution: Invert Field Order in Description

Change the description format to:
```
owner=authorKey;tag=tagKey;target=targetKey;
```

This simple change has these benefits:
1. We can query by `owner=authorKey;tag=tagKey;` for votes BY a user in a tag
2. We can query by `tag=tagKey;target=targetKey;` for votes FOR a user in a tag
3. No complex regex or special query logic needed
4. Just standard exact string matching

## Implementation Steps

1. Update the vote description format in `on_set_doc` hook:
   ```rust
   // In on_set_doc hook for votes collection
   fn on_set_doc_votes(doc: &mut Document) -> Result<(), String> {
       if let Some(data) = doc.data.as_object_mut() {
           let author_key = data.get("author_key").and_then(|v| v.as_str()).unwrap_or("");
           let target_key = data.get("target_key").and_then(|v| v.as_str()).unwrap_or("");
           let tag_key = data.get("tag_key").and_then(|v| v.as_str()).unwrap_or("");
           
           // Put tag first, then author, then target
           doc.description = format!("tag={};owner={};target={};", 
               tag_key, author_key, target_key);
       }
       Ok(())
   }
   ```

2. Update queries to use this new format:
   ```rust
   // Votes BY a user in a tag
   let description_filter = format!("tag={};owner={};", tag_key, user_key);
   
   // Votes FOR a user in a tag
   let description_filter = format!("tag={};target={};", tag_key, user_key);
   ```

## Migration Plan

1. **Create a migration script** to update all existing vote descriptions
2. **Update the hooks** that generate descriptions for new votes
3. **Update query logic** to use the new format

## Required Changes

1. Update `src/satellite/src/lib.rs`:
   - Modify the `on_set_doc` hook for "votes" collection

2. Update `src/satellite/src/utils/reputation_calculations.rs`:
   - Update `calculate_and_store_vote_weight` to use the new query pattern
   - Update any other functions that query votes

3. Update `src/routes/admin/+page.svelte`:
   - Ensure frontend queries use the new pattern

## Benefits

1. **Simpler queries**: Use exact string matching instead of complex filtering
2. **More efficient**: Transfer less data over the network
3. **Better performance**: Especially noticeable with large datasets

## Testing

1. Test that existing votes can still be found after migration
2. Test that new votes are created with the correct description format
3. Test that both query patterns (BY user and FOR user) work correctly 