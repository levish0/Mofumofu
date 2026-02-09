import { Elysia, t } from 'elysia';
import { processMarkdown } from './processor';

const PORT = parseInt(process.env.PORT ?? '6700', 10);

const app = new Elysia()
	.get('/', () => 'Mofumofu Markdown Service')

	.get('/health', () => ({
		status: 'ok',
		service: 'mofumofu-markdown',
		timestamp: new Date().toISOString()
	}))

	.post(
		'/render',
		async ({ body, set }) => {
			try {
				const { htmlContent, tocItems } = await processMarkdown(body.markdown);

				return {
					success: true,
					data: { htmlContent, tocItems }
				};
			} catch (error) {
				console.error('Markdown processing error:', error);
				set.status = 500;
				return {
					success: false,
					error: 'Failed to process markdown'
				};
			}
		},
		{
			body: t.Object({
				markdown: t.String()
			})
		}
	)

	.listen(PORT);

console.log(`Mofumofu Markdown Service running at http://localhost:${app.server?.port}`);
