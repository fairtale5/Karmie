/*! 
 * Document validation entry point for assert_set_doc hooks in Reputator.
 *
 * This module re-exports validation functions for all supported document types:
 * - Users
 * - Votes
 * - Tags
 * - Reputations
 * 
 * Each function is called by the main assert_set_doc hook to validate
 * documents before they are created or updated in the database.
 */

mod assert_doc_user;
mod assert_doc_vote;
mod assert_doc_tag;
mod assert_doc_reputation;

pub use assert_doc_user::validate_user_document;
pub use assert_doc_vote::validate_vote_document;
pub use assert_doc_tag::validate_tag_document;
pub use assert_doc_reputation::validate_reputation_document;
