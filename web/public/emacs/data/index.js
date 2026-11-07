import { lessons1 } from './lessons-1.js';
import { lessons2 } from './lessons-2.js';
import { lessons3 } from './lessons-3.js';
import { lessons4 } from './lessons-4.js';
import { modules } from './modules.js';
import { glossary } from './glossary.js';

const all = [...lessons1, ...lessons2, ...lessons3, ...lessons4];
export const lessons = Object.fromEntries(all.map((l) => [l.id, l]));
export { modules, glossary };
