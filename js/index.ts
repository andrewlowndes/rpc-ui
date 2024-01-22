import React from "react";
import { createRoot } from "react-dom/client";
import { Button } from "@mui/material";

//TODO: generate the types fot this automatically from the Rust types
const elements: any = {
    Button,
};

const rpc_methods: any = {
    say_hello,
    run_method,
    fetch_next,
    noop,
    append_to_body,
};

// rpc methods - should not have any return types
function noop() {}

function append_to_body(obj: any) {
    const domNode = document.getElementById('app');
    const root = createRoot(domNode);
    const elem = create_component(obj.component);
    root.render(elem);
}

function say_hello() {
    alert("hello");
}

function run_method(obj: any) {
    rpc_methods[obj.method](obj.params);
}

function fetch_next(obj: any) {
    fetch(obj.path).then(res => res.json()).then(run_method);
}

// utils
function create_component(obj: any): any {
    if (Array.isArray(obj)) {
        return obj.map(create_component);
    }

    switch (typeof(obj)) {
        case "boolean":
        case "number":
        case "string":
        case "undefined":
            return obj;
    }

    //for now just iterate through the attributes of the component tree and parse any commands 
    const attributes: any = {};
    
    Object.keys(obj.attributes).forEach((key) => {
        const val = obj.attributes[key];
        attributes[key] = val.method ? () => run_method(val) : val;
    });
    
    //instantiate the component tree via React.CreateElement and append to the document body
    const children = attributes.children ? create_component(attributes.children) : null;
    const elementClass = elements[obj.type] ?? obj.type;

    return React.createElement(elementClass, attributes, children);
}

// START
fetch_next({ path: "init" });
