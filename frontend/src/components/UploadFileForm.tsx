import { useState } from "react";
import { links_list, links_loading } from "@states/links";
import { useStore } from "@nanostores/react";

export const UploadFileForm = () => {

  const [file, setFile] = useState<File | null>(null);
  const [error, setError] = useState<string | null>(null);
  const loading = useStore(links_loading);
  const [dragging, setDragging] = useState(false);

  const handleUpload = async (file: File) => {
    setFile(file);
    links_loading.set(true);
    const formData = new FormData();
    formData.append("file", file);
    const response = await fetch("/upload", {
      method: "POST",
      body: formData,
    });
    if (response.ok) {
      const data = await response.json();
      links_list.set(data["links"]);
      setError(null);
    } else {
      const errorMessage = await response.text();
      links_list.set([]);
      setError(errorMessage);
    }
    links_loading.set(false);
  };

  const handleFileChange = async (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const files = event.target.files;
    const file = files && files.length > 0 ? files[0] : null;
    setFile(null);

    if (file) {
      handleUpload(file);
    } else {
      links_list.set([]);
      setFile(null);
    }
  };

  const handleDrop = (event: React.DragEvent) => {
    event.preventDefault();
    setDragging(false);
    const droppedFiles = event.dataTransfer.files;
    if (droppedFiles.length > 0) {
      handleUpload(droppedFiles[0]);
    }
  };

  const handleDragOver = (event: React.DragEvent) => {
    setDragging(true);
    event.preventDefault();
    event.stopPropagation();
  };

  return (
    <div className="flex items-center justify-center w-full max-w-md">
      <label
        htmlFor="dropzone-file"
        className="flex flex-col items-center justify-center w-full h-64 border-2 px-8 text-center border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-gray-700 drag dark:bg-gray-800 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onDragLeave={() => setDragging(false)}
      >
        {dragging && !loading && (
          <svg
            className="w-24 h-24 text-gray-800 dark:text-gray-300 opacity-50 pointer-events-none"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            fill="none"
            viewBox="0 0 24 24"
          >
            <path
              stroke="currentColor"
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M4 15v2a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3v-2m-8 1V4m0 12-4-4m4 4 4-4"
            />
          </svg>
        )}
        {!file && !dragging && (
          <div className="flex flex-col items-center justify-center pt-5 pb-6">
            <svg
              className="w-8 h-8 mb-4 text-gray-500 dark:text-gray-400"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 20 16"
            >
              <path
                stroke="currentColor"
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M13 13h3a3 3 0 0 0 0-6h-.025A5.56 5.56 0 0 0 16 6.5 5.5 5.5 0 0 0 5.207 5.021C5.137 5.017 5.071 5 5 5a4 4 0 0 0 0 8h2.167M10 15V6m0 0L8 8m2-2 2 2"
              />
            </svg>
            <p className="mb-2 text-sm text-gray-500 dark:text-gray-400">
              <span className="font-semibold"> Haz click para subir tu archivo</span> o arrastra y suelta tu archivo aquí
            </p>
            <p className="text-xs text-gray-500 dark:text-gray-400">
              Extensiones válidas: docx, pptx, xlsx, html, txt, pdf
            </p>
          </div>
        )}
        {loading && (
          <div role="status">
            <svg aria-hidden="true" className="inline w-10 h-10 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor" />
              <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill" />
            </svg>
            <span className="sr-only">Loading...</span>
          </div>
        )}
        {file && !loading && !error && !dragging && (
          <div className="flex flex-col items-center justify-center pt-5 pb-6">
            <svg
              className="w-16 h-16 mb-4 text-green-500 pointer-events-none"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              fill="none"
              viewBox="0 0 24 24"
            >
              <path
                stroke="currentColor"
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M8.5 11.5 11 14l4-4m6 2a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
              />
            </svg>
            <span className="text-md font-semibold text-green-600 text-center dark:text-green-400">
              {file.name}
            </span>
          </div>
        )}

        {file && !loading && error && !dragging && (
          <div className="flex flex-col items-center justify-center pt-5 pb-6 pointer-events-none">
            <svg
              className="w-16 h-16 mb-4 text-red-500"
              aria-hidden="true"
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              fill="none"
              viewBox="0 0 24 24"
            >
              <path
                stroke="currentColor"
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="m15 9-6 6m0-6 6 6m6-3a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
              />
            </svg>

            <span className="text-md text-red-600 text-center dark:text-red-400 font-semibold">
              Error:
            </span>
            <span className="text-md text-red-600 text-center dark:text-red-400">
              {error}
            </span>
          </div>
        )}

        <input
          id="dropzone-file"
          type="file"
          accept=".docx, .pptx, .xlsx, .html, .txt, .pdf"
          required
          className="hidden"
          onChange={handleFileChange}
        />
      </label>
    </div>
  );
};
