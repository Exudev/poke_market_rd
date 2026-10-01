<script setup lang="ts">
import { ref, onMounted } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'

const posts = ref<any[]>([])
const loading = ref(true)
const router = useRouter()

onMounted(async () => {
    const token = localStorage.getItem('token')
    if (!token) {
        router.push('/login')
        return
    }

    try {
        const res = await axios.get('api/users/me/favorites', {
            headers: { Authorization: `Bearer ${token}` }
        })
        posts.value = res.data
    } catch (e) {
        console.error(e)
    } finally {
        loading.value = false
    }
})
</script>

<template>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <h1 class="text-3xl font-bold text-gray-900 mb-8">My Favorites</h1>

    <div v-if="loading" class="text-center py-12 text-gray-500">Loading favorites...</div>
    
    <div v-else-if="posts.length === 0" class="text-center py-24 bg-white rounded-2xl shadow-sm border border-gray-100">
        <h3 class="text-xl font-bold text-gray-900 mb-2">No favorites yet</h3>
        <p class="text-gray-500 mb-6">Start exploring to save cards you want to track.</p>
        <router-link to="/" class="inline-block bg-red-600 hover:bg-red-700 text-white font-bold py-3 px-6 rounded-xl transition-colors">
            Explore Market
        </router-link>
    </div>

    <div v-else class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
        <router-link :to="`/posts/${post.id}`" v-for="post in posts" :key="post.id" class="bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden hover:shadow-md transition block group">
          <div class="aspect-[3/4] bg-gray-100 relative flex items-center justify-center overflow-hidden">
            <img v-if="post.image_url" :src="post.image_url" class="absolute inset-0 w-full h-full object-cover transition-transform group-hover:scale-105" />
            <div v-else class="w-full h-full p-4">
              <div class="w-full h-full bg-gradient-to-br from-yellow-200 to-yellow-500 rounded-lg shadow-inner border-[10px] border-yellow-300 flex items-center justify-center text-yellow-800 font-bold opacity-50 text-center px-2">
                {{ post.pokemon_name }}
              </div>
            </div>
            
            <!-- Red heart indicator -->
            <div class="absolute top-3 right-3 bg-white p-2 rounded-full shadow-sm">
                <svg class="w-5 h-5 text-red-500 fill-current" viewBox="0 0 24 24"><path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/></svg>
            </div>
          </div>
          <div class="p-4">
            <h3 class="font-bold text-gray-800 truncate mb-1">{{ post.pokemon_name }}</h3>
            <p class="text-xs text-gray-500 truncate mb-3">{{ post.set_name }} &bull; {{ post.series }}</p>
            <div class="flex justify-between items-end">
              <div>
                <span class="text-[10px] uppercase font-bold text-gray-400 block mb-0.5">Condition</span>
                <p class="text-xs font-semibold text-green-600 bg-green-50 inline-block px-2 py-0.5 rounded">{{ post.condition }}</p>
              </div>
              <div class="text-lg font-black text-gray-900">
                {{ post.currency === 'DOP' ? 'RD$' : '$' }}{{ (post.price_cents / 100).toFixed(2) }}
              </div>
            </div>
          </div>
        </router-link>
    </div>
  </div>
</template>
