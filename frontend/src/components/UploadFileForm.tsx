import { useState, useRef } from "react";
import type { FormEvent } from "react";
import type { LinkStatusResponse } from "../types/LinkStatusResponse";
import LinkStatusCard from "./LinkStatusCard";

export default function UploadFileForm() {
  const [responseMessage, setResponseMessage] = useState(
    null as LinkStatusResponse[] | null,
  );
  const fileInput = useRef<HTMLInputElement>(null);

  async function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    const formData = new FormData(e.target as HTMLFormElement);
    const response = await fetch("/upload", {
      method: "POST",
      body: formData,
    });
    const data = await response.json();
    if (data.links) {
      setResponseMessage(data.links);
    }
  }

  function reset() {
    setResponseMessage(null);
    if (fileInput.current) fileInput.current.value = "";
  }

  return (
    <form
      className="w-full p-4 bg-white border border-gray-200 rounded-lg shadow sm:p-8 mb-8 dark:bg-gray-800 dark:border-gray-700"
      onSubmit={submit}
    >
      <div className="flex mb-4">
        <label htmlFor="file" className="text-gray-900 dark:text-white">
          Selecciona un archivo:
          <input
            id="file"
            name="file"
            type="file"
            required
            ref={fileInput}
            className="text-gray-900 dark:text-white"
          />
        </label>
        <button
          type="button"
          className="focus:outline-none text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-red-600 dark:hover:bg-red-700 dark:focus:ring-red-900"
          onClick={reset}
        >
          Borrar
        </button>
        <button className="focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-4 focus:ring-green-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800">
          Enviar
        </button>
      </div>
      <div className="flex items-center justify-between mb-4">
        <h5 className="text-xl font-bold leading-none text-gray-900 dark:text-white">
          Lista Enlaces
        </h5>
      </div>
      <div className="flow-root">
        <ul
          role="list"
          className="divide-y divide-gray-200 dark:divide-gray-700"
        >
          {responseMessage &&
            responseMessage.map((link) => (
              <li className="py-3 sm:py-4">
                <LinkStatusCard link={link} />
              </li>
            ))}
        </ul>
      </div>
    </form>
  );
}
