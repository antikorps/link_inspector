import type { Link } from "@customTypes/link";
import { atom } from "nanostores";

export const links_list = atom([] as Link[]);
export const links_loading = atom(false);
