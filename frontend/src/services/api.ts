import axios from 'axios';
import type {
  ProvenanceResponse,
  EditResponse,
  CropParams,
  ResizeParams,
  RotateParams,
  ZKProofRequest,
  ZKProofResponse,
  EditingRecord,
} from '../types';

// Use localhost:8080 for the backend API
const API_BASE = 'http://localhost:8080/api/v1';

const apiClient = axios.create({
  baseURL: API_BASE,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Convert File to Base64
const fileToBase64 = (file: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = () => {
      const result = reader.result as string;
      // Remove data URL prefix
      resolve(result.split(',')[1]);
    };
    reader.onerror = reject;
  });
};

// Backend response types (from Rust API)
interface BackendProvenanceResponse {
  has_c2pa: boolean;
  c2pa_metadata?: {
    active_manifest: string;
    claim_generator: string;
    title?: string;
    format: string;
    signature_info?: {
      issuer?: string;
      time?: string;
      cert_serial_number?: string;
      alg?: string;
    };
    assertions: Array<{ label: string; data: unknown }>;
    ingredients: Array<{ title?: string; format?: string; relationship?: string }>;
  };
  image_hash: string;
  format: string;
}

interface BackendEditResponse {
  image: string;
  width: number;
  height: number;
  record: {
    operation: string;
    parameters: Record<string, unknown>;
    original_image_hash: string;
    edited_image_hash: string;
    timestamp: string;
  };
}

interface BackendZkProveResponse {
  proof_base64: string;
  public_inputs: {
    c2pa_verified: boolean;
    editing_verified: boolean;
    original_image_hash: string;
    edited_image_hash?: string;
    operations_applied: string[];
  };
  metadata: {
    prover_type: string;
    generation_time_ms: number;
    timestamp: string;
    proof_version: string;
  };
}

// Extract C2PA provenance from uploaded image
export const extractProvenance = async (file: File): Promise<ProvenanceResponse> => {
  const base64 = await fileToBase64(file);
  const response = await apiClient.post<BackendProvenanceResponse>('/provenance/extract', {
    image: base64,
  });

  const data = response.data;

  // Map to frontend type
  return {
    has_c2pa: data.has_c2pa,
    original_image_hash: data.image_hash,
    metadata: data.c2pa_metadata ? {
      active_manifest: data.c2pa_metadata.active_manifest,
      claim_generator: data.c2pa_metadata.claim_generator,
      algorithm: data.c2pa_metadata.signature_info?.alg || 'unknown',
    } : undefined,
  };
};

// Crop image
export const cropImage = async (
  imageBase64: string,
  params: CropParams
): Promise<EditResponse> => {
  const response = await apiClient.post<BackendEditResponse>('/edit/crop', {
    image: imageBase64,
    x: params.x,
    y: params.y,
    width: params.width,
    height: params.height,
  });

  const data = response.data;

  // Map to frontend type
  return {
    edited_image_base64: data.image,
    edited_image_hash: data.record.edited_image_hash,
    editing_record: {
      operation: data.record.operation as EditingRecord['operation'],
      parameters: data.record.parameters as unknown as EditingRecord['parameters'],
      input_hash: data.record.original_image_hash,
      output_hash: data.record.edited_image_hash,
    },
  };
};

// Resize image
export const resizeImage = async (
  imageBase64: string,
  params: ResizeParams
): Promise<EditResponse> => {
  const response = await apiClient.post<BackendEditResponse>('/edit/resize', {
    image: imageBase64,
    width: params.width,
    height: params.height,
  });

  const data = response.data;

  // Map to frontend type
  return {
    edited_image_base64: data.image,
    edited_image_hash: data.record.edited_image_hash,
    editing_record: {
      operation: data.record.operation as EditingRecord['operation'],
      parameters: data.record.parameters as unknown as EditingRecord['parameters'],
      input_hash: data.record.original_image_hash,
      output_hash: data.record.edited_image_hash,
    },
  };
};

// Rotate image
export const rotateImage = async (
  imageBase64: string,
  params: RotateParams
): Promise<EditResponse> => {
  const response = await apiClient.post<BackendEditResponse>('/edit/rotate', {
    image: imageBase64,
    angle: params.angle,
  });

  const data = response.data;

  // Map to frontend type
  return {
    edited_image_base64: data.image,
    edited_image_hash: data.record.edited_image_hash,
    editing_record: {
      operation: data.record.operation as EditingRecord['operation'],
      parameters: data.record.parameters as unknown as EditingRecord['parameters'],
      input_hash: data.record.original_image_hash,
      output_hash: data.record.edited_image_hash,
    },
  };
};

// Generate ZK Proof
export const generateProof = async (request: ZKProofRequest): Promise<ZKProofResponse> => {
  // Map to backend request format
  const backendRequest = {
    proof_type: 'combined',
    original_image_hash: request.original_image_hash,
    edited_image_hash: request.edited_image_hash,
    c2pa_data: request.c2pa_data ? {
      active_manifest: request.c2pa_data.active_manifest,
      claim_generator: request.c2pa_data.claim_generator,
      signature_info: request.c2pa_data.algorithm ? {
        alg: request.c2pa_data.algorithm,
      } : undefined,
    } : undefined,
    editing_records: request.editing_records.map(record => ({
      operation: record.operation.toLowerCase(),
      parameters: record.parameters,
      input_hash: record.input_hash,
      output_hash: record.output_hash,
    })),
  };

  const response = await apiClient.post<BackendZkProveResponse>('/zk/prove', backendRequest);

  const data = response.data;

  // Map to frontend type
  return {
    proof_base64: data.proof_base64,
    public_inputs: {
      c2pa_verified: data.public_inputs.c2pa_verified,
      editing_verified: data.public_inputs.editing_verified,
      operations_applied: data.public_inputs.operations_applied as ZKProofResponse['public_inputs']['operations_applied'],
    },
    metadata: {
      prover_type: data.metadata.prover_type,
      generation_time_ms: data.metadata.generation_time_ms,
      proof_size: data.proof_base64.length,
    },
  };
};

// Helper to convert base64 to blob for download
export const base64ToBlob = (base64: string, mimeType: string): Blob => {
  const byteCharacters = atob(base64);
  const byteNumbers = new Array(byteCharacters.length);
  for (let i = 0; i < byteCharacters.length; i++) {
    byteNumbers[i] = byteCharacters.charCodeAt(i);
  }
  const byteArray = new Uint8Array(byteNumbers);
  return new Blob([byteArray], { type: mimeType });
};

// Download file from blob
export const downloadBlob = (blob: Blob, filename: string): void => {
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
};
