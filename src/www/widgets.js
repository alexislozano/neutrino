const widget = {
    render: function (obj) {
        switch (obj.type) {
            case "button":
                return button.render(obj);
                break;
            case "container":
                return container.render(obj);
                break;
            case "label":
                return label.render(obj);
                break;        
            default:
                console.log("This widget is not implemented yet")
        }
    }
}

const button = {
    render: function(obj) {
        return h("button", { onclick: () => invoke({ event: "click", source: obj.name }), disabled: obj.disabled }, obj.text) 
    }
}

const container = {
    render: function(obj) {
        return h("div", { class: "container", style: obj.style }, obj.children.map((child) => {
            return widget.render(child);
        })) 
    }
}

const label = {
    render: function(obj) {
        return h("div", { class: "label", onclick: () => invoke({ event: "click", source: obj.name }) }, obj.text) 
    }
}