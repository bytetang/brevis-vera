import { Card, Tag, Descriptions, Alert, Tooltip, Typography, Space } from 'antd';
import {
  WarningOutlined,
  CopyOutlined,
  CheckCircleOutlined,
} from '@ant-design/icons';
import { useState } from 'react';
import type { C2PAMetadata } from '../types';

const { Text } = Typography;

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
      <Card
        title={
          <Space>
            <WarningOutlined style={{ color: '#f59e0b' }} />
            <span>C2PA Provenance</span>
          </Space>
        }
        style={{
          marginTop: 24,
          boxShadow: '0 1px 3px rgba(0,0,0,0.05), 0 1px 2px rgba(0,0,0,0.03)',
          borderColor: '#f59e0b',
        }}
      >
        <Alert
          message="No C2PA Metadata Found"
          description="This image does not contain C2PA provenance metadata. The image cannot be verified for authenticity."
          type="warning"
          showIcon
          style={{ borderRadius: 6 }}
        />
      </Card>
    );
  }

  return (
    <Card
      title={
        <Space>
          <CheckCircleOutlined style={{ color: '#10b981' }} />
          <span>C2PA Provenance</span>
        </Space>
      }
      style={{
        marginTop: 24,
        boxShadow: '0 1px 3px rgba(0,0,0,0.05), 0 1px 2px rgba(0,0,0,0.03)',
      }}
    >
      <Space direction="vertical" size="middle" style={{ width: '100%' }}>
        {/* Status */}
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <span className="label-tag">Status</span>
          <Tag color="success" style={{
            background: '#d1fae5',
            color: '#10b981',
            border: 'none',
            borderRadius: 4,
            fontWeight: 500,
          }}>
            Verified
          </Tag>
        </div>

        <Descriptions column={1} bordered size="small" style={{ marginTop: 16 }}>
          <Descriptions.Item label={<span className="label-tag">Algorithm</span>}>
            <Tag style={{
              background: '#eef2ff',
              color: '#4f46e5',
              border: 'none',
              borderRadius: 4,
            }}>
              {metadata?.algorithm || 'Unknown'}
            </Tag>
          </Descriptions.Item>
          <Descriptions.Item label={<span className="label-tag">Claim Generator</span>}>
            <Text>{metadata?.claim_generator || 'Unknown'}</Text>
          </Descriptions.Item>
          {metadata?.device_info && (
            <>
              <Descriptions.Item label={<span className="label-tag">Device Model</span>}>
                <Text>{metadata.device_info.model || 'Unknown'}</Text>
              </Descriptions.Item>
              <Descriptions.Item label={<span className="label-tag">Manufacturer</span>}>
                <Text>{metadata.device_info.manufacturer || 'Unknown'}</Text>
              </Descriptions.Item>
            </>
          )}
          {metadata?.timestamp && (
            <Descriptions.Item label={<span className="label-tag">Timestamp</span>}>
              <Text>{metadata.timestamp}</Text>
            </Descriptions.Item>
          )}
          <Descriptions.Item label={<span className="label-tag">Image Hash</span>}>
            <Tooltip title={imageHash || ''}>
              <code
                className="hash-display"
                onClick={() => imageHash && copyToClipboard(imageHash)}
                style={{
                  fontFamily: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
                  fontSize: 11,
                  cursor: 'pointer',
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: 6,
                  padding: '4px 8px',
                  borderRadius: 4,
                  background: '#f9fafb',
                  color: '#4b5563',
                  maxWidth: '100%',
                  overflow: 'hidden',
                  textOverflow: 'ellipsis',
                  whiteSpace: 'nowrap',
                }}
              >
                {imageHash}
                <CopyOutlined style={{ fontSize: 10, color: '#9ca3af', flexShrink: 0 }} />
              </code>
            </Tooltip>
            {copied && (
              <Tag
                color="success"
                style={{
                  background: '#d1fae5',
                  color: '#10b981',
                  border: 'none',
                  borderRadius: 4,
                  marginLeft: 8,
                  fontSize: 11,
                }}
              >
                Copied!
              </Tag>
            )}
          </Descriptions.Item>
        </Descriptions>
      </Space>
    </Card>
  );
};

export default ProvenanceDisplay;
