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

export async function processMarkdown(markdown: string): Promise<MarkdownResult> {
	try {
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
	} catch (error) {
		console.error('Markdown processing error:', error);
		return {
			htmlContent: '<p>Failed to process markdown.</p>',
			tocItems: []
		};
	}
}
