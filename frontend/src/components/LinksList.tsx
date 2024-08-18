import { links_list, links_loading } from "@states/links";
import type { Link } from "@customTypes/link";
import { useStore } from "@nanostores/react";
import { useState } from "react";
import { LinkItem } from "./LinkItem";
import { LinkListSkeleton } from "./LinkListSkeleton";

export const LinksList = () => {
  const links = useStore(links_list);
  const loading = useStore(links_loading);

  const paginate = (array: Link[], pageSize: number) => {
    const pageCount = Math.ceil(array.length / pageSize);
    return Array.from({ length: pageCount }, (_, index) =>
      array.slice(index * pageSize, (index + 1) * pageSize)
    );
  };

  const [currentPage, setCurrentPage] = useState(1);
  const [pageSize, setPageSize] = useState(5);
  const paginatedLinks = paginate(links, pageSize);
  const currentLinks = paginatedLinks[currentPage - 1];

  const nextPage = () => {
    if (currentPage < paginatedLinks.length) {
      setCurrentPage(currentPage + 1);
    }
  }

  const previousPage = () => {
    if (currentPage > 1) {
      setCurrentPage(currentPage - 1);
    }
  }
  const changePageSize = (event: React.ChangeEvent<HTMLSelectElement>) => {
    setPageSize(parseInt(event.target.value));
    setCurrentPage(1);
  }

  return (
    <div className="flex flex-col gap-6">
      <div className="flex flex-col justify-between sm:flex-row sm:items-center space-y-4 sm:space-y-0 sm:space-x-3 rtl:space-x-reverse w-full sm:w-auto">
        <div className="flex items-center gap-2">
          <select disabled={links.length == 0 ? true : false} onChange={changePageSize} defaultValue="5" id="entries-per-page" className="bg-gray-50 border border-gray-300 text-gray-900 disabled:bg-zinc-200 disabled:border-zinc-200 disabled:dark:bg-zinc-800 disabled:text-zinc-400 disabled:dark:text-zinc-500 disabled:dark:border-zinc-800 text-sm rounded-lg focus:ring-gray-100 focus:border-gray-100 block p-2.5 dark:border-gray-600 dark:bg-gray-800 dark:placeholder-gray-400 dark:text-white dark:focus:ring-gray-600 dark:focus:border-gray-600">
            <option value="5">5</option>
            <option value="10">10</option>
            <option value="15">15</option>
            <option value="25">25</option>
            <option value="50">50</option>
          </select>
          <label htmlFor="entries-per-page" className="flex items-center mb-2 text-sm text-gray-500 dark:text-gray-400">registros por página</label>
        </div>

        <button disabled={links.length == 0 ? true : false} id="exportDropdownButton" data-dropdown-toggle="dropdown" className="flex w-full items-center justify-center rounded-lg border border-gray-200 bg-white disabled:bg-zinc-200 disabled:border-zinc-200 disabled:dark:bg-zinc-800 disabled:text-zinc-400 disabled:dark:text-zinc-500 disabled:dark:border-zinc-800 px-3 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-primary-700 focus:z-10 focus:outline-none focus:ring-4 focus:ring-gray-100 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white dark:focus:ring-gray-700 sm:w-auto" type="button">Exportar como <svg className="w-2.5 h-2.5 ms-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 10 6">
          <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="m1 1 4 4 4-4" />
        </svg>
        </button>

        <div id="dropdown" data-popper-placement="bottom" className="z-10 w-52 divide-y divide-gray-100 rounded-lg bg-white shadow dark:bg-gray-700 hidden">
          <ul className="p-2 text-left text-sm font-medium text-gray-500 dark:text-gray-400" aria-labelledby="dropdownDefaultButton">
            <li>
              <button id="export-csv" className="group inline-flex w-full items-center rounded-md px-3 py-2 text-sm text-gray-500 hover:bg-gray-100 hover:text-gray-900 dark:text-gray-400 dark:hover:bg-gray-600 dark:hover:text-white"><svg className="me-1.5 h-4 w-4 text-gray-400 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24"><path fillRule="evenodd" d="M9 2.221V7H4.221a2 2 0 0 1 .365-.5L8.5 2.586A2 2 0 0 1 9 2.22ZM11 2v5a2 2 0 0 1-2 2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2 2 2 0 0 0 2 2h12a2 2 0 0 0 2-2 2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2V4a2 2 0 0 0-2-2h-7Zm1.018 8.828a2.34 2.34 0 0 0-2.373 2.13v.008a2.32 2.32 0 0 0 2.06 2.497l.535.059a.993.993 0 0 0 .136.006.272.272 0 0 1 .263.367l-.008.02a.377.377 0 0 1-.018.044.49.49 0 0 1-.078.02 1.689 1.689 0 0 1-.297.021h-1.13a1 1 0 1 0 0 2h1.13c.417 0 .892-.05 1.324-.279.47-.248.78-.648.953-1.134a2.272 2.272 0 0 0-2.115-3.06l-.478-.052a.32.32 0 0 1-.285-.341.34.34 0 0 1 .344-.306l.94.02a1 1 0 1 0 .043-2l-.943-.02h-.003Zm7.933 1.482a1 1 0 1 0-1.902-.62l-.57 1.747-.522-1.726a1 1 0 0 0-1.914.578l1.443 4.773a1 1 0 0 0 1.908.021l1.557-4.773Zm-13.762.88a.647.647 0 0 1 .458-.19h1.018a1 1 0 1 0 0-2H6.647A2.647 2.647 0 0 0 4 13.647v1.706A2.647 2.647 0 0 0 6.647 18h1.018a1 1 0 1 0 0-2H6.647A.647.647 0 0 1 6 15.353v-1.706c0-.172.068-.336.19-.457Z" clipRule="evenodd"></path></svg><span>Exportar CSV</span></button>
            </li>
            <li>
              <button id="export-json" className="group inline-flex w-full items-center rounded-md px-3 py-2 text-sm text-gray-500 hover:bg-gray-100 hover:text-gray-900 dark:text-gray-400 dark:hover:bg-gray-600 dark:hover:text-white"><svg className="me-1.5 h-4 w-4 text-gray-400 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24"><path fillRule="evenodd" d="M9 2.221V7H4.221a2 2 0 0 1 .365-.5L8.5 2.586A2 2 0 0 1 9 2.22ZM11 2v5a2 2 0 0 1-2 2H4v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2h-7Zm-.293 9.293a1 1 0 0 1 0 1.414L9.414 14l1.293 1.293a1 1 0 0 1-1.414 1.414l-2-2a1 1 0 0 1 0-1.414l2-2a1 1 0 0 1 1.414 0Zm2.586 1.414a1 1 0 0 1 1.414-1.414l2 2a1 1 0 0 1 0 1.414l-2 2a1 1 0 0 1-1.414-1.414L14.586 14l-1.293-1.293Z" clipRule="evenodd"></path></svg><span>Exportar JSON</span></button>
            </li>
            <li>
              <button id="export-txt" className="group inline-flex w-full items-center rounded-md px-3 py-2 text-sm text-gray-500 hover:bg-gray-100 hover:text-gray-900 dark:text-gray-400 dark:hover:bg-gray-600 dark:hover:text-white"><svg className="me-1.5 h-4 w-4 text-gray-400 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24"><path fillRule="evenodd" d="M9 2.221V7H4.221a2 2 0 0 1 .365-.5L8.5 2.586A2 2 0 0 1 9 2.22ZM11 2v5a2 2 0 0 1-2 2H4v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2h-7ZM8 16a1 1 0 0 1 1-1h6a1 1 0 1 1 0 2H9a1 1 0 0 1-1-1Zm1-5a1 1 0 1 0 0 2h6a1 1 0 1 0 0-2H9Z" clipRule="evenodd"></path></svg><span>Exportar TXT</span></button>
            </li>
          </ul>
        </div>

      </div>
      {links.length > 0 && !loading && (
        <ul className={`max-w-md divide-y divide-gray-200 dark:divide-gray-700`}>
          {currentLinks.map((link: Link, index: number) => (
            <LinkItem key={index} link={link} index={index} />
          ))}
        </ul>
      )}

      {links.length === 0 && !loading && (
        <div className="flex items-center justify-center h-96 dark:bg-gray-800 bg-gray-100 rounded-md opacity-65">
          <p className="text-lg text-gray-900 dark:text-gray-300">No se han encontrado enlaces</p>
        </div>
      )}

      {loading && (
        <LinkListSkeleton />
      )}
      <div className="flex items-center justify-between">
        <span className="text-sm text-gray-700 dark:text-gray-400">
          {(links.length > 0 ? 1 : 0) + ((currentPage - 1) * pageSize)} to {(pageSize * currentPage) < links.length ? (pageSize * currentPage) : links.length} of {links.length} entries.
        </span>
        <div className="inline-flex mt-2 xs:mt-0">
          <button disabled={currentPage == 1 ? true : false} onClick={previousPage} className="flex items-center disabled:pointer-events-none disabled:bg-zinc-200 disabled:border-zinc-200 disabled:dark:bg-zinc-800 disabled:text-zinc-400 disabled:dark:text-zinc-500 disabled:dark:border-zinc-800  justify-center px-3 h-8 me-3 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-lg hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white select-none">
            <svg className="w-3.5 h-3.5 me-2 rtl:rotate-180" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10">
              <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 5H1m0 0 4 4M1 5l4-4" />
            </svg>
            Prev
          </button>
          <button disabled={(pageSize * currentPage) >= links.length ? true : false} onClick={nextPage} className="flex items-center disabled:pointer-events-none disabled:bg-zinc-200 disabled:border-zinc-200 disabled:dark:bg-zinc-800 disabled:text-zinc-400 disabled:dark:text-zinc-500 disabled:dark:border-zinc-800  justify-center px-3 h-8 me-3 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-lg hover:bg-gray-100 hover:text-gray-700 dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white select-none">
            Next
            <svg className="w-3.5 h-3.5 ms-2 rtl:rotate-180" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10">
              <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M1 5h12m0 0L9 1m4 4L9 9" />
            </svg>
          </button>
        </div>
      </div>
    </div>
  );
};
