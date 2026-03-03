import { useState } from 'react';
import { ConfigProvider, Layout, Steps, Button, Card, Row, Col, Spin, message, Typography } from 'antd';
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
const { Title, Text } = Typography;

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
          colorPrimary: '#4f46e5',
          colorSuccess: '#10b981',
          colorWarning: '#f59e0b',
          colorError: '#ef4444',
          colorBgContainer: '#ffffff',
          colorBgLayout: '#f4f6f8',
          colorBorder: '#e5e7eb',
          colorText: '#111827',
          colorTextSecondary: '#4b5563',
          borderRadius: 6,
          fontFamily: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif",
        },
        components: {
          Button: {
            borderRadius: 6,
            controlHeight: 40,
            paddingContentHorizontal: 20,
          },
          Card: {
            borderRadiusLG: 8,
          },
          Input: {
            borderRadius: 6,
            controlHeight: 40,
          },
          Table: {
            headerBg: '#f9fafb',
            headerColor: '#6b7280',
            rowHoverBg: '#f9fafb',
          },
          Tag: {
            borderRadiusSM: 4,
          },
        },
      }}
    >
      <Layout style={{ minHeight: '100vh' }}>
        <Header className="app-header">
          <div style={{ maxWidth: 1200, margin: '0 auto', width: '100%', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <Title level={4} style={{ margin: 0, letterSpacing: '-0.025em' }}>Brevis Vera</Title>
            <Text type="secondary" style={{ fontSize: 14 }}>Image Authenticity Attestation</Text>
          </div>
        </Header>
        <Content style={{ padding: '32px 24px', maxWidth: 1200, margin: '0 auto', width: '100%' }}>
          <Spin spinning={loading}>
            {/* Page Title */}
            <div className="fade-in" style={{ marginBottom: 24 }}>
              <Title level={3} className="page-title" style={{ marginBottom: 8 }}>
                {currentStep === 0 && 'Upload Your Image'}
                {currentStep === 1 && 'Edit & Process'}
                {currentStep === 2 && 'Verification Result'}
              </Title>
              <Text type="secondary">
                {currentStep === 0 && 'Upload an image to extract C2PA provenance metadata and verify authenticity.'}
                {currentStep === 1 && 'Apply editing operations to the image. Each operation will be recorded for ZK proof generation.'}
                {currentStep === 2 && 'Your image has been verified. The ZK proof confirms the editing operations without revealing sensitive details.'}
              </Text>
            </div>

            {/* Steps */}
            <div className="fade-in" style={{ marginBottom: 32 }}>
              <Steps
                current={currentStep}
                items={steps}
                style={{
                  marginBottom: 24,
                  padding: '20px 24px',
                  background: '#ffffff',
                  borderRadius: 8,
                  border: '1px solid #e5e7eb',
                  boxShadow: '0 1px 3px rgba(0,0,0,0.05), 0 1px 2px rgba(0,0,0,0.03)',
                }}
              />
            </div>

            {/* Step 1: Upload */}
            {currentStep === 0 && (
              <div className="slide-up">
                <Card
                  title={<span className="section-title">Step 1: Upload Image</span>}
                  style={{
                    boxShadow: '0 1px 3px rgba(0,0,0,0.05), 0 1px 2px rgba(0,0,0,0.03)',
                  }}
                  styles={{ body: { padding: 24 } }}
                >
                  <ImageUploader onImageUploaded={handleImageUpload} />
                </Card>
              </div>
            )}

            {/* Step 2: Edit */}
            {currentStep === 1 && (
              <Row gutter={24} className="slide-up">
                <Col span={12}>
                  <Card
                    title={<span className="section-title">Original Image</span>}
                    style={{
                      boxShadow: '0 1px 3px rgba(0,0,0,0.05), 0 1px 2px rgba(0,0,0,0.03)',
                      marginBottom: 24,
                    }}
                    styles={{ body: { padding: 16 } }}
                  >
                    <div className="image-preview">
                      {originalImage && (
                        <img
                          src={`data:image/jpeg;base64,${originalImage}`}
                          alt="Original"
                        />
                      )}
                    </div>
                  </Card>
                  <ProvenanceDisplay
                    metadata={c2paMetadata}
                    hasC2PA={hasC2PA}
                    imageHash={originalImageHash}
                  />
                </Col>
                <Col span={12}>
                  <Card
                    title={<span className="section-title">Image Editor</span>}
                    style={{
                      boxShadow: '0 1px 3px rgba(0,0,0,0.05), 0 1px 2px rgba(0,0,0,0.03)',
                      marginBottom: 24,
                    }}
                  >
                    <div className="image-preview" style={{ marginBottom: 16 }}>
                      {editedImage || originalImage ? (
                        <img
                          src={`data:image/jpeg;base64,${editedImage || originalImage}`}
                          alt="Preview"
                        />
                      ) : null}
                    </div>
                    <ImageEditor
                      imageBase64={editedImage || originalImage || ''}
                      onImageEdited={handleImageEdited}
                    />
                  </Card>
                  <Card
                    style={{
                      boxShadow: '0 1px 3px rgba(0,0,0,0.05), 0 1px 2px rgba(0,0,0,0.03)',
                    }}
                  >
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 16 }}>
                      <Text strong>Editing Records</Text>
                      <span className="status-badge" style={{
                        background: editingRecords.length > 0 ? '#d1fae5' : '#f3f4f6',
                        color: editingRecords.length > 0 ? '#10b981' : '#9ca3af',
                      }}>
                        {editingRecords.length} operation{editingRecords.length !== 1 ? 's' : ''}
                      </span>
                    </div>
                    <Button
                      type="primary"
                      size="large"
                      onClick={handleGenerateProof}
                      disabled={editingRecords.length === 0}
                      block
                      style={{
                        height: 48,
                        fontSize: 15,
                        fontWeight: 600,
                      }}
                    >
                      Generate ZK Proof
                    </Button>
                  </Card>
                </Col>
              </Row>
            )}

            {/* Step 3: Verify */}
            {currentStep === 2 && (
              <Row gutter={24} className="slide-up">
                <Col span={12}>
                  <Card
                    title={<span className="section-title">Original Image</span>}
                    style={{
                      boxShadow: '0 1px 3px rgba(0,0,0,0.05), 0 1px 2px rgba(0,0,0,0.03)',
                      marginBottom: 24,
                    }}
                    styles={{ body: { padding: 16 } }}
                  >
                    <div className="image-preview">
                      {originalImage && (
                        <img
                          src={`data:image/jpeg;base64,${originalImage}`}
                          alt="Original"
                        />
                      )}
                    </div>
                  </Card>
                </Col>
                <Col span={12}>
                  <Card
                    title={<span className="section-title">Edited Image</span>}
                    style={{
                      boxShadow: '0 1px 3px rgba(0,0,0,0.05), 0 1px 2px rgba(0,0,0,0.03)',
                      marginBottom: 24,
                    }}
                    styles={{ body: { padding: 16 } }}
                  >
                    <div className="image-preview">
                      {editedImage && (
                        <img
                          src={`data:image/jpeg;base64,${editedImage}`}
                          alt="Edited"
                        />
                      )}
                    </div>
                  </Card>
                  <VerificationResult proof={proof} editedImageBase64={editedImage} />
                </Col>
              </Row>
            )}

            {/* Error */}
            {error && (
              <Card style={{ marginTop: 24, borderColor: '#fee2e2', background: '#fef2f2' }}>
                <Text type="danger">{error}</Text>
              </Card>
            )}

            {/* Reset Button */}
            {currentStep > 0 && (
              <div style={{ marginTop: 32, textAlign: 'center' }}>
                <Button
                  onClick={handleReset}
                  size="large"
                  style={{
                    minWidth: 160,
                    borderRadius: 6,
                  }}
                >
                  Start Over
                </Button>
              </div>
            )}
          </Spin>
        </Content>
      </Layout>
    </ConfigProvider>
  );
};

export default App;
