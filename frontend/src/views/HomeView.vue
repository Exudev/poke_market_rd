<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import axios from 'axios'
import { formatFilters } from '../utils/pokemonSets'

const posts = ref<any[]>([])
const currentPage = ref(0)
const limit = 12
const hasMore = ref(true)
const sortBy = ref('newest')

const filters = ref(formatFilters())
const route = useRoute()

const toggleSelectAll = (filterGroup: any) => {
  const allChecked = filterGroup.options.every((opt: any) => opt.checked)
  filterGroup.options.forEach((opt: any) => opt.checked = !allChecked)
}

const fetchPosts = async (reset = false) => {
    if (reset) {
        currentPage.value = 0
        posts.value = []
        hasMore.value = true
    }
    
    if (!hasMore.value) return;

    try {
        let selectedSets: string[] = []
        filters.value.forEach(group => {
            group.options.forEach(opt => {
                if (opt.checked) {
                    selectedSets.push(opt.label)
                }
            })
        })
        
        let url = `api/posts?limit=${limit}&offset=${currentPage.value * limit}`
        if (selectedSets.length > 0) {
            url += `&sets=${encodeURIComponent(selectedSets.join(','))}`
        }
        
        if (route.query.q) {
            url += `&search=${encodeURIComponent(route.query.q as string)}`
        }

        url += `&sort=${sortBy.value}`
        
        const response = await axios.get(url)
        const newPosts = response.data
        if (newPosts.length < limit) {
            hasMore.value = false
        }
        posts.value = [...posts.value, ...newPosts]
        currentPage.value++
    } catch (error) {
        console.error("Failed to fetch posts", error)
    }
}

onMounted(() => {
    fetchPosts(true)
})

watch(filters, () => fetchPosts(true), { deep: true })
watch(sortBy, () => fetchPosts(true))
watch(() => route.query.q, () => fetchPosts(true))
</script>

<template>
  <div class="flex flex-col md:flex-row gap-6">
    <!-- Sidebar Filters -->
    <aside class="w-full md:w-64 lg:w-72 flex-shrink-0">
      <div class="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden sticky top-24">
        <div class="p-5 border-b border-gray-100 flex justify-between items-center">
          <h2 class="text-lg font-black text-gray-900 tracking-tight">Filters</h2>
          <button @click="filters.forEach(g => g.options.forEach(o => o.checked = false))" class="text-xs font-semibold text-gray-500 hover:text-red-600 transition-colors">
            Clear All
          </button>
        </div>
        
        <div class="max-h-[calc(100vh-200px)] overflow-y-auto p-2 scrollbar-thin scrollbar-thumb-gray-200">
            <div v-for="(group, idx) in filters" :key="idx" class="mb-1 border-b border-gray-50 last:border-0 pb-1">
                <button @click="group.expanded = !group.expanded" class="w-full flex justify-between items-center p-3 rounded-xl hover:bg-gray-50 transition-colors">
                    <span class="font-bold text-gray-800 text-sm">{{ group.name }}</span>
                    <svg class="w-4 h-4 text-gray-400 transition-transform duration-200" :class="{ 'rotate-180': group.expanded }" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                    </svg>
                </button>
                
                <div v-show="group.expanded" class="px-3 pb-3 pt-1 space-y-3">
                    <button @click="toggleSelectAll(group)" class="text-[11px] font-bold text-red-600 hover:text-red-700 uppercase tracking-wider">
                        {{ group.options.every(o => o.checked) ? 'Deselect All' : 'Select All' }}
                    </button>
                    <label v-for="opt in group.options" :key="opt.id" class="flex items-start gap-3 cursor-pointer group">
                        <div class="relative flex items-center justify-center mt-0.5 flex-shrink-0">
                            <input type="checkbox" v-model="opt.checked" class="peer appearance-none w-5 h-5 border-2 border-gray-200 rounded md:rounded-md checked:bg-red-500 checked:border-red-500 focus:ring-2 focus:ring-red-500 focus:ring-offset-1 transition-all cursor-pointer" />
                            <svg class="absolute w-3 h-3 text-white opacity-0 peer-checked:opacity-100 pointer-events-none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
                        </div>
                        <span class="text-sm text-gray-600 group-hover:text-gray-900 transition-colors leading-snug">{{ opt.label }}</span>
                    </label>
                </div>
            </div>
        </div>
      </div>
    </aside>

    <!-- Main Feed -->
    <div class="flex-1">
      
    <div class="col-span-full mb-6 bg-gradient-to-r from-red-500 to-red-600 rounded-2xl shadow-lg p-6 md:p-10 text-white flex flex-col md:flex-row justify-between items-center overflow-hidden relative">
      <div class="relative z-10 max-w-xl">
        <h1 class="text-3xl md:text-4xl font-black mb-2 tracking-tight">The #1 TCG Marketplace in DR 🇩🇴</h1>
        <p class="text-red-100 text-lg md:text-xl font-medium mb-6">Buy, sell, and trade your Pokémon cards safely with collectors across the Dominican Republic.</p>
        <router-link to="/create" class="inline-block bg-white text-red-600 font-bold py-3 px-8 rounded-full shadow-md hover:bg-gray-50 transition transform hover:-translate-y-0.5">
          Start Selling Now
        </router-link>
      </div>
      <div class="absolute right-0 opacity-10 md:opacity-20 transform translate-x-1/4 scale-150 pointer-events-none">
        <svg class="w-64 h-64" viewBox="0 0 100 100" fill="currentColor">
          <path d="M50 0C22.4 0 0 22.4 0 50s22.4 50 50 50 50-22.4 50-50S77.6 0 50 0zm0 90C27.9 90 10 72.1 10 50c0-9.8 3.5-18.8 9.3-25.7 3.5 6.2 10.3 10.4 18.2 10.4h5c4.4 0 8 3.6 8 8v5c0 4.4 3.6 8 8 8h2c4.4 0 8-3.6 8-8v-2c0-8.8 7.2-16 16-16 2.3 0 4.4.5 6.4 1.4C86.5 31.2 90 40.2 90 50c0 22.1-17.9 40-40 40z" />
        </svg>
      </div>
    </div>

      <div class="flex justify-between items-center mb-6">
        <h1 class="text-2xl font-bold text-gray-800">Featured Listings</h1>
        <div class="flex gap-2">
          <select v-model="sortBy" class="border border-gray-300 rounded-md py-1 px-2 text-sm text-gray-600 bg-white">
            <option value="newest">Sort: Newest</option>
            <option value="price_asc">Price: Low to High</option>
            <option value="price_desc">Price: High to Low</option>
          </select>
        </div>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        <!-- Actual Cards -->
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
            <button class="w-full mt-4 border border-gray-300 text-gray-700 font-semibold py-2 rounded hover:bg-gray-50 transition text-sm">
              View Details
            </button>
          </div>
        </router-link>
        
        <!-- Empty State -->
        <div v-if="posts.length === 0" class="col-span-full text-center py-12 text-gray-500">
            No cards found. Be the first to sell one!
        </div>
      </div>
      
      <!-- Load More Button -->
      <div v-if="hasMore" class="mt-8 text-center">
        <button @click="fetchPosts(false)" class="bg-white border-2 border-red-100 hover:border-red-200 text-red-600 font-bold py-3 px-8 rounded-full shadow-sm hover:shadow transition-all uppercase tracking-widest text-sm">
            Load More Cards
        </button>
      </div>
      <div v-if="!hasMore && posts.length > 0" class="mt-12 text-center text-gray-400 font-medium pb-8">
        You've reached the end of the market!
      </div>
      
    </div>
  </div>
</template>
