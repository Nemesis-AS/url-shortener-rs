window.onload = () => main();

function main() {
    document.getElementById("linkForm").addEventListener("submit", e => {
        e.preventDefault();

        let link = document.getElementById("linkInput").value;
        submitLink(link);
    });

    updateTable();
}

async function submitLink(link) {
    let res = await fetch("/shorten", {
        method: "POST",
        body: JSON.stringify({
            link
        })
    });
    let json = await res.json();
    console.log(json);
    saveLink(json.id, json.link);
}

function saveLink(key, link) {
    let data = loadLinks();
    data[key] = link;
    localStorage.setItem("linkData", JSON.stringify(data));

    updateTable();
}

function loadLinks() {
    let rawData = localStorage.getItem("linkData");
    if (!rawData) return {};

    return JSON.parse(rawData);
}

function updateTable() {
    const data = loadLinks();
    const table = document.getElementById("linkBody");
    table.innerHTML = "";

    Object.keys(data).forEach(key => {
        const row = createTableRow([data[key], key]);
        table.appendChild(row);
    });
}

function createTableRow(data) {
    const row = document.createElement("tr");

    data.forEach(item => {
        const cell = document.createElement("td");

        const link = document.createElement("a");
        link.setAttribute("href", item);

        link.appendChild(document.createTextNode(item));
        cell.appendChild(link);
        row.appendChild(cell);
    });

    return row;
}

