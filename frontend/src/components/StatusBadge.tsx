interface Props {
  status?: number;
  active: number;
}

export const StatusBadge = ({ status, active }: Props) => {
  return (
    <>
      {active == 1 && (
        <span className=" inline-flex w-2 h-2 me-1 bg-green-500 rounded-full md:hidden"></span>
      )}
      {active == 2 && (
        <span className=" inline-flex w-2 h-2 me-1 bg-yellow-500 rounded-full md:hidden"></span>
      )}
      {active == 3 && (
        <span className=" inline-flex w-2 h-2 me-1 bg-red-500 rounded-full md:hidden"></span>
      )}

      {active == 1 && (
        <span className="items-center bg-green-100 text-green-800 text-xs font-medium px-2.5 py-0.5 rounded-full dark:bg-green-900 dark:text-green-300 hidden md:inline-flex">
          <span className="w-2 h-2 me-1 bg-green-500 rounded-full"></span>
          {status}
        </span>
      )}
      {active == 2 && (
        <span className="items-center bg-yellow-100 text-yellow-800 text-xs font-medium px-2.5 py-0.5 rounded-full dark:bg-yellow-900 dark:text-yellow-300 hidden md:inline-flex">
          <span className="w-2 h-2 me-1 bg-yellow-500 rounded-full"></span>
          {status}
        </span>
      )}
      {active == 3 && (
        <span className="items-center bg-red-100 text-red-800 text-xs font-medium px-2.5 py-0.5 rounded-full dark:bg-red-900 dark:text-red-300 hidden md:inline-flex">
          <span className="w-2 h-2 me-1 bg-red-500 rounded-full"></span>
          {status ? status : "X"}
        </span>
      )}
    </>
  );
};
