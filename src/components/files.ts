import { make, type Maybe } from "momo_core/core";
import { Parcel } from "momo_core/parcel";
import { ShadowContainer } from "momo_components/wrappers/container";

import { files_tree as files_tree } from "./files/files-tree";
import { files_meta as files_meta } from "./files/files-meta";
import { files_menu as files_menu } from "./files/files-menu";

export async function files(parcel: Parcel): Promise<ShadowContainer> {
	const menu = await files_menu(parcel);
	const tree = await files_tree(parcel);
	const meta = await files_meta(parcel);
	const cont = new ShadowContainer("files", menu, tree, meta);

	cont.push("menu", menu);
	cont.push("tree", tree);
	cont.push("meta", meta);

	cont.css("styles/files.css");

	return cont;
}
