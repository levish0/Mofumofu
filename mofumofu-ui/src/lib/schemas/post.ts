import * as v from 'valibot';

export function createPostSchema() {
	return v.object({
		title: v.pipe(
			v.string(),
			v.minLength(1, 'Title is required.'),
			v.maxLength(80, 'Title must be 80 characters or less.')
		),
		content: v.pipe(v.string(), v.minLength(1, 'Content is required.')),
		slug: v.pipe(
			v.string(),
			v.minLength(1, 'Slug is required.'),
			v.maxLength(80, 'Slug must be 80 characters or less.'),
			v.regex(
				/^[^\s\/\?#\[\]@!$&'()*+,;=]+$/,
				'Slug contains invalid characters.'
			)
		),
		summary: v.optional(
			v.pipe(v.string(), v.maxLength(500, 'Summary must be 500 characters or less.'))
		),
		tags: v.optional(
			v.pipe(
				v.string(),
				v.transform((input) => {
					return input
						.split(',')
						.map((tag) => tag.trim())
						.filter((tag) => tag);
				}),
				v.maxLength(8, 'You can add up to 8 tags.')
			)
		)
	});
}

export type PostData = v.InferInput<ReturnType<typeof createPostSchema>>;
