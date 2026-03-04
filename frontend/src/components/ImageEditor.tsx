import { useState } from 'react';
import { Button, InputNumber, Space, Spin, message, Typography } from 'antd';
import { ScissorOutlined, ExpandOutlined, SwapOutlined } from '@ant-design/icons';
import type { EditingRecord } from '../types';
import { cropImage, resizeImage, rotateImage } from '../services/api';

const { Text } = Typography;

interface ImageEditorProps {
  imageBase64: string;
  onImageEdited: (editedBase64: string, record: EditingRecord) => void;
  disabled?: boolean;
}

const ImageEditor: React.FC<ImageEditorProps> = ({ imageBase64, onImageEdited, disabled }) => {
  const [loading, setLoading] = useState(false);
  const [cropParams, setCropParams] = useState({ x: 432, y: 288, width: 3456, height: 2304 });
  const [resizeParams, setResizeParams] = useState({ width: 800, height: 600 });

  const handleCrop = async () => {
    setLoading(true);
    try {
      const response = await cropImage(imageBase64, cropParams);
      message.success('Image cropped successfully');
      onImageEdited(response.edited_image_base64, response.editing_record);
    } catch (error) {
      message.error('Crop failed');
    } finally {
      setLoading(false);
    }
  };

  const handleResize = async () => {
    setLoading(true);
    try {
      const response = await resizeImage(imageBase64, resizeParams);
      message.success('Image resized successfully');
      onImageEdited(response.edited_image_base64, response.editing_record);
    } catch (error) {
      message.error('Resize failed');
    } finally {
      setLoading(false);
    }
  };

  const handleRotate = async (angle: 90 | 180 | 270) => {
    setLoading(true);
    try {
      const response = await rotateImage(imageBase64, { angle });
      message.success(`Image rotated ${angle} degrees`);
      onImageEdited(response.edited_image_base64, response.editing_record);
    } catch (error) {
      message.error('Rotate failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Spin spinning={loading}>
      <Space direction="vertical" style={{ width: '100%' }} size="large">
        {/* Crop Tool */}
        <div className="tool-card">
          <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 16 }}>
            <div style={{
              width: 32,
              height: 32,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              background: '#eef2ff',
              borderRadius: 6,
            }}>
              <ScissorOutlined style={{ color: '#4f46e5' }} />
            </div>
            <Text strong>Crop</Text>
          </div>
          <Space wrap style={{ width: '100%' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
              <Text type="secondary" style={{ fontSize: 12 }}>X:</Text>
              <InputNumber
                min={0}
                value={cropParams.x}
                onChange={(v) => setCropParams({ ...cropParams, x: v || 0 })}
                disabled={disabled}
                style={{ width: 70 }}
                size="small"
              />
            </div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
              <Text type="secondary" style={{ fontSize: 12 }}>Y:</Text>
              <InputNumber
                min={0}
                value={cropParams.y}
                onChange={(v) => setCropParams({ ...cropParams, y: v || 0 })}
                disabled={disabled}
                style={{ width: 70 }}
                size="small"
              />
            </div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
              <Text type="secondary" style={{ fontSize: 12 }}>W:</Text>
              <InputNumber
                min={1}
                value={cropParams.width}
                onChange={(v) => setCropParams({ ...cropParams, width: v || 1 })}
                disabled={disabled}
                style={{ width: 70 }}
                size="small"
              />
            </div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
              <Text type="secondary" style={{ fontSize: 12 }}>H:</Text>
              <InputNumber
                min={1}
                value={cropParams.height}
                onChange={(v) => setCropParams({ ...cropParams, height: v || 1 })}
                disabled={disabled}
                style={{ width: 70 }}
                size="small"
              />
            </div>
            <Button
              type="primary"
              onClick={handleCrop}
              disabled={disabled}
              icon={<ScissorOutlined />}
              size="small"
            >
              Apply
            </Button>
          </Space>
        </div>

        {/* Resize Tool */}
        <div className="tool-card">
          <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 16 }}>
            <div style={{
              width: 32,
              height: 32,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              background: '#eef2ff',
              borderRadius: 6,
            }}>
              <ExpandOutlined style={{ color: '#4f46e5' }} />
            </div>
            <Text strong>Resize</Text>
          </div>
          <Space wrap style={{ width: '100%' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
              <Text type="secondary" style={{ fontSize: 12 }}>Width:</Text>
              <InputNumber
                min={1}
                value={resizeParams.width}
                onChange={(v) => setResizeParams({ ...resizeParams, width: v || 800 })}
                disabled={disabled}
                style={{ width: 80 }}
                size="small"
              />
            </div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
              <Text type="secondary" style={{ fontSize: 12 }}>Height:</Text>
              <InputNumber
                min={1}
                value={resizeParams.height}
                onChange={(v) => setResizeParams({ ...resizeParams, height: v || 600 })}
                disabled={disabled}
                style={{ width: 80 }}
                size="small"
              />
            </div>
            <Button
              type="primary"
              onClick={handleResize}
              disabled={disabled}
              icon={<ExpandOutlined />}
              size="small"
            >
              Apply
            </Button>
          </Space>
        </div>

        {/* Rotate Tool */}
        <div className="tool-card">
          <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 16 }}>
            <div style={{
              width: 32,
              height: 32,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              background: '#eef2ff',
              borderRadius: 6,
            }}>
              <SwapOutlined style={{ color: '#4f46e5' }} />
            </div>
            <Text strong>Rotate</Text>
          </div>
          <Space>
            <Button
              onClick={() => handleRotate(90)}
              disabled={disabled}
              size="small"
            >
              90°
            </Button>
            <Button
              onClick={() => handleRotate(180)}
              disabled={disabled}
              size="small"
            >
              180°
            </Button>
            <Button
              onClick={() => handleRotate(270)}
              disabled={disabled}
              size="small"
            >
              270°
            </Button>
          </Space>
        </div>
      </Space>
    </Spin>
  );
};

export default ImageEditor;
