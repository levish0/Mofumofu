import { describe, expect, it } from 'bun:test';

import { processMarkdown } from './processor';

describe('processMarkdown', () => {
	it('renders basic English markdown', async () => {
		const { htmlContent, tocItems } = await processMarkdown(
			'# Hello World\n\n- first item\n- second item\n\n[OpenAI](https://openai.com)'
		);

		expect(htmlContent).toContain('<h1 id="user-content-h-hello-world">Hello World</h1>');
		expect(htmlContent).toContain('<li>first item</li>');
		expect(htmlContent).toContain('<li>second item</li>');
		expect(htmlContent).toContain('<a href="https://openai.com">OpenAI</a>');
		expect(tocItems).toEqual([{ level: 1, text: 'Hello World', id: 'user-content-h-hello-world' }]);
	});

	it('matches GitHub-style soft line break behavior for markdown files', async () => {
		const { htmlContent } = await processMarkdown('첫째 줄\n둘째 줄');

		expect(htmlContent).toContain('<p>첫째 줄\n둘째 줄</p>');
		expect(htmlContent).not.toContain('<br>');
	});

	it('keeps explicit hard line breaks', async () => {
		const withTrailingSpaces = await processMarkdown('첫째 줄  \n둘째 줄');
		const withBackslash = await processMarkdown('첫째 줄\\\n둘째 줄');

		expect(withTrailingSpaces.htmlContent).toContain('<br>');
		expect(withBackslash.htmlContent).toContain('<br>');
	});

	it('requires a space after heading markers', async () => {
		const withoutSpace = await processMarkdown('#제목');
		const withSpace = await processMarkdown('# 제목');

		expect(withoutSpace.htmlContent).toContain('<p>#제목</p>');
		expect(withSpace.htmlContent).toContain('<h1 id="user-content-h-제목">제목</h1>');
	});

	it('requires a space after list markers', async () => {
		const unordered = await processMarkdown('-목록');
		const ordered = await processMarkdown('1.목록');
		const validUnordered = await processMarkdown('- 목록');
		const validOrdered = await processMarkdown('1. 목록');

		expect(unordered.htmlContent).toContain('<p>-목록</p>');
		expect(ordered.htmlContent).toContain('<p>1.목록</p>');
		expect(validUnordered.htmlContent).toContain('<li>목록</li>');
		expect(validOrdered.htmlContent).toContain('<li>목록</li>');
	});

	it('sanitizes dangerous raw HTML', async () => {
		const { htmlContent } = await processMarkdown(
			'<script>alert(1)</script><img src="https://example.com/x.png" onerror="alert(1)"><a href="javascript:alert(1)">click</a>'
		);

		expect(htmlContent).not.toContain('<script>');
		expect(htmlContent).not.toContain('alert(1)');
		expect(htmlContent).not.toContain('onerror');
		expect(htmlContent).not.toContain('javascript:');
		expect(htmlContent).toContain('<img src="https://example.com/x.png">');
		expect(htmlContent).toContain('<a>click</a>');
	});

	it('blocks data urls and keeps toc ids aligned with sanitized heading ids', async () => {
		const { htmlContent, tocItems } = await processMarkdown(
			'# Hello\n\n<h2 id="history">Manual Heading</h2>\n\n<img src="data:image/png;base64,AAAA" alt="inline">'
		);

		expect(htmlContent).toContain('<h1 id="user-content-h-hello">Hello</h1>');
		expect(htmlContent).toContain('<h2 id="user-content-history">Manual Heading</h2>');
		expect(htmlContent).toContain('<img alt="inline">');
		expect(tocItems).toEqual([
			{ level: 1, text: 'Hello', id: 'user-content-h-hello' },
			{ level: 2, text: 'Manual Heading', id: 'user-content-history' }
		]);
	});
});
