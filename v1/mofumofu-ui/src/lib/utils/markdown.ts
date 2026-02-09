import * as m from '../../paraglide/messages';

export interface TocItem {
	level: number;
	text: string;
	id: string;
}

export interface MarkdownProcessResult {
	htmlContent: string;
	tocItems: TocItem[];
}

interface MarkdownNode {
	type: string;
	tagName?: string;
	value?: string;
	children?: MarkdownNode[];
	properties?: { id?: string; [key: string]: unknown };
}

export async function processMarkdown(markdown: string): Promise<MarkdownProcessResult> {
	try {
		const { unified } = await import('unified');
		const { default: remarkParse } = await import('remark-parse');
		const { default: remarkGfm } = await import('remark-gfm');
		const { default: remarkBreaks } = await import('remark-breaks');
		const { default: remarkToc } = await import('remark-toc');
		const { default: remarkMath } = await import('remark-math');
		const { default: remarkEmoji } = await import('remark-emoji');
		const { default: remarkGithubBlockquoteAlert } = await import('remark-github-blockquote-alert');
		const { default: remarkRehype } = await import('remark-rehype');
		const { default: rehypeKatex } = await import('rehype-katex');
		const { default: rehypeHighlight } = await import('rehype-highlight');
		const { default: rehypeSlug } = await import('rehype-slug');
		const { default: rehypeSanitize, defaultSchema } = await import('rehype-sanitize');
		const { default: rehypeStringify } = await import('rehype-stringify');
		const { default: rehypeRaw } = await import('rehype-raw');

		let tocItems: TocItem[] = [];

		// TOC extraction plugin
		const tocPlugin = () => {
			return (tree: MarkdownNode) => {
				const headings: TocItem[] = [];

				const extractText = (node: MarkdownNode): string => {
					if (node.type === 'text') {
						return node.value || '';
					}
					if (node.children) {
						return node.children.map(extractText).join('');
					}
					return '';
				};

				const visit = (node: MarkdownNode) => {
					if (node.tagName && /^h[1-6]$/.test(node.tagName)) {
						const level = parseInt(node.tagName.charAt(1));
						const text = extractText(node).trim();
						const id = node.properties?.id;

						if (id && text) {
							headings.push({ level, text, id });
						}
					}
					node.children?.forEach(visit);
				};

				visit(tree);
				tocItems = headings;
			};
		};

		// Sanitize schema for GFM + KaTeX + Code highlighting
		const sanitizeSchema = {
			...defaultSchema,
			clobberPrefix: '', // user-content- prefix 제거
			tagNames: [
				// 기본 HTML 태그들 (명시적으로 나열)
				'div',
				'span',
				'p',
				'br',
				'hr',
				'h1',
				'h2',
				'h3',
				'h4',
				'h5',
				'h6',
				'strong',
				'em',
				'b',
				'i',
				'u',
				's',
				'mark',
				'small',
				'blockquote',
				'pre',
				'code',
				'ul',
				'ol',
				'li',
				'dl',
				'dt',
				'dd',
				'table',
				'thead',
				'tbody',
				'tfoot',
				'tr',
				'th',
				'td',
				'a',
				'img',

				// defaultSchema의 태그들도 포함
				...(defaultSchema.tagNames || []),

				// GFM 확장
				'input',
				'details',
				'summary',
				'del',
				'ins',
				'section',
				'aside',

				// KaTeX 수학 요소들
				'math',
				'semantics',
				'mrow',
				'mi',
				'mo',
				'mn',
				'msup',
				'msub',
				'mfrac',
				'munder',
				'mover',
				'munderover',
				'mtable',
				'mtr',
				'mtd',
				'mspace',
				'mtext',
				'annotation',
				'mstyle',
				'merror',
				'mpadded',
				'mphantom',
				'menclose',

				// SVG 지원
				'svg',
				'g',
				'path',
				'rect',
				'circle',
				'ellipse',
				'line',
				'polyline',
				'polygon',
				'text',
				'tspan',
				'defs',
				'marker',
				'use'
			],

			attributes: {
				// 모든 요소에 허용할 속성들
				'*': ['id', 'className', 'style', 'title', 'dir', 'lang', 'data-*', 'aria-*'],

				// 링크
				a: ['href', 'title', 'target', 'rel'],

				// 이미지
				img: ['src', 'alt', 'title', 'width', 'height', 'loading', 'decoding'],

				// 폼 요소
				input: ['type', 'disabled', 'checked', 'value'],

				// 테이블
				th: ['scope', 'colspan', 'rowspan', 'headers'],
				td: ['colspan', 'rowspan', 'headers'],
				table: ['summary'],

				// details/summary
				details: ['open'],

				// SVG 속성들
				svg: ['width', 'height', 'viewBox', 'xmlns', 'fill', 'stroke', 'preserveAspectRatio'],
				g: ['transform', 'fill', 'stroke'],
				path: ['d', 'fill', 'stroke', 'strokeWidth'],
				rect: ['x', 'y', 'width', 'height', 'fill', 'stroke'],
				circle: ['cx', 'cy', 'r', 'fill', 'stroke'],
				ellipse: ['cx', 'cy', 'rx', 'ry', 'fill', 'stroke'],
				line: ['x1', 'y1', 'x2', 'y2', 'stroke'],
				text: ['x', 'y', 'fill', 'fontSize', 'textAnchor'],
				tspan: ['x', 'y', 'dx', 'dy', 'rotate', 'textLength', 'lengthAdjust'],
				defs: ['id'],
				marker: ['id', 'viewBox', 'refX', 'refY', 'markerUnits', 'markerWidth', 'markerHeight', 'orient'],
				use: ['href'],

				// KaTeX 속성들
				math: ['xmlns', 'display'],
				mrow: ['mathcolor', 'mathbackground'],
				mi: ['mathvariant', 'mathcolor'],
				mo: ['form', 'fence', 'separator', 'lspace', 'rspace', 'stretchy', 'symmetric', 'maxsize', 'minsize'],
				mn: ['mathvariant', 'mathcolor'],
				mfrac: ['linethickness', 'numalign', 'denomalign'],
				msup: ['superscriptshift'],
				msub: ['subscriptshift'],
				mspace: ['width', 'height', 'depth'],
				menclose: ['notation'],
				mpadded: ['width', 'height', 'depth', 'lspace', 'rspace']
			},

			protocols: {
				href: ['http', 'https', 'mailto', 'tel'],
				src: ['http', 'https', 'data']
			},

			// 중요: style 속성을 완전히 허용
			strip: [], // 아무것도 strip하지 않음
			clobber: [] // 아무것도 clobber하지 않음
		};

		const result = await unified()
			.use(remarkParse)
			.use(remarkGfm)
			.use(remarkBreaks)
			.use(remarkToc)
			.use(remarkMath)
			.use(remarkEmoji)
			.use(remarkGithubBlockquoteAlert)
			.use(remarkRehype, { allowDangerousHtml: true })
			.use(rehypeRaw)
			.use(rehypeKatex)
			.use(rehypeHighlight)
			.use(rehypeSlug, { prefix: 'h-' }) // 헤딩에 ID 생성
			.use(tocPlugin) // slug 이후, sanitize 이전에 실행
			.use(rehypeSanitize, sanitizeSchema)
			.use(rehypeStringify, { allowDangerousHtml: true })
			.process(markdown);

		return {
			htmlContent: String(result),
			tocItems: tocItems
		};
	} catch (error) {
		console.error('Markdown processing error:', error);
		return {
			htmlContent: `<p>${m.markdown_processing_error()}</p>`,
			tocItems: []
		};
	}
}
