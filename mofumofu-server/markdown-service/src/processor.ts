/**
 * Markdown-to-HTML processor.
 *
 * Builds a unified pipeline and exports processMarkdown().
 *
 * Improvements over V1:
 * - Top-level imports (no per-call dynamic imports)
 * - Removed remark-toc (custom tocPlugin replaces it)
 * - tocItems injected per call instead of closure mutation
 */

import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkGfm from 'remark-gfm';
import remarkBreaks from 'remark-breaks';
import remarkMath from 'remark-math';
import remarkEmoji from 'remark-emoji';
import remarkGithubBlockquoteAlert from 'remark-github-blockquote-alert';
import remarkRehype from 'remark-rehype';
import rehypeRaw from 'rehype-raw';
import rehypeKatex from 'rehype-katex';
import rehypeHighlight from 'rehype-highlight';
import rehypeSlug from 'rehype-slug';
import rehypeSanitize from 'rehype-sanitize';
import rehypeStringify from 'rehype-stringify';

import { sanitizeSchema } from './sanitize-schema';
import { tocPlugin, type TocItem } from './toc';

export interface MarkdownResult {
	htmlContent: string;
	tocItems: TocItem[];
}

/**
 * Converts a markdown string to rendered HTML + TOC.
 *
 * Pipeline order:
 *  1. remark-parse        — Markdown -> mdast
 *  2. remark-gfm          — GFM extensions (tables, strikethrough, checkboxes, etc.)
 *  3. remark-breaks        — Soft line breaks -> <br>
 *  4. remark-math          — Math expressions ($...$, $$...$$)
 *  5. remark-emoji         — :shortcode: -> emoji
 *  6. remark-github-alert  — > [!NOTE] etc. GitHub alert blockquotes
 *  7. remark-rehype        — mdast -> hast (allowDangerousHtml: preserves raw HTML)
 *  8. rehype-raw           — Parses raw HTML in markdown
 *  9. rehype-katex         — Math -> MathML
 * 10. rehype-highlight     — Code block syntax highlighting
 * 11. rehype-slug          — Assigns id to headings (prefix: 'h-')
 * 12. tocPlugin            — Extracts TOC from headings
 * 13. rehype-sanitize      — XSS prevention
 * 14. rehype-stringify      — hast -> HTML string
 */
export async function processMarkdown(markdown: string): Promise<MarkdownResult> {
	const tocItems: TocItem[] = [];

	const result = await unified()
		.use(remarkParse)
		.use(remarkGfm)
		.use(remarkBreaks)
		.use(remarkMath)
		.use(remarkEmoji)
		.use(remarkGithubBlockquoteAlert)
		.use(remarkRehype, { allowDangerousHtml: true })
		.use(rehypeRaw)
		.use(rehypeKatex)
		.use(rehypeHighlight)
		.use(rehypeSlug, { prefix: 'h-' })
		.use(tocPlugin(tocItems))
		.use(rehypeSanitize, sanitizeSchema)
		.use(rehypeStringify, { allowDangerousHtml: true })
		.process(markdown);

	return {
		htmlContent: String(result),
		tocItems
	};
}
