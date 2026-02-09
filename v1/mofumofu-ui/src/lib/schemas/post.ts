import * as v from 'valibot';
import * as m from '../../paraglide/messages';

export function createPostSchema() {
	return v.object({
		title: v.pipe(
			v.string(),
			v.minLength(1, m.post_validation_title_required()),
			v.maxLength(80, m.post_validation_title_max_length())
		),
		content: v.pipe(v.string(), v.minLength(1, m.post_validation_content_required())),
		slug: v.pipe(
			v.string(),
			v.minLength(1, m.post_validation_slug_required()),
			v.maxLength(80, m.post_validation_slug_max_length()),
			v.regex(/^[^\s\/\?#\[\]@!$&'()*+,;=]+$/, m.post_validation_slug_format())
		),
		summary: v.optional(v.pipe(v.string(), v.maxLength(500, m.post_validation_summary_max_length()))),
		tags: v.optional(
			v.pipe(
				v.string(),
				v.transform((input) => {
					const tagArray = input
						.split(',')
						.map((tag) => tag.trim())
						.filter((tag) => tag);
					return tagArray;
				}),
				v.maxLength(8, m.post_validation_tags_max_count())
			)
		)
	});
}

export type PostData = v.InferInput<ReturnType<typeof createPostSchema>>;
