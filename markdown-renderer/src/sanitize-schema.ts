import { defaultSchema } from 'rehype-sanitize';
import type { Schema } from 'rehype-sanitize';

/**
 * Custom rehype-sanitize schema: extends default with GFM + KaTeX + highlight.js + SVG support.
 *
 * - Based on defaultSchema
 * - `style` attribute globally disallowed (XSS vector)
 * - `className` allowed globally (required by highlight.js / KaTeX)
 */
export const sanitizeSchema: Schema = {
	...defaultSchema,
	tagNames: [
		...(defaultSchema.tagNames ?? []),

		// GFM extensions
		'input',
		'details',
		'summary',
		'del',
		'ins',
		'section',
		'aside',

		// KaTeX MathML
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

		// SVG
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
		...defaultSchema.attributes,

		// Global: style stays blocked, but className is needed for syntax highlighting and KaTeX output.
		'*': [...(defaultSchema.attributes?.['*'] ?? []), 'className', 'title', 'dir', 'lang'],

		// Links
		a: [...(defaultSchema.attributes?.a ?? []), 'title', 'target', 'rel'],

		// Images
		img: [
			...(defaultSchema.attributes?.img ?? []),
			'alt',
			'title',
			'width',
			'height',
			'loading',
			'decoding'
		],

		// GFM task list checkbox
		input: [...(defaultSchema.attributes?.input ?? []), ['checked', true]],

		// Tables
		th: ['scope', 'colspan', 'rowspan', 'headers'],
		td: ['colspan', 'rowspan', 'headers'],
		table: ['summary'],

		// Details/summary
		details: ['open'],

		// Code blocks: highlight.js uses className
		code: [...(defaultSchema.attributes?.code ?? []), ['className', /^language-/, 'hljs']],
		span: [
			...(defaultSchema.attributes?.span ?? []),
			['className', /^hljs-/, 'hljs', /^katex/, /^katex-/]
		],
		pre: [...(defaultSchema.attributes?.pre ?? []), ['className', 'hljs']],
		div: [
			...(defaultSchema.attributes?.div ?? []),
			['className', 'katex', 'katex-display', 'math', 'math-inline', 'math-display']
		],

		// SVG
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
		marker: [
			'id',
			'viewBox',
			'refX',
			'refY',
			'markerUnits',
			'markerWidth',
			'markerHeight',
			'orient'
		],
		use: ['href'],

		// KaTeX MathML
		math: ['xmlns', 'display'],
		mrow: ['mathcolor', 'mathbackground'],
		mi: ['mathvariant', 'mathcolor'],
		mo: [
			'form',
			'fence',
			'separator',
			'lspace',
			'rspace',
			'stretchy',
			'symmetric',
			'maxsize',
			'minsize'
		],
		mn: ['mathvariant', 'mathcolor'],
		mfrac: ['linethickness', 'numalign', 'denomalign'],
		msup: ['superscriptshift'],
		msub: ['subscriptshift'],
		mspace: ['width', 'height', 'depth'],
		menclose: ['notation'],
		mpadded: ['width', 'height', 'depth', 'lspace', 'rspace'],
		annotation: ['encoding']
	},

	protocols: {
		...defaultSchema.protocols,
		href: [...(defaultSchema.protocols?.href ?? []), 'tel']
	}
};
