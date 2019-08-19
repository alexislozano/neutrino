let node = document.getElementById("app");

function render(template) {
    morphdom(node, template);
}

function invoke(arg) {
    window.external.invoke(JSON.stringify(arg));
}

window.onload = function() {
    invoke({ event: "init", source: "app", value: "app" });
}