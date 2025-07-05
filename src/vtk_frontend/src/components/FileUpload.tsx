import { useRef, useState, useEffect } from "react";
// import { backend } from "@/declarations/backend"; // Adjust path if needed
import { vtk_backend } from "../../../declarations/vtk_backend";

const CHUNK_SIZE = 2 * 1024 * 1024; // 2MB

export default function FileUpload() {
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const [progress, setProgress] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<boolean>(false);
  const [isUploading, setIsUploading] = useState(false);
  const [fileIdCounter, setFileIdCounter] = useState(1); // Simple counter for file_id
  const [selectedFile, setSelectedFile] = useState<File | null>(null);

  const handleFileChange = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;
    setSelectedFile(file);
    if (file.size > 100 * 1024 * 1024) {
      setError("File too large. Max 100MB allowed.");
      setSuccess(false);
      setProgress(null);
      return;
    }

    setError(null);
    setSuccess(false);
    setProgress(0);
    setIsUploading(true);

    // Generate a file_id (in production, get this from backend or a UUID)
    const file_id = BigInt(fileIdCounter);
    setFileIdCounter((id) => id + 1);

    try {
      const totalChunks = Math.ceil(file.size / CHUNK_SIZE);
      const num_chunks = BigInt(totalChunks);
      for (let i = 0; i < totalChunks; i++) {
        const start = i * CHUNK_SIZE;
        const end = Math.min(start + CHUNK_SIZE, file.size);
        const chunk = file.slice(start, end);
        const buffer = await chunk.arrayBuffer();
        const file_content = [...new Uint8Array(buffer)];

        if (i === 0) {
          // First chunk with metadata
          const uploadArgs = {
            name: file.name, // ✅ Backend expects 'name'
            content: new Uint8Array(buffer), // ✅ Backend expects 'content'
            file_type: file.type || "application/octet-stream",
            num_chunks,
          };
          console.log("upload_file_atomic args:", uploadArgs);
          await vtk_backend.upload_file_atomic(uploadArgs);
          //   await backend.upload_file_atomic({
          //     file_id,
          //     file_content,
          //     file_type: file.type || "application/octet-stream",
          //     num_chunks,
          //   });
        } else {
          // Remaining chunks
          await vtk_backend.upload_file_continue({
            file_id,
            file_content: new Uint8Array(buffer),
            file_type: file.type || "application/octet-stream",
            num_chunks,
          });
        }

        setProgress(Math.round(((i + 1) / totalChunks) * 100));
      }

      setSuccess(true);
    } catch (err) {
      console.error("Upload failed", err);
      setError((err as Error).message || "Upload failed");
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <div className="space-y-4 my-6">
      <input
        type="file"
        accept="*/*"
        ref={fileInputRef}
        className="block"
        onChange={handleFileChange}
        disabled={isUploading}
      />

      {selectedFile && (
        <div className="text-sm text-gray-700">
          <div>
            <strong>File:</strong> {selectedFile.name}
          </div>
          <div>
            <strong>Size:</strong> {(selectedFile.size / (1024 * 1024)).toFixed(2)} MB
          </div>
        </div>
      )}

      {isUploading && (
        <div className="w-full max-w-md">
          <div className="flex items-center justify-between mb-1">
            <span className="text-xs font-medium text-blue-700">Uploading...</span>
            <span className="text-xs font-medium text-blue-700">{progress}%</span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700">
            <div
              className="bg-blue-600 h-2.5 rounded-full transition-all duration-200"
              style={{ width: `${progress ?? 0}%` }}
            ></div>
          </div>
        </div>
      )}
      {error && <p className="text-red-500">❌ {error}</p>}
      {success && <p className="text-green-600">✅ Upload successful!</p>}
    </div>
  );
}

// FileList component: lists files from the backend
export function FileList() {
  const [files, setFiles] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchFiles() {
      setLoading(true);
      setError(null);
      try {
        const result = await vtk_backend.list_files();
        setFiles(result);
      } catch (err) {
        setError((err as Error).message || "Failed to fetch files");
      } finally {
        setLoading(false);
      }
    }
    fetchFiles();
  }, []);

  if (loading) return <div>Loading files...</div>;
  if (error) return <div className="text-red-500">❌ {error}</div>;
  if (files.length === 0) return <div>No files found.</div>;

  return (
    <div className="my-6">
      <h2 className="font-bold mb-2">Uploaded Files</h2>
      <ul className="space-y-2">
        {files.map((file, idx) => (
          <li key={file.file_id ?? idx} className="border p-2 rounded">
            <div>
              <strong>Name:</strong> {file.file_name}
            </div>
            <div>
              <strong>Status:</strong> {JSON.stringify(file.file_status)}
            </div>
            {file.size && (
              <div>
                <strong>Size:</strong> {file.size} bytes
              </div>
            )}
          </li>
        ))}
      </ul>
    </div>
  );
}
