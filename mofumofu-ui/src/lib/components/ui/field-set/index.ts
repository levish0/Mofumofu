import Root from '$lib/components/ui/field-set/field-set.svelte';
import Content from '$lib/components/ui/field-set/field-set-content.svelte';
import Footer from '$lib/components/ui/field-set/field-set-footer.svelte';
import Title from '$lib/components/ui/field-set/field-set-title.svelte';
import type {
	FieldSetRootProps,
	FieldSetTitleProps,
	FieldSetContentProps,
	FieldSetFooterProps
} from '$lib/components/ui/field-set/types';
import { tv, type VariantProps } from 'tailwind-variants';

export const fieldSetVariants = tv({
	base: 'border-border flex h-fit w-full flex-col rounded-lg border',
	variants: {
		variant: {
			default: 'border-border bg-card',
			destructive: 'border-destructive'
		}
	}
});

export type Variant = VariantProps<typeof fieldSetVariants>['variant'];

export {
	Root,
	Content,
	Footer,
	Title,
	type FieldSetRootProps as RootProps,
	type FieldSetTitleProps as TitleProps,
	type FieldSetContentProps as ContentProps,
	type FieldSetFooterProps as FooterProps
};
