import { StatusBadge } from "./StatusBadge";
import { Clipboard } from "flowbite-react";
import type { Link } from "@customTypes/link";


export const LinkItem = ({ link, index }: { link: Link, index: number }) => {

  return (
    <li className="py-3 sm:py-4 relative">
      <div className="flex items-center space-x-4 rtl:space-x-reverse">
        <div className="flex-shrink-0">
          <StatusBadge status={link.status} active={link.active} />
        </div>
        <a href={link.url} target="_blank" className="flex-1 min-w-0">
          <p className="text-sm font-medium text-gray-900 truncate dark:text-white">
            {link.text}
          </p>
          <p
            id={`url-${index}`}
            className="text-sm text-gray-500 truncate dark:text-gray-400"
          >
            {link.url}
          </p>
        </a>
        <div className="inline-flex items-center text-base font-semibold dark:hover:bg-gray-800 rounded-md p-0.5">
          <div>
            <Clipboard.WithIcon
              className="relative"
              valueToCopy={link.url}
            />
          </div>
        </div>
      </div>
    </li>
  )
};
