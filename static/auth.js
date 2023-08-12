const mainDiv = document.querySelector("main");

const main = () => {
    if (window.location.search.includes("login")) {
        setPage(1);
    }

    document.querySelectorAll(".link-btn").forEach(btn => {
        btn.addEventListener("click", e => {
            setPage(Number(e.target.dataset.page));
        });
    });

    document.querySelectorAll("form").forEach(form => {
        form.addEventListener("submit", e => {
            e.preventDefault();

            if (form.id == "register") {
                registerUser();
            } else if (form.id = "login") {
                loginUser();
            }
        });
    });
};

function setPage(pageID) {
    switch (pageID) {
        case 0:
            mainDiv.classList.remove("show-lower");
            break;
        case 1:
            mainDiv.classList.add("show-lower");
            break;
    }
};

function registerUser() {
    const username = document.getElementById("rUsername").value;
    const password = document.getElementById("rPassword").value;
    const password2 = document.getElementById("rconfirmPass").value;

    if (!(password === password2)) {
        console.warn("Password Do Not Match");
        return;
    }

    fetch("/register", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            username,
            password
        }),
    }).then(res => res.json()).then(json => {
        if (json.status == 200) {
            window.location = "/";
            localStorage.setItem("user", JSON.stringify({ token: json.token, username: json.username }));
        } else {
            console.error("Could not register user!");
        }
    }).catch(err => {
        console.error("An Error Occurred while registering!");
        console.error(err);
    });
}

function loginUser() {
    const username = document.getElementById("lUsername").value;
    const password = document.getElementById("lPassword").value;

    fetch("/login", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            username,
            password
        }),
    }).then(res => res.json()).then(json => {
        if (json.status == 200) {
            localStorage.setItem("user", JSON.stringify({ token: json.token, username: json.username }));
            window.location = "/";
        } else {
            console.error("Could not login user in!");
        }
    }).catch(err => {
        console.error("An Error Occurred while logging in!");
        console.error(err);
    });
}

window.onload = main;