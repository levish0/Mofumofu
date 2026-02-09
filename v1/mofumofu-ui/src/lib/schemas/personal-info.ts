import * as v from 'valibot';
import * as m from '../../paraglide/messages';

export function createPersonalInfoSchema() {
	return v.object({
		handle: v.pipe(
			v.string(),
			v.minLength(1, m.validation_handle_required()),
			v.minLength(3, m.validation_handle_min_length()),
			v.maxLength(20, m.validation_handle_max_length()),
			v.regex(/^[a-zA-Z0-9_]+$/, m.validation_handle_invalid_format())
		),
		name: v.pipe(
			v.string(),
			v.minLength(1, m.validation_name_required()),
			v.minLength(3, m.validation_name_min_length()),
			v.maxLength(20, m.validation_name_max_length())
		),
		bio: v.pipe(v.string(), v.maxLength(200, m.validation_bio_max_length())),
		location: v.pipe(v.string(), v.maxLength(30, m.validation_location_max_length())),
		website: v.union([
			v.literal(''), // Allow empty string
			v.pipe(v.string(), v.maxLength(50, m.validation_website_max_length()), v.url(m.validation_website_invalid_url()))
		]),
		profileImage: v.nullable(v.string()),
		bannerImage: v.nullable(v.string()),
		profileImageFile: v.nullable(v.instance(Blob)),
		bannerImageFile: v.nullable(v.instance(Blob))
	});
}

export type PersonalInfo = v.InferInput<ReturnType<typeof createPersonalInfoSchema>>;
