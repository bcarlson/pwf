import { readFile, writeFile } from 'node:fs/promises';
import { compile } from 'json-schema-to-typescript';

async function loadSchema(path) {
  const contents = await readFile(new URL(path, import.meta.url), 'utf8');
  return JSON.parse(contents);
}

async function writeTypes(path, contents) {
  await writeFile(new URL(path, import.meta.url), contents, 'utf8');
}

async function generate() {
  const planSchema = await loadSchema('../schema/pwf-v1.json');
  const historySchema = await loadSchema('../schema/pwf-history-v1.json');

  const planTypes = await compile(planSchema, 'PwfPlan', {
    bannerComment: ''
  });

  const historyTypes = await compile(historySchema, 'PwfHistory', {
    bannerComment: ''
  });

  await writeTypes('../src/generated/plan.ts', planTypes.trim() + '\n');
  await writeTypes('../src/generated/history.ts', historyTypes.trim() + '\n');
}

generate();
