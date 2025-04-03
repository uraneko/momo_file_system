import { Parcel } from "momo_core/parcel";
import { TreeElement } from "momo_components/collections/tree"

export async function files_tree(parcel: Parcel) {
	parcel.header("Content-Type", "application/json");
	const fs_root = await parcel.get("/files-tree", "dist");

	return new TreeElement("files-tree", fs_root as JSON);
}
