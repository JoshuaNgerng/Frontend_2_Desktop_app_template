import { Project, SyntaxKind } from "ts-morph"
import camelCase from "camelcase"

const project = new Project({
  tsConfigFilePath: "./tsconfig.json",
})

const sourceFiles = project.getSourceFiles("src/schemas/**/*.ts")

for (const file of sourceFiles) {
  file.forEachDescendant((node) => {
    if (
      node.getKind() === SyntaxKind.PropertySignature ||
      node.getKind() === SyntaxKind.PropertyDeclaration
    ) {
      const prop = node.asKindOrThrow(
        node.getKind() as SyntaxKind.PropertySignature
      )

      const name = prop.getName()

      // skip already camelCase
      if (!name.includes("_")) return

      const newName = camelCase(name)

      prop.rename(newName)

      console.log(`${name} -> ${newName}`)
    }
  })
}

project.saveSync()