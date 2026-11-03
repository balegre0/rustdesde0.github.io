import { defineCollection } from 'astro:content';
import { z } from 'astro/zod';
import { glob } from 'astro/loaders';
import { docsLoader } from '@astrojs/starlight/loaders';
import { docsSchema } from '@astrojs/starlight/schema';
import { topicSchema } from 'starlight-sidebar-topics/schema';

// Campos propios de CONTENIDO.md §2 (lecciones) y §5 (referencia). Todos opcionales.
// Solo para depurar la plataforma: RDC_SOLO="demo/**,curso/02-*/**" construye un subconjunto de páginas.
const solo = process.env.RDC_SOLO?.split(',').map((s) => s.trim()).filter(Boolean);
// RDC_EXCLUIR="curso/03-*/05-*.mdx" excluye páginas rotas mientras se arreglan.
const excluir = process.env.RDC_EXCLUIR?.split(',').map((s) => '!' + s.trim()).filter((s) => s.length > 1) ?? [];
const loader = excluir.length && !solo?.length
  ? glob({ base: './src/content/docs', pattern: ['**/[^_]*.{md,mdx}', ...excluir] })
  : solo?.length
  ? glob({ base: './src/content/docs', pattern: ['index.mdx', 'repaso.mdx', 'progreso.mdx', 'curso/index.mdx', 'ref/index.mdx', ...solo] })
  : docsLoader();

const texto = z.union([z.string(), z.number()]).transform(String);

export const collections = {
  docs: defineCollection({
    loader,
    schema: docsSchema({
      extend: topicSchema.extend({
        duracion: z.number().int().positive().max(120).optional(),
        desde: texto.optional(),
        requiere: z.array(z.string()).optional(),
        tarjetas: z.array(z.object({ p: z.string(), r: z.string() })).optional(),
        // La idea de la lección en una sola frase, y una comparación con algo cotidiano (se ven arriba).
        frase: z.string().optional(),
        imagina: z.string().optional(),
        insignias: z.array(z.string()).optional(),
      }),
    }),
  }),
};
