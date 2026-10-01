<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'

const user = ref<any>(null)
const posts = ref<any[]>([])
const loading = ref(true)
const editing = ref(false)
const editForm = ref({ username: '', bio: '', avatar_url: '', phone: '' })
const router = useRouter()

const fetchProfile = async (token: string) => {
    try {
        const res = await axios.get('api/users/me', {
            headers: { Authorization: `Bearer ${token}` }
        })
        user.value = res.data
        editForm.value = {
            username: res.data.username || '',
            bio: res.data.bio || '',
            avatar_url: res.data.avatar_url || '',
            phone: res.data.phone || ''
        }
    } catch (error) {
        console.error("Failed to load user profile", error)
        if (axios.isAxiosError(error) && error.response?.status === 401) {
            localStorage.removeItem('token')
            router.push('/login')
        }
    }
}

const currentPage = ref(0)
const limit = 12
const hasMore = ref(true)

const fetchPosts = async (reset = false) => {
    const token = localStorage.getItem('token')
    if (!token) return

    if (reset) {
        currentPage.value = 0
        posts.value = []
        hasMore.value = true
    }
    
    if (!hasMore.value) return;

    try {
        const res = await axios.get(`api/users/me/posts?limit=${limit}&offset=${currentPage.value * limit}`, {
            headers: { Authorization: `Bearer ${token}` }
        })
        const newPosts = res.data
        if (newPosts.length < limit) {
            hasMore.value = false
        }
        posts.value = [...posts.value, ...newPosts]
        currentPage.value++
    } catch (error) {
        console.error("Failed to load user posts", error)
    }
}

onMounted(async () => {
    const token = localStorage.getItem('token')
    if (!token) {
        router.push('/login')
        return
    }
    await fetchProfile(token)
    await fetchPosts(true)
    loading.value = false
})

const saveProfile = async () => {
    try {
        const token = localStorage.getItem('token')
        const response = await axios.put('api/users/me', editForm.value, {
            headers: { Authorization: `Bearer ${token}` }
        })
        user.value = response.data
        editing.value = false
    } catch (error) {
        console.error("Failed to update profile", error)
        alert("Failed to update profile")
    }
}

const logout = () => {
    localStorage.removeItem('token')
    router.push('/login')
}
</script>

<template>
  <div class="max-w-4xl mx-auto py-8 px-4 sm:px-6 lg:px-8">
    <div v-if="loading" class="text-center py-12 text-gray-500">Loading profile...</div>
    
    <div v-else>
        <!-- Profile Header -->
        <div class="bg-white shadow rounded-lg p-6 mb-8 flex flex-col md:flex-row items-center md:items-start gap-6 relative">
            <button @click="logout" class="absolute top-4 right-4 text-sm text-red-600 hover:text-red-800 font-medium">Sign Out</button>

            <img 
                :src="user.avatar_url || 'https://ui-avatars.com/api/?name=' + user.username + '&background=random'" 
                class="w-32 h-32 rounded-full border-4 border-gray-100 shadow-sm object-cover" 
                alt="Profile Avatar"
            />
            
            <div v-if="!editing" class="flex-1 text-center md:text-left mt-4 md:mt-0">
                <h1 class="text-3xl font-bold text-gray-900">{{ user.username }}</h1>
                <p class="text-gray-500 text-sm mt-1">{{ user.email || 'No email provided' }}</p>
                <p class="text-gray-700 mt-4">{{ user.bio || 'No bio yet. Edit your profile to tell us about your collection!' }}</p>
                
                <button @click="editing = true" class="mt-6 inline-flex items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
                    Edit Profile
                </button>
            </div>

            <div v-else class="flex-1 w-full mt-4 md:mt-0">
                <form @submit.prevent="saveProfile" class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700">Username</label>
                        <input v-model="editForm.username" type="text" required class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-red-500 focus:border-red-500 sm:text-sm">
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">Bio</label>
                        <textarea v-model="editForm.bio" rows="3" class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-red-500 focus:border-red-500 sm:text-sm"></textarea>
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">Avatar URL</label>
                        <input v-model="editForm.avatar_url" type="url" class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-red-500 focus:border-red-500 sm:text-sm">
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-gray-700">Phone (WhatsApp)</label>
                        <input v-model="editForm.phone" type="text" placeholder="+1234567890" class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-red-500 focus:border-red-500 sm:text-sm">
                        <p class="text-xs text-gray-500 mt-1">Include country code for WhatsApp (e.g. +1...)</p>
                    </div>
                    <div class="flex space-x-3">
                        <button type="submit" class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
                            Save Changes
                        </button>
                        <button type="button" @click="editing = false" class="inline-flex justify-center py-2 px-4 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
                            Cancel
                        </button>
                    </div>
                </form>
            </div>
        </div>

        <!-- User's Posts -->
        <div>
            <h2 class="text-xl font-bold text-gray-900 mb-6">My Listings</h2>
            <div v-if="posts.length === 0" class="text-gray-500 bg-white p-8 rounded-lg shadow text-center">
                You haven't listed any cards yet.
                <router-link to="/create" class="block mt-4 text-red-600 hover:underline">Start selling today!</router-link>
            </div>
            <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                <router-link :to="`/posts/${post.id}`" v-for="post in posts" :key="post.id" class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden hover:shadow-md transition block">
                    <div class="aspect-[3/4] bg-gray-200 relative flex items-center justify-center overflow-hidden">
                        <img v-if="post.image_url" :src="post.image_url" class="absolute inset-0 w-full h-full object-cover" />
                        <div v-else class="w-full h-full p-4">
                            <div class="w-full h-full bg-gradient-to-br from-yellow-200 to-yellow-500 rounded-lg shadow-inner border-[10px] border-yellow-300 flex items-center justify-center text-yellow-800 font-bold opacity-50 text-center px-2">
                                {{ post.pokemon_name }}
                            </div>
                        </div>
                    </div>
                    <div class="p-4">
                        <h3 class="font-bold text-gray-800 truncate">{{ post.pokemon_name }}</h3>
                        <p class="text-xs text-gray-500 truncate mb-2">{{ post.set_name }} ({{ post.series }})</p>
                        <div class="flex justify-between items-end">
                        <div>
                            <span class="text-xs text-gray-400">Condition</span>
                            <p class="text-sm font-semibold text-green-600">{{ post.condition }}</p>
                        </div>
                        <div class="text-lg font-bold text-gray-900">
                            {{ post.currency === 'DOP' ? 'RD$' : '$' }}{{ (post.price_cents / 100).toFixed(2) }}
                        </div>
                        </div>
                    </div>
                </router-link>
            </div>
        </div>
    </div>
  </div>
</template>
