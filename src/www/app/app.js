let node = document.getElementById("app");

function render(template) {
    morphdom(node, template);
}

function emit(arg) {
    window.external.invoke(JSON.stringify(arg));
}

window.onload = function() {
    emit({ type: "Update" });
}