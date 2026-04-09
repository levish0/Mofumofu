import { describe, expect, it } from 'bun:test';

import { processMarkdown } from './processor';

describe('processMarkdown', () => {
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
		expect(withSpace.htmlContent).toContain('<h1 id="h-제목">제목</h1>');
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
});
