import { Card, Tag, Button, Descriptions, Alert, Space, Divider } from 'antd';
import {
  CheckCircleOutlined,
  CloseCircleOutlined,
  DownloadOutlined,
} from '@ant-design/icons';
import type { ZKProofResponse } from '../types';
import { base64ToBlob, downloadBlob } from '../services/api';

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
    <Card title="Verification Result" style={{ marginTop: 16 }}>
      <Space direction="vertical" style={{ width: '100%' }} size="large">
        {/* Status Tags */}
        <Space>
          {public_inputs.c2pa_verified ? (
            <Tag icon={<CheckCircleOutlined />} color="success">
              C2PA Verified
            </Tag>
          ) : (
            <Tag icon={<CloseCircleOutlined />} color="error">
              C2PA Not Verified
            </Tag>
          )}
          {public_inputs.editing_verified ? (
            <Tag icon={<CheckCircleOutlined />} color="success">
              Editing Verified
            </Tag>
          ) : (
            <Tag icon={<CloseCircleOutlined />} color="error">
              Editing Not Verified
            </Tag>
          )}
        </Space>

        {/* Details */}
        <Descriptions column={2} bordered size="small">
          <Descriptions.Item label="Prover Type">
            {metadata.prover_type}
          </Descriptions.Item>
          <Descriptions.Item label="Generation Time">
            {metadata.generation_time_ms} ms
          </Descriptions.Item>
          <Descriptions.Item label="Proof Size">
            {metadata.proof_size} bytes
          </Descriptions.Item>
          <Descriptions.Item label="Operations Applied">
            {public_inputs.operations_applied.length}
          </Descriptions.Item>
        </Descriptions>

        {/* Operations List */}
        {public_inputs.operations_applied.length > 0 && (
          <div>
            <Divider>Applied Operations</Divider>
            <Space wrap>
              {public_inputs.operations_applied.map((op, index) => (
                <Tag key={index} color="blue">
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
          />
        )}

        {/* Download Buttons */}
        <Divider>Downloads</Divider>
        <Space>
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
        </Space>
      </Space>
    </Card>
  );
};

export default VerificationResult;
