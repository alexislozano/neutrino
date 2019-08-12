const node = document.getElementById("app");
let child = document.createElement("div");
node.appendChild(child);

function render(template) {
    morphdom(child, template);
}

function invoke(arg) {
    window.external.invoke(JSON.stringify(arg));
}

window.onload = function() {
    invoke({ event: "init", source: "app", value: "app" });
}