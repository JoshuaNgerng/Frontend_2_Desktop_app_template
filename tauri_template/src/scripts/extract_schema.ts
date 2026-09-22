import ts from "typescript";
import fs from "fs/promises";
import { glob } from "glob";
import yaml from "js-yaml";
import path from "path";

interface FieldSchema {
  name: string;
  type: string;
  nullable: boolean;
}

interface Schema {
  schema_name: string;
  fields: FieldSchema[];
}

function extractSchemasFromFile(fileName: string, sourceText: string): Schema[] {
  const sourceFile = ts.createSourceFile(
    fileName,
    sourceText,
    ts.ScriptTarget.Latest,
    true
  );

  const schemas: Schema[] = [];

  function getTypeString(typeNode?: ts.TypeNode): string {
    if (!typeNode) return "any";

    return typeNode.getText(sourceFile);
  }

  function parseMembers(
    members: ts.NodeArray<ts.TypeElement | ts.ClassElement>
  ): FieldSchema[] {
    const fields: FieldSchema[] = [];

    members.forEach(member => {
      // === INTERFACE PROPERTY ===
      if (ts.isPropertySignature(member)) {
        const name = member.name.getText(sourceFile);
        const type = getTypeString(member.type);
        const nullable = type.includes("null");

        fields.push({
          name,
          type: type.replace(" | null", "").replace("null", "").trim(),
          nullable,
        });
      }

      // === CLASS PROPERTY ===
      if (ts.isPropertyDeclaration(member)) {
        const name = member.name.getText(sourceFile);

        // skip methods/constructors automatically (they are different node types)

        const type = getTypeString(member.type);
        const initializer = member.initializer?.getText(sourceFile);

        const inferredType =
          type ||
          (initializer?.match(/^\d+$/)
            ? "number"
            : initializer?.startsWith("'") || initializer?.startsWith('"')
              ? "string"
              : "any");

        const nullable =
          inferredType.includes("null") ||
          member.questionToken !== undefined;

        fields.push({
          name,
          type: inferredType.replace(" | null", "").replace("null", "").trim(),
          nullable,
        });
      }
    });

    return fields;
  }

  function visit(node: ts.Node) {
    // ===== INTERFACE =====
    if (ts.isInterfaceDeclaration(node)) {
      schemas.push({
        schema_name: node.name.text,
        fields: parseMembers(node.members),
      });
    }

    // ===== CLASS =====
    if (ts.isClassDeclaration(node) && node.name) {
      schemas.push({
        schema_name: node.name.text,
        fields: parseMembers(node.members),
      });
    }

    ts.forEachChild(node, visit);
  }

  visit(sourceFile);

  return schemas;
}

async function generateYaml(input_dir: string, output_dir: string) {
  const files = await glob(input_dir);

  const allSchemas: Schema[] = [];

  for (const file of files) {
    const content = await fs.readFile(file, "utf8");
    allSchemas.push(...extractSchemasFromFile(file, content));
  }

  const output = yaml.dump(
    { schemas: allSchemas },
    { noRefs: true, lineWidth: -1 }
  );

  await fs.writeFile(
    path.resolve(output_dir),
    output,
    "utf8"
  );

  console.log(`Generated ${allSchemas.length} schemas`);
}

generateYaml("src/schemas/**/*.ts", "schemas.yaml").catch(console.error);