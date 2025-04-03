import { Parcel } from "momo_core/parcel";
import { MatrixElement } from "momo_components/collections/matrix"

export async function files_meta(parcel: Parcel) {
	parcel.header("Content-Type", "application/json");
	const fs_root = await parcel.get("/files-meta", "dist");
	const vals = Object.values(fs_root).map((entry: JSON) => Object.values(entry));

	const meta = new MatrixElement("files-meta", ...COLUMNS);
	meta.extend(...vals);

	return meta;
}

const COLUMNS = ["name", "extension", "created", "modified", "accessed", "read-only", "size"];
