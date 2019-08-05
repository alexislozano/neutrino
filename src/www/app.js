let h = superfine.h;
let patch = superfine.patch;

const node = document.getElementById("app")

function render(obj) {
    patch(
        node,
        widget.render(obj)
    )
}

function invoke(arg) {
    window.external.invoke(JSON.stringify(arg));
}

window.onload = function() {
    invoke({ event: "init", source: "app" });
}