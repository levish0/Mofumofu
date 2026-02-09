// UI Components - Namespaced to avoid conflicts
import * as Badge from './components/ui/badge/index';
import * as Button from './components/ui/button/index';
import * as Calendar from './components/ui/calendar/index';
import * as Command from './components/ui/command/index';
import * as Dialog from './components/ui/dialog/index';
import * as HoverCard from './components/ui/hover-card/index';
import * as Input from './components/ui/input/index';
import * as InputOTP from './components/ui/input-otp/index';
import * as Popover from './components/ui/popover/index';
import * as Select from './components/ui/select/index';
import * as Separator from './components/ui/separator/index';
import * as Switch from './components/ui/switch/index';
import * as Textarea from './components/ui/textarea/index';

export {
	Badge,
	Button,
	Calendar,
	Command,
	Dialog,
	HoverCard,
	Input,
	InputOTP,
	Popover,
	Select,
	Separator,
	Switch,
	Textarea
};

// Domain Components
export { default as PostCard } from './components/post/card/PostCard.svelte';
export { default as PostList } from './components/post/PostList.svelte';
export { default as PostCardSkeleton } from './components/post/card/PostCardSkeleton.svelte';
export { default as Navbar } from './components/navbar/Navbar.svelte';
export { default as ProfileHeader } from './components/profile/ProfileHeader.svelte';
export { default as ProfileInfo } from './components/profile/ProfileInfo.svelte';
export { default as ProfilePostGrid } from './components/profile/ProfilePostGrid.svelte';
export { default as ImageCropModal } from './components/modal/ImageCropModal.svelte';

// Comment Components
export { default as CommentList } from './components/comment/CommentList.svelte';
export { default as CommentView } from './components/comment/CommentView.svelte';
export { default as CommentItem } from './components/comment/CommentItem.svelte';
export { default as CommentForm } from './components/comment/forms/CommentForm.svelte';
export { default as CommentContent } from './components/comment/ui/CommentContent.svelte';
export { default as CommentSkeleton } from './components/comment/CommentSkeleton.svelte';

// Report Components
export { default as ReportDialog } from './components/report/ReportDialog.svelte';

// Settings Components
export { default as AccountSettings } from './components/settings/forms/AccountSettings/AccountSettings.svelte';
export { default as DisplaySettings } from './components/settings/forms/DisplaySettings/DisplaySettings.svelte';
export { default as NotificationSettings } from './components/settings/forms/NotificationSettings.svelte';
export { default as PersonalInfoSettings } from './components/settings/forms/PersonalInfoSettings/PersonalInfoSettings.svelte';
export { default as PrivacySettings } from './components/settings/forms/PrivacySettings.svelte';
export { default as WritingSettings } from './components/settings/forms/WritingSettings.svelte';

// Write Components
export { default as WriteEditor } from './components/write/WriteEditor.svelte';
export { default as WritePreview } from './components/write/WritePreview.svelte';
export { default as WriteToolbar } from './components/write/WriteToolbar.svelte';
export { default as WriteHeader } from './components/write/WriteHeader.svelte';
export { default as WriteActions } from './components/write/WriteActions.svelte';
export * from './components/write/PublishDialog/index';

// Stores
export { authStore } from './stores/auth.svelte';
export { userStore } from './stores/user.svelte';
export { postsStore } from './stores/posts.svelte';
export * from './stores/settings.svelte';

// Hooks
export { useNavbarScroll } from './hooks/ui/useNavbarScroll.svelte';
export { useInfiniteScroll } from './hooks/ui/useInfiniteScroll.svelte';
export { useResizable } from './hooks/ui/useResizable.svelte';
export { useWriteEditor } from './hooks/write/useWriteEditor.svelte';
export { usePostsData } from './hooks/posts/usePostsData.svelte';
export { usePostsFilter } from './hooks/posts/usePostsFilter.svelte';
export { useFieldValidation } from './hooks/settings/useFieldValidation.svelte';
export { useComments } from './hooks/comment/useComments.svelte';
export { useReplies } from './hooks/comment/useReplies.svelte';

// API Layer
export * from './api/index';
export * from './api/auth/types';
export * from './api/user/types';
export * from './api/post/types';
export * from './api/follow/types';
export * from './api/like/types';
export * from './api/comment/types';
export * from './api/report/types';

// Utils
export * from './utils/index';
export * from './utils/imagecrop';

// Schemas
export { createPostSchema, type PostData } from './schemas/post';
export { createSignupSchema, type SignupInfo } from './schemas/signup';
export { createPersonalInfoSchema, type PersonalInfo } from './schemas/personal-info';

// OAuth
export * from './oauth/config';
