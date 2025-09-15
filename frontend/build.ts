import { build } from "bun";
import { copyFile, readFile, rm, writeFile } from "fs/promises";
import path from "path";

const outdir = "./dist";
const backenddir = "../src/resources";

async function bundleProject() {
  await rm(outdir, { recursive: true, force: true });

  const result = await build({
    entrypoints: ["src/index.ts"],
    outdir: outdir,
    target: "browser",
    minify: false,
    sourcemap: "external",
  });

  result.logs.forEach(console.log);
  if (result.success) {
    console.log("✅ Build completed successfully");
    console.log(
      "Output files:",
      result.outputs.map((output) => path.basename(output.path)),
    );
  } else {
    console.error("❌ Build failed");
    result.logs.forEach(console.error);
    process.exit(1);
  }

  await copyToBackend();
}

async function copyToBackend() {
  let html = await readFile("src/index.html", "utf-8");
  html = html.replace(/index.ts/g, "index.js");
  await writeFile(path.join(backenddir, "index.html"), html);
  await copyFile(
    path.join(outdir, "index.js"),
    path.join(backenddir, "index.js"),
  );
  await copyFile("src/style.css", path.join(backenddir, "style.css"));
}

bundleProject().catch(console.error);
