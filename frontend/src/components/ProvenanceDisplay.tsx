import { Card, Tag, Descriptions, Alert, Tooltip } from 'antd';
import { SafetyCertificateOutlined, WarningOutlined, CopyOutlined } from '@ant-design/icons';
import { useState } from 'react';
import type { C2PAMetadata } from '../types';

interface ProvenanceDisplayProps {
  metadata: C2PAMetadata | null;
  hasC2PA: boolean;
  imageHash: string | null;
}

const ProvenanceDisplay: React.FC<ProvenanceDisplayProps> = ({ metadata, hasC2PA, imageHash }) => {
  const [copied, setCopied] = useState(false);

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!hasC2PA) {
    return (
      <Card title="C2PA Provenance" extra={<WarningOutlined style={{ color: '#faad14' }} />}>
        <Alert
          message="No C2PA Metadata Found"
          description="This image does not contain C2PA provenance metadata. The image cannot be verified for authenticity."
          type="warning"
          showIcon
        />
      </Card>
    );
  }

  return (
    <Card
      title={
        <span>
          <SafetyCertificateOutlined style={{ color: '#52c41a', marginRight: 8 }} />
          C2PA Provenance
        </span>
      }
    >
      <Descriptions column={1} bordered size="small">
        <Descriptions.Item label="Status">
          <Tag color="green">Verified</Tag>
        </Descriptions.Item>
        <Descriptions.Item label="Algorithm">
          <Tag color="blue">{metadata?.algorithm || 'Unknown'}</Tag>
        </Descriptions.Item>
        <Descriptions.Item label="Claim Generator">
          {metadata?.claim_generator || 'Unknown'}
        </Descriptions.Item>
        {metadata?.device_info && (
          <>
            <Descriptions.Item label="Device Model">
              {metadata.device_info.model || 'Unknown'}
            </Descriptions.Item>
            <Descriptions.Item label="Manufacturer">
              {metadata.device_info.manufacturer || 'Unknown'}
            </Descriptions.Item>
          </>
        )}
        {metadata?.timestamp && (
          <Descriptions.Item label="Timestamp">
            {metadata.timestamp}
          </Descriptions.Item>
        )}
        <Descriptions.Item label="Image Hash">
          <Tooltip title={imageHash || ''}>
            <code
              style={{
                fontSize: 12,
                cursor: 'pointer',
                display: 'inline-flex',
                alignItems: 'center',
                gap: 4,
              }}
              onClick={() => imageHash && copyToClipboard(imageHash)}
            >
              {imageHash}
              <CopyOutlined style={{ fontSize: 10, color: '#888' }} />
              {copied && <Tag color="green" style={{ marginLeft: 4, fontSize: 10 }}>Copied!</Tag>}
            </code>
          </Tooltip>
        </Descriptions.Item>
      </Descriptions>
    </Card>
  );
};

export default ProvenanceDisplay;
