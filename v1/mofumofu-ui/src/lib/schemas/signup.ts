import * as v from 'valibot';
import * as m from '../../paraglide/messages';

export function createSignupSchema() {
	return v.object({
		name: v.pipe(
			v.string(),
			v.minLength(1, m.signup_validation_name_required()),
			v.minLength(3, m.signup_validation_name_min_length()),
			v.maxLength(20, m.signup_validation_name_max_length())
		),
		email: v.pipe(
			v.string(),
			v.minLength(1, m.signup_validation_email_required()),
			v.email(m.signup_validation_email_invalid())
		),
		handle: v.pipe(
			v.string(),
			v.minLength(1, m.signup_validation_handle_required()),
			v.minLength(3, m.signup_validation_handle_min_length()),
			v.maxLength(20, m.signup_validation_handle_max_length()),
			v.regex(/^[a-zA-Z0-9_]+$/, m.signup_validation_handle_format())
		),
		password: v.pipe(
			v.string(),
			v.minLength(1, m.signup_validation_password_required()),
			v.minLength(8, m.signup_validation_password_min_length()),
			v.maxLength(128, m.signup_validation_password_max_length()),
			v.regex(/(?=.*[a-z])/, m.signup_validation_password_lowercase()),
			v.regex(/(?=.*[A-Z])/, m.signup_validation_password_uppercase()),
			v.regex(/(?=.*\d)/, m.signup_validation_password_numbers()),
			v.regex(/(?=.*[!@#$%^&*(),.?":{}|<>])/, m.signup_validation_password_special())
		)
	});
}

export type SignupInfo = v.InferInput<ReturnType<typeof createSignupSchema>>;
