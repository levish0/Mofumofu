import type { CommentInfo } from '$lib/api/comment/types';

export interface CommentNode {
	comment: CommentInfo;
	children: CommentNode[];
}

/**
 * 플랫한 댓글 리스트를 트리 구조로 변환
 * @param comments 플랫한 댓글 리스트
 * @param parentId 부모 댓글 ID (null이면 최상위 댓글들)
 * @returns 트리 구조의 댓글 노드 배열
 */
export function buildCommentTree(comments: CommentInfo[], parentId: string | null = null): CommentNode[] {
	const result: CommentNode[] = [];

	// 해당 부모의 직계 자식 댓글들 찾기
	const directChildren = comments.filter((comment) => comment.parent_id === parentId);

	for (const comment of directChildren) {
		const node: CommentNode = {
			comment,
			children: buildCommentTree(comments, comment.id) // 재귀적으로 자식들 찾기
		};
		result.push(node);
	}

	// 생성 시간 기준 정렬 (최신순)
	return result.sort((a, b) => new Date(b.comment.created_at).getTime() - new Date(a.comment.created_at).getTime());
}

/**
 * 댓글을 플랫 리스트에 추가하고 트리 재구성
 */
export function addCommentToList(comments: CommentInfo[], newComment: CommentInfo): CommentInfo[] {
	// 중복 확인 후 추가
	const existingIndex = comments.findIndex((c) => c.id === newComment.id);
	if (existingIndex >= 0) {
		// 이미 존재하면 업데이트
		comments[existingIndex] = newComment;
		return [...comments];
	} else {
		// 새로 추가
		return [...comments, newComment];
	}
}
