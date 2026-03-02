import { useState } from 'react';
import { Card, Button, InputNumber, Space, Divider, Spin, message } from 'antd';
import { ScissorOutlined, ExpandOutlined, SwapOutlined } from '@ant-design/icons';
import type { EditingRecord } from '../types';

interface ImageEditorProps {
  imageBase64: string;
  onImageEdited: (editedBase64: string, record: EditingRecord) => void;
  disabled?: boolean;
}

const ImageEditor: React.FC<ImageEditorProps> = ({ imageBase64, onImageEdited, disabled }) => {
  const [loading, setLoading] = useState(false);
  const [cropParams, setCropParams] = useState({ x: 0, y: 0, width: 100, height: 100 });
  const [resizeParams, setResizeParams] = useState({ width: 800, height: 600 });

  // Mock editing functions - in real implementation, call backend API
  const handleCrop = async () => {
    setLoading(true);
    try {
      // Simulate API call
      await new Promise((resolve) => setTimeout(resolve, 500));
      // In real implementation: call cropImage API
      const mockRecord: EditingRecord = {
        operation: 'Crop',
        parameters: cropParams,
        input_hash: 'mock_input_hash',
        output_hash: 'mock_output_hash',
      };
      message.success('Image cropped successfully');
      // Pass back mock result - in real app, get edited image from API
      onImageEdited(imageBase64, mockRecord);
    } catch (error) {
      message.error('Crop failed');
    } finally {
      setLoading(false);
    }
  };

  const handleResize = async () => {
    setLoading(true);
    try {
      await new Promise((resolve) => setTimeout(resolve, 500));
      const mockRecord: EditingRecord = {
        operation: 'Resize',
        parameters: resizeParams,
        input_hash: 'mock_input_hash',
        output_hash: 'mock_output_hash',
      };
      message.success('Image resized successfully');
      onImageEdited(imageBase64, mockRecord);
    } catch (error) {
      message.error('Resize failed');
    } finally {
      setLoading(false);
    }
  };

  const handleRotate = async (angle: 90 | 180 | 270) => {
    setLoading(true);
    try {
      await new Promise((resolve) => setTimeout(resolve, 500));
      const mockRecord: EditingRecord = {
        operation: 'Rotate',
        parameters: { angle },
        input_hash: 'mock_input_hash',
        output_hash: 'mock_output_hash',
      };
      message.success(`Image rotated ${angle} degrees`);
      onImageEdited(imageBase64, mockRecord);
    } catch (error) {
      message.error('Rotate failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Spin spinning={loading}>
      <Card title="Image Editor" style={{ marginTop: 16 }}>
        <Space direction="vertical" style={{ width: '100%' }} size="large">
          {/* Crop Tool */}
          <div>
            <Divider orientation="left">
              <ScissorOutlined /> Crop
            </Divider>
            <Space wrap>
              <label>X:</label>
              <InputNumber
                min={0}
                value={cropParams.x}
                onChange={(v) => setCropParams({ ...cropParams, x: v || 0 })}
                disabled={disabled}
              />
              <label>Y:</label>
              <InputNumber
                min={0}
                value={cropParams.y}
                onChange={(v) => setCropParams({ ...cropParams, y: v || 0 })}
                disabled={disabled}
              />
              <label>Width:</label>
              <InputNumber
                min={1}
                value={cropParams.width}
                onChange={(v) => setCropParams({ ...cropParams, width: v || 100 })}
                disabled={disabled}
              />
              <label>Height:</label>
              <InputNumber
                min={1}
                value={cropParams.height}
                onChange={(v) => setCropParams({ ...cropParams, height: v || 100 })}
                disabled={disabled}
              />
              <Button
                type="primary"
                onClick={handleCrop}
                disabled={disabled}
                icon={<ScissorOutlined />}
              >
                Apply Crop
              </Button>
            </Space>
          </div>

          {/* Resize Tool */}
          <div>
            <Divider orientation="left">
              <ExpandOutlined /> Resize
            </Divider>
            <Space wrap>
              <label>Width:</label>
              <InputNumber
                min={1}
                value={resizeParams.width}
                onChange={(v) => setResizeParams({ ...resizeParams, width: v || 800 })}
                disabled={disabled}
              />
              <label>Height:</label>
              <InputNumber
                min={1}
                value={resizeParams.height}
                onChange={(v) => setResizeParams({ ...resizeParams, height: v || 600 })}
                disabled={disabled}
              />
              <Button
                type="primary"
                onClick={handleResize}
                disabled={disabled}
                icon={<ExpandOutlined />}
              >
                Apply Resize
              </Button>
            </Space>
          </div>

          {/* Rotate Tool */}
          <div>
            <Divider orientation="left">
              <SwapOutlined /> Rotate
            </Divider>
            <Space>
              <Button
                onClick={() => handleRotate(90)}
                disabled={disabled}
                icon={<SwapOutlined />}
              >
                90°
              </Button>
              <Button
                onClick={() => handleRotate(180)}
                disabled={disabled}
                icon={<SwapOutlined />}
              >
                180°
              </Button>
              <Button
                onClick={() => handleRotate(270)}
                disabled={disabled}
                icon={<SwapOutlined />}
              >
                270°
              </Button>
            </Space>
          </div>
        </Space>
      </Card>
    </Spin>
  );
};

export default ImageEditor;
