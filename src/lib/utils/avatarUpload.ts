import { uploadFile } from '@junobuild/core';

/**
 * Uploads an avatar file to Juno Storage
 * @param file - The file to upload
 * @param filename - The filename to use for storage
 * @returns Promise that resolves to the download URL
 */
export async function uploadAvatarFile(file: File, filename: string): Promise<string> {
  const result = await uploadFile({
    data: file,
    collection: 'user_avatars',
    filename: filename
  });
  
  return result.downloadUrl || result.fullPath || '';
} 