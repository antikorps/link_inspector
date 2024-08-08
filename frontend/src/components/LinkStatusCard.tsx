import type { LinkStatusResponse } from "../types/LinkStatusResponse";

const LinkStatusCard = ({ link }: { link: LinkStatusResponse }) => {
  return (
    <article className="flex justify-between gap-2">
      <span>{link.active === true ? "🟢" : "🔴"}</span>
      <span className="font-medium text-gray-900 grow text-wrap break-all dark:text-white">
        {link.url}
      </span>
      <div className="flex flex-col justify-center align-end">
        <span className="text-sm text-base text-gray-900 font-semibold dark:text-white">
          Status:
        </span>
        <span className="text-sm text-base text-gray-900 dark:text-white">
          {link.status}
        </span>
      </div>
    </article>
  );
};

export default LinkStatusCard;
