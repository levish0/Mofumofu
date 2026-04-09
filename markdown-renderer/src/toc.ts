/**
 * Custom rehype plugin: extracts TOC items from headings.
 *
 * Must run after rehype-slug (to have IDs) and before rehype-sanitize.
 * Walks heading nodes and collects level, text, and id.
 */

export interface TocItem {
	level: number;
	text: string;
	id: string;
}

interface HastNode {
	type: string;
	tagName?: string;
	value?: string;
	children?: HastNode[];
	properties?: Record<string, unknown>;
}

function extractText(node: HastNode): string {
	if (node.type === 'text') return node.value ?? '';
	if (node.children) return node.children.map(extractText).join('');
	return '';
}

/**
 * Rehype plugin factory.
 * Accepts a tocItems array that will be populated with extracted headings.
 */
export function tocPlugin(tocItems: TocItem[]) {
	return () => {
		return (tree: HastNode) => {
			const visit = (node: HastNode) => {
				if (node.tagName && /^h[1-6]$/.test(node.tagName)) {
					const level = parseInt(node.tagName.charAt(1));
					const text = extractText(node).trim();
					const id = node.properties?.id as string | undefined;

					if (id && text) {
						tocItems.push({ level, text, id });
					}
				}
				node.children?.forEach(visit);
			};

			visit(tree);
		};
	};
}
