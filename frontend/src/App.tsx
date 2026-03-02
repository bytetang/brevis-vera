import { useState } from 'react';
import { ConfigProvider, Layout, Steps, Button, Card, Row, Col, Spin, message } from 'antd';
import { UploadOutlined, EditOutlined, SafetyCertificateOutlined } from '@ant-design/icons';
import ImageUploader from './components/ImageUploader';
import ProvenanceDisplay from './components/ProvenanceDisplay';
import ImageEditor from './components/ImageEditor';
import VerificationResult from './components/VerificationResult';
import type {
  C2PAMetadata,
  EditingRecord,
  ZKProofResponse,
} from './types';
import { extractProvenance, generateProof } from './services/api';

const { Header, Content } = Layout;

const App: React.FC = () => {
  // State
  const [currentStep, setCurrentStep] = useState(0);
  const [originalImage, setOriginalImage] = useState<string | null>(null);
  const [originalImageHash, setOriginalImageHash] = useState<string | null>(null);
  const [editedImage, setEditedImage] = useState<string | null>(null);
  const [c2paMetadata, setC2paMetadata] = useState<C2PAMetadata | null>(null);
  const [hasC2PA, setHasC2PA] = useState(false);
  const [editingRecords, setEditingRecords] = useState<EditingRecord[]>([]);
  const [proof, setProof] = useState<ZKProofResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Handle image upload
  const handleImageUpload = async (base64: string, _filename: string) => {
    setLoading(true);
    setError(null);
    try {
      // Call the API to extract provenance
      const response = await extractProvenance(base64);

      setOriginalImage(base64);
      if (response.metadata) {
        setC2paMetadata(response.metadata);
      }
      setHasC2PA(response.has_c2pa);
      setOriginalImageHash(response.original_image_hash);
      setCurrentStep(1);
      message.success('Image uploaded successfully');
    } catch (err) {
      setError('Failed to extract provenance');
      message.error('Failed to process image');
    } finally {
      setLoading(false);
    }
  };

  // Handle image edit
  const handleImageEdited = (editedBase64: string, record: EditingRecord) => {
    setEditedImage(editedBase64);
    setEditingRecords([...editingRecords, record]);
    message.success('Image edited');
  };

  // Handle proof generation
  const handleGenerateProof = async () => {
    if (!originalImageHash || !editedImage) {
      message.error('Please upload and edit an image first');
      return;
    }

    setLoading(true);
    try {
      // Get the edited image hash from the last editing record
      const lastRecord = editingRecords[editingRecords.length - 1];
      const editedImageHash = lastRecord?.output_hash || originalImageHash;

      // Call the API to generate ZK proof
      const response = await generateProof({
        original_image_hash: originalImageHash,
        edited_image_hash: editedImageHash,
        c2pa_data: c2paMetadata ? {
          active_manifest: c2paMetadata.active_manifest,
          claim_generator: c2paMetadata.claim_generator,
          algorithm: c2paMetadata.algorithm,
        } : undefined,
        editing_records: editingRecords,
      });

      setProof(response);
      setCurrentStep(2);
      message.success('ZK Proof generated successfully');
    } catch (err) {
      message.error('Failed to generate proof');
    } finally {
      setLoading(false);
    }
  };

  // Reset
  const handleReset = () => {
    setCurrentStep(0);
    setOriginalImage(null);
    setOriginalImageHash(null);
    setEditedImage(null);
    setC2paMetadata(null);
    setHasC2PA(false);
    setEditingRecords([]);
    setProof(null);
    setError(null);
  };

  const steps = [
    { title: 'Upload', icon: <UploadOutlined /> },
    { title: 'Edit', icon: <EditOutlined /> },
    { title: 'Verify', icon: <SafetyCertificateOutlined /> },
  ];

  return (
    <ConfigProvider
      theme={{
        token: {
          colorPrimary: '#1890ff',
        },
      }}
    >
      <Layout style={{ minHeight: '100vh' }}>
        <Header style={{ background: '#fff', padding: '0 24px', borderBottom: '1px solid #f0f0f0' }}>
          <h1 style={{ margin: 0, fontSize: 20 }}>Brevis Vera - Image Authenticity Attestation</h1>
        </Header>
        <Content style={{ padding: '24px', maxWidth: 1200, margin: '0 auto', width: '100%' }}>
          <Spin spinning={loading}>
            {/* Steps */}
            <Steps current={currentStep} items={steps} style={{ marginBottom: 24 }} />

            {/* Step 1: Upload */}
            {currentStep === 0 && (
              <Card title="Step 1: Upload Image">
                <ImageUploader onImageUploaded={handleImageUpload} />
              </Card>
            )}

            {/* Step 2: Edit */}
            {currentStep === 1 && (
              <Row gutter={16}>
                <Col span={12}>
                  <Card title="Original Image">
                    {originalImage && (
                      <img
                        src={`data:image/jpeg;base64,${originalImage}`}
                        alt="Original"
                        style={{ maxWidth: '100%' }}
                      />
                    )}
                  </Card>
                  <ProvenanceDisplay
                    metadata={c2paMetadata}
                    hasC2PA={hasC2PA}
                    imageHash={originalImageHash}
                  />
                </Col>
                <Col span={12}>
                  <ImageEditor
                    imageBase64={editedImage || originalImage || ''}
                    onImageEdited={handleImageEdited}
                  />
                  <Card style={{ marginTop: 16 }}>
                    <p>Editing Records: {editingRecords.length}</p>
                    <Button
                      type="primary"
                      size="large"
                      onClick={handleGenerateProof}
                      disabled={editingRecords.length === 0}
                      block
                    >
                      Generate ZK Proof
                    </Button>
                  </Card>
                </Col>
              </Row>
            )}

            {/* Step 3: Verify */}
            {currentStep === 2 && (
              <Row gutter={16}>
                <Col span={12}>
                  <Card title="Original Image">
                    {originalImage && (
                      <img
                        src={`data:image/jpeg;base64,${originalImage}`}
                        alt="Original"
                        style={{ maxWidth: '100%' }}
                      />
                    )}
                  </Card>
                </Col>
                <Col span={12}>
                  <Card title="Edited Image">
                    {editedImage && (
                      <img
                        src={`data:image/jpeg;base64,${editedImage}`}
                        alt="Edited"
                        style={{ maxWidth: '100%' }}
                      />
                    )}
                  </Card>
                  <VerificationResult proof={proof} editedImageBase64={editedImage} />
                </Col>
              </Row>
            )}

            {/* Error */}
            {error && (
              <Card style={{ marginTop: 16, borderColor: '#ff4d4f' }}>
                <p style={{ color: '#ff4d4f' }}>{error}</p>
              </Card>
            )}

            {/* Reset Button */}
            {currentStep > 0 && (
              <div style={{ marginTop: 24, textAlign: 'center' }}>
                <Button onClick={handleReset}>Start Over</Button>
              </div>
            )}
          </Spin>
        </Content>
      </Layout>
    </ConfigProvider>
  );
};

export default App;
