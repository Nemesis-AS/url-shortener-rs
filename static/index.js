// @todo

// 1) Save shortened links in LocalStorage
// 2) Populate Table with links shortened
// 3) BACKEND: Add Link Expiry


window.onload = () => main();

function main() {
    document.getElementById("linkForm").addEventListener("submit", e => {
        e.preventDefault();

        let link = document.getElementById("linkInput").value;
        // console.log(link);
        submitLink(link);
    });
}

async function submitLink(link) {
    let res = await fetch("/shorten", {
        method: "POST",
        body: JSON.stringify({
            link
        })
    });
    let json = await res.text();
    console.log(json);
}

// main();

