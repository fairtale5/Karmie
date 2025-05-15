<script lang="ts">
/**
 * AvatarCropper.svelte
 *
 * A reusable component for uploading and cropping profile avatars.
 *
 * - FileUpload is always visible.
 * - Cropper appears below FileUpload when a file is selected.
 * - Clicking 'Crop' only confirms the crop and generates a preview, but does NOT upload.
 * - The cropped image Blob is passed to the parent via the 'cropped' prop.
 * - The parent is responsible for uploading the Blob when saving the profile.
 * - Preview is shown below the cropper after cropping.
 *
 * References:
 * - svelte-easy-crop: https://github.com/ValentinH/svelte-easy-crop
 */
import Cropper from 'svelte-easy-crop';
import { Slider, FileUpload } from '@skeletonlabs/skeleton-svelte';
import { toaster } from '../skeletonui/toaster-skeleton';
import { XCircle } from 'lucide-svelte';

/**
 * Props
 * @prop {string} principal - The user's principal (unique user ID), used for deterministic avatar filename.
 * @prop {string} initialUrl - Initial avatar URL for preview (empty string if no avatar).
 * @prop {(blob: Blob | null) => void} cropped - Callback to parent with cropped image Blob (or null if removed).
 * @prop {(url: string) => void} change - Callback to parent with preview URL (empty string if removed).
 */
export let principal: string;
export let initialUrl = '';
export let cropped: (blob: Blob | null) => void = () => {};
export let change: (value: string) => void = () => {};

const MAX_SIZE_MB = 20;
const ACCEPTED_TYPES = [
  'image/png',
  'image/jpeg',
  'image/webp',
  'image/svg+xml',
  'image/gif'
];

let selectedFile: File | null = null;
let imageUrl: string | null = initialUrl;
let crop = { x: 0, y: 0 };
let zoom = 1;
let croppedAreaPixels: { x: number; y: number; width: number; height: number } | null = null;
let showCropper = false;
let previewUrl: string | null = null;
let previewBlob: Blob | null = null;
let zoomArr = [1]; // For Slider value binding

function onFileUploadChange(details: { acceptedFiles: File[] }) {
  const files = details.acceptedFiles ?? [];
  if (!files.length) return;
  const file = files[0];
  if (!ACCEPTED_TYPES.includes(file.type)) {
    toaster.error({ title: 'Unsupported file type', description: 'Please select a PNG, JPEG, WEBP, SVG, or GIF image.' });
    return;
  }
  if (file.size > MAX_SIZE_MB * 1024 * 1024) {
    toaster.error({ title: 'File too large', description: `Max size is ${MAX_SIZE_MB}MB.` });
    return;
  }
  selectedFile = file;
  const reader = new FileReader();
  reader.onload = (ev) => {
    imageUrl = ev.target?.result as string;
    showCropper = true;
    previewUrl = null;
    previewBlob = null;
    cropped(null); // Reset parent blob
  };
  reader.readAsDataURL(file);
}

function onRemove() {
  selectedFile = null;
  imageUrl = null;
  showCropper = false;
  previewUrl = null;
  previewBlob = null;
  cropped(null);
  change('');  // Use empty string instead of null
}

function onCropComplete(areaPixels: { x: number; y: number; width: number; height: number }) {
  croppedAreaPixels = areaPixels;
}

function onSliderChange(details: { value: number[] }) {
  zoom = details.value[0];
  zoomArr = [zoom];
}

// Only confirm crop and generate preview, do NOT upload
async function confirmCrop() {
  if (!imageUrl || !croppedAreaPixels) return;
  try {
    const croppedBlob = await getCroppedImg(imageUrl, croppedAreaPixels);
    if (croppedBlob.size > MAX_SIZE_MB * 1024 * 1024) {
      toaster.error({ title: 'Cropped image too large', description: `Max size is ${MAX_SIZE_MB}MB.` });
      return;
    }
    previewBlob = croppedBlob;
    const previewUrlLocal = URL.createObjectURL(croppedBlob);
    previewUrl = previewUrlLocal;
    showCropper = false;
    cropped(croppedBlob); // Pass blob to parent
    change(previewUrlLocal); // Pass preview URL to parent
    toaster.success({ title: 'Crop confirmed', description: 'Your avatar crop is ready to save.' });
  } catch (e) {
    toaster.error({ title: 'Crop failed', description: e instanceof Error ? e.message : 'Unknown error.' });
  }
}

/**
 * Helper: Crop and convert image to webp using canvas.
 * @param {string} imageSrc - The image data URL
 * @param {object} crop - Cropped area in pixels
 * @returns {Promise<Blob>} - Cropped image as webp Blob
 */
async function getCroppedImg(imageSrc: string, crop: { x: number; y: number; width: number; height: number }): Promise<Blob> {
  return new Promise((resolve, reject) => {
    const image = new window.Image();
    image.onload = () => {
      const canvas = document.createElement('canvas');
      canvas.width = crop.width;
      canvas.height = crop.height;
      const ctx = canvas.getContext('2d');
      if (!ctx) return reject(new Error('Failed to get canvas context'));
      ctx.drawImage(
        image,
        crop.x,
        crop.y,
        crop.width,
        crop.height,
        0,
        0,
        crop.width,
        crop.height
      );
      canvas.toBlob(
        (blob) => {
          if (!blob) return reject(new Error('Failed to create blob'));
          resolve(blob);
        },
        'image/webp',
        0.95
      );
    };
    image.onerror = reject;
    image.src = imageSrc;
  });
}
</script>

<div class="avatar-cropper space-y-2">
  <!-- FileUpload dropzone (always visible) -->
  <FileUpload
    name="avatar"
    accept={{
      "image/png": [".png"],
      "image/jpeg": [".jpg", ".jpeg"],
      "image/webp": [".webp"],
      "image/svg+xml": [".svg"],
      "image/gif": [".gif"]
    }}
    maxFiles={1}
    classes="w-full"
    onFileChange={onFileUploadChange}
    onFileReject={(err) => toaster.error({ title: 'File rejected', description: err })}
  />
</div>

<!-- Cropper (only visible when a file is selected) -->
{#if showCropper && imageUrl}
  <div class="mt-4">
    <div class="relative w-full aspect-square bg-surface-200-800 rounded overflow-hidden">
      <Cropper
        image={imageUrl}
        crop={crop}
        zoom={zoom}
        aspect={1}
        cropShape="rect"
        showGrid={false}
        oncropcomplete={({ pixels }) => onCropComplete(pixels)}
      />
      <button type="button" class="absolute top-2 right-2 z-10" on:click={onRemove} aria-label="Remove image">
        <XCircle class="text-error-500 w-6 h-6" />
      </button>
    </div>
    <div class="flex items-center gap-4 mt-2">
      <span class="text-xs">Zoom</span>
      <Slider min={1} max={3} step={0.01} value={zoomArr} onValueChange={onSliderChange} />
      <button type="button" class="btn btn-sm preset-filled-primary-500" on:click={confirmCrop}>
        Crop
      </button>
    </div>
  </div>
{/if}

<!-- Preview (only visible after cropping) -->
{#if previewUrl}
  <div class="relative w-24 h-24 mx-auto mt-2">
    <img src={previewUrl} alt="Avatar preview" class="rounded-full w-full h-full object-cover" />
    <button type="button" class="absolute top-1 right-1 z-10" on:click={onRemove} aria-label="Remove image">
      <XCircle class="text-error-500 w-5 h-5" />
    </button>
  </div>
{/if} 