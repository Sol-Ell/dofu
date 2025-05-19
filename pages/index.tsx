const DEFAULT_PATH: string = "";

type FsEntry = {
    name: string,
    last_modified: string,
    ty: FsEntryType
};

type FsEntryType = | { Dir: {} }
    | { File: { size: number } };

const fetch_entries = async (path: string) => {
    let resp = await fetch("/api/get_folder_content?" + new URLSearchParams({
        path: DEFAULT_PATH,
    }), {
        method: "GET",
        headers: {
            "Content-Type": "JSON"
        }
    });

    if (resp.ok) {
        const json: { folder_content: Array<FsEntry> } = await resp.json();
        console.log(json);
        return json;
    }

    console.log(`Fetching folder content failed. ${resp.statusText}`);

    return null;
};

const update_content = (fs_entries: Array<FsEntry>) => {
    const table = document.getElementById("table-of-entries");
    if (table instanceof HTMLElement) {
        fs_entries.forEach(element => {
            const row = document.createElement("tr");

            const name = document.createElement("td");
            const last_modified = document.createElement("td");
            const size = document.createElement("td");

            name.textContent = element.name;
            last_modified.textContent = element.last_modified;
            if ("File" in element.ty) {
                size.textContent = element.ty.File.size.toString();
            }

            row.appendChild(name);
            row.appendChild(last_modified);
            row.appendChild(size);

            table.appendChild(row);
        });
        return;
    }
    console.log("Table of folder entries not found.");
};

var resp = await fetch_entries(DEFAULT_PATH);

if (resp != null) {
    update_content(resp.folder_content);
}

export { }