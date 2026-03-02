import { useState } from 'react';
import { Upload, message } from 'antd';
import { UploadOutlined } from '@ant-design/icons';
import type { UploadFile } from 'antd/es/upload/interface';

interface ImageUploaderProps {
  onImageUploaded: (base64: string, filename: string) => void;
  disabled?: boolean;
}

const ImageUploader: React.FC<ImageUploaderProps> = ({ onImageUploaded, disabled }) => {
  const [loading, setLoading] = useState(false);

  const beforeUpload = (file: File) => {
    const isImage = file.type === 'image/jpeg' || file.type === 'image/png';
    if (!isImage) {
      message.error('You can only upload JPEG or PNG files!');
      return false;
    }
    const isLt10M = file.size / 1024 / 1024 < 10;
    if (!isLt10M) {
      message.error('Image must be smaller than 10MB!');
      return false;
    }
    return false; // Prevent auto upload
  };

  const handleChange = async (info: { file: UploadFile }) => {
    const file = info.file.originFileObj || info.file as unknown as File;
    if (!file) return;

    setLoading(true);
    try {
      const reader = new FileReader();
      reader.onload = (e) => {
        const result = e.target?.result as string;
        const base64 = result.split(',')[1];
        onImageUploaded(base64, file.name);
        setLoading(false);
      };
      reader.onerror = () => {
        message.error('Failed to read file');
        setLoading(false);
      };
      reader.readAsDataURL(file);
    } catch (error) {
      message.error('Upload failed');
      setLoading(false);
    }
  };

  const uploadProps = {
    name: 'image',
    accept: '.jpg,.jpeg,.png',
    beforeUpload,
    onChange: handleChange,
    showUploadList: false,
    disabled,
  };

  return (
    <Upload.Dragger {...uploadProps}>
      <p className="ant-upload-drag-icon">
        <UploadOutlined />
      </p>
      <p className="ant-upload-text">Click or drag image to upload</p>
      <p className="ant-upload-hint">Support for JPEG or PNG images (max 10MB)</p>
      {loading && <p>Processing...</p>}
    </Upload.Dragger>
  );
};

export default ImageUploader;
