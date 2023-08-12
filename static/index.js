function main() {
    if (!localStorage.getItem("user")) {
        window.location = "/auth";
        return;
    }

    document.getElementById("username").innerText = JSON.parse(localStorage.getItem("user")).username;

    document.getElementById("linkForm").addEventListener("submit", e => {
        e.preventDefault();

        const linkInput = document.getElementById("linkInput");
        let link = linkInput.value;
        linkInput.value = "";
        submitLink(link);
    });

    document.getElementById("logout").addEventListener("click", logout);

    updateTable();
}

async function submitLink(link) {
    const userData = JSON.parse(localStorage.getItem("user"));
    if (!userData) {
        console.error("Cannot Shorten Link: User not logged in!");
        return;
    }

    let res = await fetch("/shorten", {
        method: "POST",
        headers: {
            "Authorization": `Bearer ${userData.token || "null"}`,
        },
        body: JSON.stringify({
            link,
        }),
    });
    let json = await res.json();
    console.log(json);
    updateTable();
}

async function fetchLinks() {
    const userData = JSON.parse(localStorage.getItem("user"));
    if (!userData) {
        console.error("Cannot Load Links: User not logged in!");
        return;
    }

    const res = await fetch("/get-user-links", {
        headers: {
            "Authorization": `Bearer ${userData.token || "null"}`,
        },
    });

    const links = await res.json();
    return links;
}

async function updateTable() {
    const data = await fetchLinks();
    const table = document.getElementById("linkBody");
    table.innerHTML = "";

    for (idx in data) {
        const row = createTableRow([data[idx].link, data[idx].id]);
        table.appendChild(row);
    }
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

function logout() {
    localStorage.clear();
    window.location = "/auth?login";
}

window.onload = main;