import { Parcel } from "momo_core/parcel";
import { Vector, VectorElement } from "momo_components/collections/vector"
import { Jar } from "momo_components/wrappers/jar";

export async function files_menu(parcel: Parcel): Promise<VectorElement> {
	parcel.header("Content-Type", "application/json");
	const icons = await parcel.get("/comp-icons", "files-menu");
	// console.log(icons);

	const menu = new Vector("files-menu", icons as JSON);
	menu.order("bin-add", "bin-up", "bin-down", "bin-del");

	menu.nodes().forEach((n: [string, Element]) => {
		const name = n[0];
		const node = n[1];
		const jar = new Jar(node);
		jar.make_key((e: Event) => console.log("ive been clicked", e.target));
		menu.update(name, jar);
	});

	const vec = menu.to_element();
	vec.direction("row");

	return vec;
}


