# Avatar Media Cropper Redesign

## Overview
Redesign the avatar selection and cropping flow to support both images and videos, with a modern, flexible UI and robust UX. This will allow users to set their avatar via file upload (dropzone or file select) or by providing a URL, and will support cropping for both media types.

---

## Requirements

- **Tabbed interface**: Two options for avatar selection:
  - **Upload**: Drag-and-drop area or file select button
  - **URL**: Input field to fetch media from a remote URL
- **Supported media types**: Images (`.png`, `.jpg`, `.jpeg`, `.webp`, `.svg`, `.gif` — including animated GIFs) and videos (`.mp4`)
- **Loader**: Show a loading/progress indicator (e.g., Skeleton ProgressRing) when fetching media from a URL
- **Shared variable**: Use a single variable (e.g., `avatarMedia`) as the source of truth for the selected media, shared between upload and cropper
- **State-driven UI**:
  - When `avatarMedia` is null, show the upload interface
  - When set, show the cropper interface
- **Cropper integration**: Use [`svelte-crop-window`](https://github.com/sabine/svelte-crop-window) for both images and videos
- **Cropping logic**:
  - For images: After cropping, convert to `.webp` (as currently implemented)
  - For videos: After cropping, use [`ffmpeg.wasm`](https://github.com/ffmpegwasm/ffmpeg.wasm?tab=readme-ov-file) to crop and process the video
- **File size limits**:
  - Enforce a maximum file size for both images and videos (TBD: recommend a value, e.g., 20MB for images, 50MB for videos?)
- **File size display**:
  - Show file size as the file is uploaded/fetched
  - After cropping, show the new (reduced) file size
- **Upload logic**:
  - After cropping, upload the processed file using `setDoc` and set it as the user's avatar
- **UX**:
  - Only one interface (upload or cropper) is visible at a time
  - All transitions are user-driven, no recursion or infinite loops
  - Accessibility and keyboard navigation are supported

---

## Flow Summary
1. **User chooses tab**: "Upload" or "URL"
2. **User selects or pastes file**: File is validated (type/size), loader shown if fetching from URL
3. **File is set to shared variable**: Upload UI disappears, cropper appears
4. **User crops media**: Cropper interface for image or video
5. **After crop**:
    - For images: Convert to `.webp`, show new file size
    - For videos: Crop/process with ffmpeg.wasm, show new file size
6. **Upload**: Upload cropped file and set as avatar

---

## References
- [svelte-crop-window](https://github.com/sabine/svelte-crop-window)
- [ffmpeg.wasm](https://github.com/ffmpegwasm/ffmpeg.wasm?tab=readme-ov-file)

---

## Open Questions
- What is the recommended maximum file size for images and videos?
- What video formats (besides .mp4) should be supported?
- Should we allow users to skip cropping for videos?
- What fallback or error UI should be shown if cropping or upload fails? 