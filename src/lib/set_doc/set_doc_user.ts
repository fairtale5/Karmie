import { formatUserKey } from '../keys/format_key_user';
import type { UserData, UserDocument } from '../types';
import type { ULID } from '../keys/ulid_types';
import { VALIDATE_USER_DOC } from '../settings';

/**
 * Creates a full user document object for setDoc, using the new key format.
 *
 * @param principal - The user's Internet Identity principal (string, from ic_cdk or auth library)
 * @param user_key - The user's ULID (ULID type, must be uppercase, 26 chars)
 * @param user_handle - The user's handle (string, required)
 * @param display_name - The user's display name (string, required)
 * @param avatar_url - Optional avatar URL (string)
 * @param description - Optional description (string)
 * @param validationOverride - Optional boolean to override global validation toggle
 * @returns {UserDocument} The full user document object, ready for setDoc
 *
 * This function:
 * - Gets the user_handle, display_name, and user_key (ULID) from the caller (frontend auth context)
 * - Uses formatUserKey to generate the document key in the format:
 *   _prn_{principal}_usr_{userULID}_hdl_{user_handle}_
 * - Assembles the UserData object, including all required and optional fields
 * - Returns the full UserDocument object, omitting owner and only including provided fields
 * - Optionally runs frontend validation if enabled in settings
 */
export default function setDocUser({
  principal,
  user_key,
  user_handle,
  display_name,
  avatar_url,
  description,
  validationOverride
}: {
  principal: string;
  user_key: ULID;
  user_handle: string;
  display_name: string;
  avatar_url?: string;
  description?: string;
  validationOverride?: boolean;
}): UserDocument {
  // Step 1: Generate the document key using the new format
  const key = formatUserKey(principal, user_key, user_handle);

  // Step 2: Assemble the UserData object, only including provided fields
  const data: UserData = {
    user_handle,
    display_name,
    user_key
  };
  if (avatar_url) data.avatar_url = avatar_url;

  // Step 3: Optionally run frontend validation if enabled
  const doValidate = validationOverride !== undefined ? validationOverride : VALIDATE_USER_DOC;
  if (doValidate) {
    // TODO: Implement frontend validation for user fields (user_handle, display_name, user_key)
    // For now, this is a stub
  }

  // Step 4: Return the full user document object, omitting owner and only including provided description
  const doc: UserDocument = {
    key,
    data,
    version: BigInt(0),
    created_at: BigInt(0),
    updated_at: BigInt(0)
  } as UserDocument;
  if (description) doc.description = description;
  return doc;
} 