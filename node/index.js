const babel = require("babel-core");
const parser = require("@babel/parser");
const fs = require("fs");

function getVisitor() {
  return {
    CallExpression(path) {
      const callee = path.get("callee");

      if (Array.isArray(callee) || !callee.isMemberExpression()) {
        return;
      }

      const object = callee.get("object");
      const prop = callee.get("property");

      if (Array.isArray(object) || Array.isArray(prop)) return;

      if (
        object.isIdentifier({ name: "console" }) &&
        !prop.isIdentifier({ name: "error" }) &&
        !prop.isIdentifier({ name: "warn" })
      ) {
        console.log("found console log");
      }
    },
  };
}

const file = fs.readFileSync("foo.js");

const ast = parser.parse(file.toString());

babel.traverse(ast, getVisitor());
