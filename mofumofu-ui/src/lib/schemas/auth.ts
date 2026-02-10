import * as v from 'valibot';

export const emailSchema = v.pipe(
	v.string(),
	v.nonEmpty('Email is required.'),
	v.email('Please enter a valid email address.')
);

export const passwordSchema = v.pipe(
	v.string(),
	v.nonEmpty('Password is required.'),
	v.minLength(8, 'Password must be at least 8 characters.')
);

export const handleSchema = v.pipe(
	v.string(),
	v.nonEmpty('Handle is required.'),
	v.minLength(3, 'Handle must be at least 3 characters.'),
	v.maxLength(20, 'Handle must be at most 20 characters.'),
	v.regex(/^\w+$/, 'Only letters, numbers, and underscores allowed.')
);

export const displayNameSchema = v.pipe(
	v.string(),
	v.nonEmpty('Display name is required.'),
	v.maxLength(50, 'Display name must be at most 50 characters.')
);

export const loginSchema = v.object({
	email: emailSchema,
	password: v.pipe(v.string(), v.nonEmpty('Password is required.'))
});

export const signupSchema = v.object({
	display_name: displayNameSchema,
	email: emailSchema,
	handle: handleSchema,
	password: passwordSchema
});

export const resetPasswordSchema = v.pipe(
	v.object({
		new_password: passwordSchema,
		confirm_password: v.string()
	}),
	v.forward(
		v.partialCheck(
			[['new_password'], ['confirm_password']],
			(input) => input.new_password === input.confirm_password,
			'Passwords do not match.'
		),
		['confirm_password']
	)
);

export const totpCodeSchema = v.pipe(
	v.string(),
	v.nonEmpty('Code is required.'),
	v.regex(/^[\da-zA-Z]{6,8}$/, 'Enter a 6-digit code or 8-character backup code.')
);

export function validateField<T>(
	schema: v.BaseSchema<string, T, v.BaseIssue<unknown>>,
	value: string
): string | null {
	const result = v.safeParse(schema, value);
	if (result.success) return null;
	return result.issues[0]?.message ?? 'Invalid value.';
}
