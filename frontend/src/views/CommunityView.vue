<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'
import { Heart, MessageSquare, Camera, X } from 'lucide-vue-next'

const router = useRouter()
const posts = ref<any[]>([])
const loading = ref(true)
const errorMessage = ref('')

const newPostContent = ref('')
const newPostCategory = ref('General')
const imageUrl = ref('')
const uploadingImage = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)

const categories = ['All', 'General', 'Pulls', 'Q&A', 'Decks']
const activeCategory = ref('All')

const expandedComments = ref<Record<string, boolean>>({})
const postComments = ref<Record<string, any[]>>({})
const newCommentContent = ref<Record<string, string>>({})

const fetchPosts = async () => {
    try {
        loading.value = true
        const token = localStorage.getItem('token')
        const headers = token ? { Authorization: `Bearer ${token}` } : {}
        
        const res = await axios.get('api/community', {
            params: { category: activeCategory.value },
            headers
        })
        posts.value = res.data
    } catch (error: any) {
        errorMessage.value = error.response?.data || "Failed to load community posts."
    } finally {
        loading.value = false
    }
}

watch(activeCategory, () => {
    fetchPosts()
})

const handleFileUpload = async (event: any) => {
    const file = event.target.files[0]
    if (!file) return

    uploadingImage.value = true
    const formData = new FormData()
    formData.append('file', file)

    try {
        const token = localStorage.getItem('token')
        const response = await axios.post('api/upload', formData, {
            headers: {
                'Authorization': `Bearer ${token}`,
                'Content-Type': 'multipart/form-data'
            }
        })
        imageUrl.value = response.data.url
    } catch (error) {
        console.error("Failed to upload image", error)
        alert("Failed to upload image.")
    } finally {
        uploadingImage.value = false
    }
}

const clearImage = () => {
    imageUrl.value = ''
    if (fileInput.value) fileInput.value.value = ''
}

const submitPost = async () => {
    if (!newPostContent.value.trim() && !imageUrl.value) return
    
    try {
        const token = localStorage.getItem('token')
        if (!token) {
            router.push('/login')
            return
        }
        
        await axios.post('api/community', {
            content: newPostContent.value,
            image_url: imageUrl.value || null,
            category: newPostCategory.value
        }, {
            headers: { Authorization: `Bearer ${token}` }
        })
        
        newPostContent.value = ''
        imageUrl.value = ''
        newPostCategory.value = 'General'
        await fetchPosts()
    } catch (e: any) {
        console.error(e)
        alert(e.response?.data || "Failed to create post.")
    }
}

const toggleLike = async (post: any) => {
    try {
        const token = localStorage.getItem('token')
        if (!token) {
            router.push('/login')
            return
        }

        // Optimistic update
        const wasLiked = post.user_liked
        post.user_liked = !wasLiked
        post.likes_count = parseInt(post.likes_count) + (wasLiked ? -1 : 1)

        await axios.post(`api/community/${post.id}/like`, {}, {
            headers: { Authorization: `Bearer ${token}` }
        })
    } catch (e) {
        // Revert on failure
        post.user_liked = !post.user_liked
        post.likes_count = parseInt(post.likes_count) + (post.user_liked ? 1 : -1)
        console.error("Failed to toggle like", e)
    }
}

const fetchComments = async (postId: string) => {
    try {
        const res = await axios.get(`api/community/${postId}/comments`)
        postComments.value[postId] = res.data
    } catch (e) {
        console.error("Failed to load comments", e)
    }
}

const toggleComments = (postId: string) => {
    expandedComments.value[postId] = !expandedComments.value[postId]
    if (expandedComments.value[postId] && !postComments.value[postId]) {
        fetchComments(postId)
    }
}

const submitComment = async (postId: string) => {
    const content = newCommentContent.value[postId]
    if (!content || !content.trim()) return

    try {
        const token = localStorage.getItem('token')
        if (!token) {
            router.push('/login')
            return
        }
        
        const res = await axios.post(`api/community/${postId}/comments`, {
            content: content.trim()
        }, {
            headers: { Authorization: `Bearer ${token}` }
        })
        
        if (!postComments.value[postId]) {
            postComments.value[postId] = []
        }
        postComments.value[postId].push(res.data)
        newCommentContent.value[postId] = ''
        
        // Update post comment count
        const post = posts.value.find(p => p.id === postId)
        if (post) post.comments_count = parseInt(post.comments_count) + 1
        
    } catch (e) {
        console.error("Failed to post comment", e)
        alert("Failed to post comment.")
    }
}

onMounted(() => {
    fetchPosts()
})
</script>

<template>
  <div class="max-w-3xl mx-auto py-8 px-4">
    <div class="flex items-center justify-between mb-8">
        <h1 class="text-3xl font-black text-gray-900">Community Hub</h1>
    </div>
    
    <!-- Filter Chips -->
    <div class="flex gap-2 overflow-x-auto pb-4 mb-6 scrollbar-hide">
        <button 
            v-for="cat in categories" 
            :key="cat"
            @click="activeCategory = cat"
            class="px-4 py-2 rounded-full font-semibold text-sm whitespace-nowrap transition-colors"
            :class="activeCategory === cat ? 'bg-red-500 text-white' : 'bg-white text-gray-600 border border-gray-200 hover:bg-gray-50'"
        >
            {{ cat }}
        </button>
    </div>
    
    <!-- Create Post box -->
    <div class="bg-white p-6 rounded-xl border border-gray-200 shadow-sm mb-8">
        <div class="flex gap-3 mb-3">
            <select v-model="newPostCategory" class="text-sm border-gray-300 rounded-lg shadow-sm focus:border-red-500 focus:ring-red-500 bg-gray-50 text-gray-700">
                <option v-for="cat in categories.filter(c => c !== 'All')" :key="cat" :value="cat">{{ cat }}</option>
            </select>
        </div>
        
        <textarea v-model="newPostContent" rows="3" placeholder="Share your pulls, ask for deck advice, or just say hi..." class="w-full border-gray-300 rounded-lg shadow-sm focus:border-red-500 focus:ring-red-500 resize-none p-3 text-gray-700"></textarea>
        
        <!-- Image Preview -->
        <div v-if="imageUrl" class="relative mt-3 inline-block">
            <img :src="`${imageUrl}`" class="h-32 rounded-lg object-cover border border-gray-200" />
            <button @click="clearImage" class="absolute -top-2 -right-2 bg-white rounded-full p-1 shadow-md border border-gray-200 hover:bg-gray-50">
                <X class="w-4 h-4 text-gray-500" />
            </button>
        </div>
        
        <div class="mt-4 flex items-center justify-between">
            <div>
                <input type="file" ref="fileInput" @change="handleFileUpload" accept="image/*" class="hidden" />
                <button @click="() => fileInput?.click()" :disabled="uploadingImage" class="p-2 text-gray-500 hover:text-red-500 hover:bg-red-50 rounded-full transition" title="Attach Image">
                    <Camera class="w-6 h-6" :class="{'animate-pulse': uploadingImage}" />
                </button>
            </div>
            <button @click="submitPost" :disabled="(!newPostContent.trim() && !imageUrl) || uploadingImage" class="bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-6 rounded-lg transition disabled:opacity-50">
                Post
            </button>
        </div>
    </div>

    <!-- Feed -->
    <div v-if="loading" class="text-center py-12 text-gray-500">Loading posts...</div>
    <div v-else-if="errorMessage" class="bg-red-50 text-red-700 p-4 rounded-xl border border-red-200">{{ errorMessage }}</div>
    <div v-else-if="posts.length === 0" class="text-center py-12 text-gray-500 bg-white rounded-xl border border-gray-200 border-dashed">
        <p class="font-bold text-gray-600 mb-1">No posts found</p>
        <p class="text-sm">Be the first to start the conversation in {{ activeCategory }}!</p>
    </div>
    
    <div v-else class="space-y-6">
        <div v-for="post in posts" :key="post.id" class="bg-white p-6 rounded-xl border border-gray-200 shadow-sm">
            <div class="flex items-center justify-between mb-4">
                <div class="flex items-center gap-3">
                    <img :src="post.avatar_url || 'https://ui-avatars.com/api/?name=' + post.username + '&background=random'" class="w-10 h-10 rounded-full" />
                    <div>
                        <p class="font-bold text-gray-900">{{ post.username }}</p>
                        <p class="text-xs text-gray-400">{{ new Date(post.created_at).toLocaleString() }}</p>
                    </div>
                </div>
                <span class="text-xs font-semibold px-2 py-1 bg-gray-100 text-gray-600 rounded-full">{{ post.category }}</span>
            </div>
            
            <p class="text-gray-800 whitespace-pre-wrap mb-4">{{ post.content }}</p>
            
            <div v-if="post.image_url" class="mb-4">
                <img :src="`${post.image_url}`" class="rounded-xl max-h-96 w-full object-cover border border-gray-100" />
            </div>
            
            <!-- Actions -->
            <div class="flex items-center gap-6 pt-4 border-t border-gray-100">
                <button @click="toggleLike(post)" class="flex items-center gap-2 group transition-colors" :class="post.user_liked ? 'text-red-500' : 'text-gray-500 hover:text-red-500'">
                    <Heart class="w-5 h-5 transition-transform group-hover:scale-110" :class="{'fill-current': post.user_liked}" />
                    <span class="font-semibold text-sm">{{ post.likes_count || 0 }}</span>
                </button>
                <button @click="toggleComments(post.id)" class="flex items-center gap-2 text-gray-500 hover:text-blue-500 group transition-colors">
                    <MessageSquare class="w-5 h-5 transition-transform group-hover:scale-110" :class="{'fill-current text-blue-100': expandedComments[post.id]}" />
                    <span class="font-semibold text-sm">{{ post.comments_count || 0 }}</span>
                </button>
            </div>
            
            <!-- Comments Section -->
            <div v-if="expandedComments[post.id]" class="mt-4 pt-4 border-t border-gray-100">
                <!-- Existing Comments -->
                <div class="space-y-4 mb-4">
                    <div v-if="!postComments[post.id]" class="text-xs text-gray-400 text-center py-2">Loading comments...</div>
                    <div v-else-if="postComments[post.id].length === 0" class="text-xs text-gray-400 text-center py-2">No comments yet.</div>
                    
                    <div v-for="comment in postComments[post.id]" :key="comment.id" class="flex gap-3">
                        <img :src="comment.avatar_url || 'https://ui-avatars.com/api/?name=' + comment.username + '&background=random'" class="w-8 h-8 rounded-full flex-shrink-0" />
                        <div class="bg-gray-50 p-3 rounded-2xl rounded-tl-none w-full border border-gray-100">
                            <div class="flex justify-between items-baseline mb-1">
                                <p class="font-bold text-sm text-gray-900">{{ comment.username }}</p>
                                <p class="text-[10px] text-gray-400">{{ new Date(comment.created_at).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'}) }}</p>
                            </div>
                            <p class="text-sm text-gray-700 whitespace-pre-wrap">{{ comment.content }}</p>
                        </div>
                    </div>
                </div>
                
                <!-- New Comment Input -->
                <div class="flex gap-2 items-center">
                    <input 
                        v-model="newCommentContent[post.id]" 
                        @keyup.enter="submitComment(post.id)"
                        type="text" 
                        placeholder="Write a comment..." 
                        class="flex-1 border border-gray-200 rounded-full px-4 py-2 text-sm focus:border-blue-500 focus:ring-1 focus:ring-blue-500 bg-gray-50"
                    />
                    <button @click="submitComment(post.id)" :disabled="!newCommentContent[post.id]?.trim()" class="p-2 bg-blue-500 text-white rounded-full hover:bg-blue-600 transition disabled:opacity-50">
                        <MessageSquare class="w-4 h-4" />
                    </button>
                </div>
            </div>
        </div>
    </div>
  </div>
</template>
