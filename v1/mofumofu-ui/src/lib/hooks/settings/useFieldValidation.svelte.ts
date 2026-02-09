// src/lib/hooks/settings/useFieldValidation.svelte.ts

export interface FieldValidationConfig<T extends Record<string, any>> {
	onValidationChange?: (errors: Partial<T>) => void;
	validateForm?: () => void;
}

/**
 * 제네릭 필드 검증 관리 훅
 */
export function useFieldValidation<T extends Record<string, any>>(config: FieldValidationConfig<T> = {}) {
	const { onValidationChange, validateForm } = config;

	let localErrors = $state<Partial<T>>({});

	/**
	 * 제네릭 validation 핸들러 생성 함수
	 */
	const createValidationHandler = <K extends keyof T>(field: K) => {
		return (error?: string) => {
			if (error) {
				localErrors[field] = error as T[K];
			} else {
				delete localErrors[field];
			}

			// 객체 참조 업데이트로 reactivity 보장
			localErrors = { ...localErrors };

			// 콜백 호출
			onValidationChange?.(localErrors);
			validateForm?.();
		};
	};

	/**
	 * 모든 에러 클리어
	 */
	const clearAllErrors = () => {
		localErrors = {};
		onValidationChange?.(localErrors);
	};

	/**
	 * 특정 필드 에러 클리어
	 */
	const clearFieldError = <K extends keyof T>(field: K) => {
		delete localErrors[field];
		localErrors = { ...localErrors };
		onValidationChange?.(localErrors);
	};

	/**
	 * 특정 필드에 에러가 있는지 확인
	 */
	const hasFieldError = (field: keyof T) => {
		return field in localErrors && localErrors[field] != null;
	};

	return {
		get localErrors() {
			return localErrors;
		},
		createValidationHandler,
		clearAllErrors,
		clearFieldError,
		get hasErrors() {
			return Object.keys(localErrors).length > 0;
		},
		hasFieldError
	};
}
