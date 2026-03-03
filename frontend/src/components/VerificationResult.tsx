import { Card, Tag, Button, Descriptions, Alert, Space, Typography } from 'antd';
import {
  CheckCircleOutlined,
  CloseCircleOutlined,
  DownloadOutlined,
} from '@ant-design/icons';
import type { ZKProofResponse } from '../types';
import { base64ToBlob, downloadBlob } from '../services/api';

const { Text } = Typography;

interface VerificationResultProps {
  proof: ZKProofResponse | null;
  editedImageBase64: string | null;
}

const VerificationResult: React.FC<VerificationResultProps> = ({ proof, editedImageBase64 }) => {
  if (!proof) {
    return null;
  }

  const { public_inputs, metadata } = proof;

  const handleDownloadImage = () => {
    if (editedImageBase64) {
      const blob = base64ToBlob(editedImageBase64, 'image/jpeg');
      downloadBlob(blob, 'edited_image.jpg');
    }
  };

  const handleDownloadProof = () => {
    const blob = base64ToBlob(proof.proof_base64, 'application/octet-stream');
    downloadBlob(blob, 'zk_proof.bin');
  };

  return (
    <Card
      title={<span className="section-title">Verification Result</span>}
      style={{
        boxShadow: '0 1px 3px rgba(0,0,0,0.05), 0 1px 2px rgba(0,0,0,0.03)',
      }}
    >
      <Space direction="vertical" style={{ width: '100%' }} size="large">
        {/* Status Tags */}
        <div style={{ display: 'flex', gap: 12, flexWrap: 'wrap' }}>
          <div
            className="status-badge"
            style={{
              background: public_inputs.c2pa_verified ? '#d1fae5' : '#fee2e2',
              color: public_inputs.c2pa_verified ? '#10b981' : '#ef4444',
            }}
          >
            {public_inputs.c2pa_verified ? (
              <CheckCircleOutlined />
            ) : (
              <CloseCircleOutlined />
            )}
            C2PA Verified
          </div>
          <div
            className="status-badge"
            style={{
              background: public_inputs.editing_verified ? '#d1fae5' : '#fee2e2',
              color: public_inputs.editing_verified ? '#10b981' : '#ef4444',
            }}
          >
            {public_inputs.editing_verified ? (
              <CheckCircleOutlined />
            ) : (
              <CloseCircleOutlined />
            )}
            Editing Verified
          </div>
        </div>

        {/* Details */}
        <Descriptions column={2} bordered size="small">
          <Descriptions.Item label={<span className="label-tag">Prover Type</span>}>
            <Text>{metadata.prover_type}</Text>
          </Descriptions.Item>
          <Descriptions.Item label={<span className="label-tag">Generation Time</span>}>
            <Text>{metadata.generation_time_ms} ms</Text>
          </Descriptions.Item>
          <Descriptions.Item label={<span className="label-tag">Proof Size</span>}>
            <Text>{metadata.proof_size} bytes</Text>
          </Descriptions.Item>
          <Descriptions.Item label={<span className="label-tag">Operations Applied</span>}>
            <Text>{public_inputs.operations_applied.length}</Text>
          </Descriptions.Item>
        </Descriptions>

        {/* Operations List */}
        {public_inputs.operations_applied.length > 0 && (
          <div>
            <Text type="secondary" strong style={{ display: 'block', marginBottom: 12 }}>
              Applied Operations
            </Text>
            <Space wrap>
              {public_inputs.operations_applied.map((op, index) => (
                <Tag
                  key={index}
                  style={{
                    background: '#eef2ff',
                    color: '#4f46e5',
                    border: 'none',
                    borderRadius: 4,
                    fontWeight: 500,
                    padding: '4px 12px',
                  }}
                >
                  {op}
                </Tag>
              ))}
            </Space>
          </div>
        )}

        {/* Success Alert */}
        {public_inputs.c2pa_verified && public_inputs.editing_verified && (
          <Alert
            message="Verification Successful"
            description="The image has been verified. The C2PA provenance is valid and the editing operations have been proven without revealing sensitive details."
            type="success"
            showIcon
            style={{ borderRadius: 6 }}
          />
        )}

        {/* Download Buttons */}
        <div style={{
          display: 'flex',
          gap: 12,
          paddingTop: 16,
          borderTop: '1px solid #f3f4f6',
        }}>
          <Button
            type="primary"
            icon={<DownloadOutlined />}
            onClick={handleDownloadImage}
            disabled={!editedImageBase64}
          >
            Download Edited Image
          </Button>
          <Button
            icon={<DownloadOutlined />}
            onClick={handleDownloadProof}
          >
            Download ZK Proof
          </Button>
        </div>
      </Space>
    </Card>
  );
};

export default VerificationResult;
