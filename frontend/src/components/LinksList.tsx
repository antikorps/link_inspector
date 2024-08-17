import { links_list, links_loading } from "@states/links";
import type { Link } from "@customTypes/link";
import { useStore } from "@nanostores/react";
import { Clipboard } from "flowbite-react";
import { StatusBadge } from "./StatusBadge";

export const LinksList = () => {
  const links = useStore(links_list);
  const loading = useStore(links_loading);

  console.log(links);
  return (
    <div>
      {links.length > 0 && !loading && (
        <ul className="max-w-md divide-y divide-gray-200 dark:divide-gray-700">
          {links.map((link: Link, index: number) => (
            <li key={index.toString()} className="py-3 sm:py-4 relative">
              <div className="flex items-center space-x-4 rtl:space-x-reverse">
                <div className="flex-shrink-0">
                  <StatusBadge status={link.status} active={link.active} />
                </div>
                <div className="flex-1 min-w-0">
                  <p className="text-sm font-medium text-gray-900 truncate dark:text-white">
                    {link.text}
                  </p>
                  <p
                    id={`url-${index}`}
                    className="text-sm text-gray-500 truncate dark:text-gray-400"
                  >
                    {link.url}
                  </p>
                </div>
                <div className="inline-flex items-center text-base font-semibold dark:hover:bg-gray-800 rounded-md p-0.5">
                  <div>
                    <label htmlFor={`url-${index}`} className="sr-only">
                      Label
                    </label>
                    <Clipboard.WithIcon
                      className="relative"
                      valueToCopy={link.url}
                    />
                  </div>
                </div>
              </div>
            </li>
          ))}
        </ul>
      )}

      {links.length === 0 && !loading && (
        <p className="text-lg font-semibold dark:text-white">No links yet</p>
      )}

      {loading && (


        <div role="status" className="max-w-md p-4 space-y-4 border border-gray-200 divide-y divide-gray-200 rounded shadow animate-pulse dark:divide-gray-700 md:p-6 dark:border-gray-700">
          <div className="flex items-center justify-between">
            <div>
              <div className="h-2.5 bg-gray-300 rounded-full dark:bg-gray-600 w-24 mb-2.5"></div>
              <div className="w-32 h-2 bg-gray-200 rounded-full dark:bg-gray-700"></div>
            </div>
            <div className="h-2.5 bg-gray-300 rounded-full dark:bg-gray-700 w-12"></div>
          </div>
          <div className="flex items-center justify-between pt-4">
            <div>
              <div className="h-2.5 bg-gray-300 rounded-full dark:bg-gray-600 w-24 mb-2.5"></div>
              <div className="w-32 h-2 bg-gray-200 rounded-full dark:bg-gray-700"></div>
            </div>
            <div className="h-2.5 bg-gray-300 rounded-full dark:bg-gray-700 w-12"></div>
          </div>
          <div className="flex items-center justify-between pt-4">
            <div>
              <div className="h-2.5 bg-gray-300 rounded-full dark:bg-gray-600 w-24 mb-2.5"></div>
              <div className="w-32 h-2 bg-gray-200 rounded-full dark:bg-gray-700"></div>
            </div>
            <div className="h-2.5 bg-gray-300 rounded-full dark:bg-gray-700 w-12"></div>
          </div>
          <div className="flex items-center justify-between pt-4">
            <div>
              <div className="h-2.5 bg-gray-300 rounded-full dark:bg-gray-600 w-24 mb-2.5"></div>
              <div className="w-32 h-2 bg-gray-200 rounded-full dark:bg-gray-700"></div>
            </div>
            <div className="h-2.5 bg-gray-300 rounded-full dark:bg-gray-700 w-12"></div>
          </div>
          <div className="flex items-center justify-between pt-4">
            <div>
              <div className="h-2.5 bg-gray-300 rounded-full dark:bg-gray-600 w-24 mb-2.5"></div>
              <div className="w-32 h-2 bg-gray-200 rounded-full dark:bg-gray-700"></div>
            </div>
            <div className="h-2.5 bg-gray-300 rounded-full dark:bg-gray-700 w-12"></div>
          </div>
          <span className="sr-only">Loading...</span>
        </div>
      )}
    </div>
  );
};
