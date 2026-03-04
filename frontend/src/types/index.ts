// Types for API requests and responses

export interface C2PAMetadata {
  active_manifest: string;
  claim_generator: string;
  algorithm: string;
  timestamp?: string;
  device_info?: {
    model?: string;
    manufacturer?: string;
  };
}

export interface ProvenanceResponse {
  has_c2pa: boolean;
  metadata?: C2PAMetadata;
  original_image_hash: string;
  original_image_base64?: string;
}

export interface CropParams {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface ResizeParams {
  width: number;
  height: number;
}

export interface RotateParams {
  angle: 90 | 180 | 270;
}

export type EditOperation = 'Crop' | 'Resize' | 'Rotate';

export interface EditingRecord {
  operation: EditOperation;
  parameters: CropParams | ResizeParams | RotateParams;
  input_hash: string;
  output_hash: string;
  // Raw pixels for ZK proof (private input)
  raw_pixels?: string;
  pixel_width?: number;
  pixel_height?: number;
}

export interface EditResponse {
  edited_image_base64: string;
  editing_record: EditingRecord;
  edited_image_hash: string;
  raw_pixels?: string;
}

export interface ZKProofRequest {
  original_image_hash: string;
  edited_image_hash: string;
  c2pa_data?: C2PAMetadata;
  editing_records: EditingRecord[];
  original_image?: string; // Base64 encoded original image for hash verification
}

export interface ZKProofResponse {
  proof_base64: string;
  public_inputs: {
    c2pa_verified: boolean;
    editing_verified: boolean;
    operations_applied: EditOperation[];
  };
  metadata: {
    prover_type: string;
    generation_time_ms: number;
    proof_size: number;
  };
}

export interface AppState {
  // Image state
  originalImage: string | null;
  originalImageHash: string | null;
  editedImage: string | null;
  editedImageHash: string | null;

  // C2PA state
  c2paMetadata: C2PAMetadata | null;
  hasC2PA: boolean;

  // Editing state
  editingRecords: EditingRecord[];

  // Proof state
  proof: ZKProofResponse | null;
  isGeneratingProof: boolean;

  // UI state
  currentStep: 'upload' | 'edit' | 'proof';
  error: string | null;
}
