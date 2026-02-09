import { Elysia, t } from 'elysia';
import { processMarkdown } from './markdown';

const app = new Elysia()
  .get('/', () => 'Mofumofu Markdown Service')
  
  .get('/health', () => ({ 
    status: 'ok',
    service: 'mofumofu-markdown-service',
    timestamp: new Date().toISOString()
  }))
  
  .post('/render', async ({ body }) => {
    try {
      const { htmlContent, tocItems } = await processMarkdown(body.markdown);
      
      return {
        success: true,
        data: {
          htmlContent,
          tocItems
        }
      };
    } catch (error) {
      console.error('Markdown processing error:', error);
      return {
        success: false,
        error: 'Failed to process markdown',
        data: {
          htmlContent: '<p>마크다운 처리 중 오류가 발생했습니다.</p>',
          tocItems: []
        }
      };
    }
  }, {
    body: t.Object({
      markdown: t.String()
    })
  })
  
  .listen(6700);

console.log(`🦊 Mofumofu Markdown Service is running at http://localhost:6700`);