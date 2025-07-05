import { useEffect, useState } from "react";
import { vtk_backend } from "../../../declarations/vtk_backend";

type FileStatus =
  | { uploaded: { uploaded_at: bigint } }
  | { partially_uploaded: null }
  | { pending: { alias: string; requested_at: bigint } };

type FileMetadata = {
  file_id: bigint;
  file_name: string;
  file_status: FileStatus;
};

function formatStatus(status: FileStatus): string {
  if ("uploaded" in status) return "✅ Uploaded";
  if ("partially_uploaded" in status) return "⏳ Partially Uploaded";
  if ("pending" in status) return `🕓 Pending (alias: ${status.pending.alias})`;
  return "❓ Unknown";
}

export default function ListFiles() {
  const [files, setFiles] = useState<FileMetadata[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    vtk_backend
      .list_files()
      .then((result) => {
        setFiles(result);
      })
      .catch((err) => {
        console.error("Failed to load files:", err);
      })
      .finally(() => setLoading(false));
  }, []);

  return (
    <div className="my-6">
      <h2 className="text-lg font-semibold mb-4">📁 Uploaded Files</h2>

      {loading ? (
        <p className="text-gray-600">Loading...</p>
      ) : files.length === 0 ? (
        <p className="text-gray-600">No files uploaded yet.</p>
      ) : (
        <ul className="space-y-3">
          {files.map((file) => (
            <li key={file.file_id.toString()} className="p-4 border rounded-md shadow-sm bg-white">
              <div className="text-sm font-medium text-gray-800">{file.file_name}</div>
              <div className="text-xs text-gray-500">ID: {file.file_id.toString()}</div>
              <div className="text-xs mt-1">{formatStatus(file.file_status)}</div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
