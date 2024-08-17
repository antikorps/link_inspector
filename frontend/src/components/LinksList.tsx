import { links_list, links_loading } from "@states/links";
import type { Link } from "@customTypes/link";
import { useStore } from "@nanostores/react";
import { LinkItem } from "./LinkItem";
import { LinkListSkeleton } from "./LinkListSkeleton";

export const LinksList = () => {
  const links = useStore(links_list);
  const loading = useStore(links_loading);

  console.log(links);
  return (
    <div>
      {links.length > 0 && !loading && (
        <ul className="max-w-md divide-y divide-gray-200 dark:divide-gray-700">
          {links.map((link: Link, index: number) => (
            <LinkItem key={index} link={link} index={index} />
          ))}
        </ul>
      )}

      {links.length === 0 && !loading && (
        <p className="text-lg font-semibold dark:text-white">No links yet</p>
      )}

      {loading && (
        <LinkListSkeleton />
      )}
    </div>
  );
};
