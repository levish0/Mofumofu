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
