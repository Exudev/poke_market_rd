<script setup lang="ts">
import { ref, watch, onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import axios from 'axios'
import { Search, Bell, Heart, User, Inbox } from 'lucide-vue-next'

const isLoggedIn = ref(false)
const route = useRoute()

const notifications = ref<any[]>([])
const showNotifications = ref(false)
const currentUser = ref<any>(null)

const checkAuth = async () => {
  isLoggedIn.value = !!localStorage.getItem('token')
  if (isLoggedIn.value) {
      fetchNotifications()
      fetchUser()
  }
}

const fetchUser = async () => {
    try {
        const token = localStorage.getItem('token')
        const res = await axios.get('api/users/me', {
            headers: { Authorization: `Bearer ${token}` }
        })
        currentUser.value = res.data
    } catch (e) {
        console.error("Failed to load user profile", e)
    }
}

const fetchNotifications = async () => {
    try {
        const token = localStorage.getItem('token')
        const res = await axios.get('api/notifications', {
            headers: { Authorization: `Bearer ${token}` }
        })
        notifications.value = res.data
    } catch (e) {
        console.error("Failed to load notifications", e)
    }
}

const toggleNotifications = async () => {
    showNotifications.value = !showNotifications.value
    if (showNotifications.value && unreadCount.value > 0) {
        // Mark as read
        try {
            const token = localStorage.getItem('token')
            await axios.put('api/notifications', {}, {
                headers: { Authorization: `Bearer ${token}` }
            })
            notifications.value.forEach(n => n.is_read = true)
        } catch (e) {
            console.error("Failed to mark notifications read", e)
        }
    }
}


import { useRouter } from 'vue-router'

const unreadCount = computed(() => notifications.value.filter(n => !n.is_read).length)
const router = useRouter()
const searchQuery = ref('')

const handleSearch = () => {
    if (searchQuery.value.trim()) {
        router.push(`/?q=${encodeURIComponent(searchQuery.value.trim())}`)
    } else {
        router.push('/')
    }
}

onMounted(checkAuth)
watch(() => route.path, () => {
    checkAuth()
    showNotifications.value = false
})
</script>

<template>
  <nav class="bg-white border-b border-gray-200 sticky top-0 z-50">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex flex-col md:flex-row md:justify-between md:h-16 py-3 md:py-0 gap-3 md:gap-0">
        
        <!-- Top Row (Mobile) / Left Side (Desktop) -->
        <div class="flex items-center justify-between w-full md:w-auto">
          <div class="flex items-center">
            <router-link to="/" class="flex-shrink-0 flex items-center gap-2 cursor-pointer">
              <div class="w-8 h-8 rounded-full bg-red-500 border-2 border-black relative overflow-hidden flex flex-col">
                  <div class="h-1/2 w-full bg-red-500"></div>
                  <div class="h-1/2 w-full bg-white"></div>
                  <div class="absolute inset-0 m-auto w-3 h-3 bg-white border-2 border-black rounded-full"></div>
                  <div class="absolute inset-0 m-auto h-[2px] w-full bg-black"></div>
              </div>
              <span class="font-bold text-xl tracking-tight hidden sm:block">PokéMart</span>
            </router-link>
            
            <div class="ml-4 sm:ml-8 flex space-x-3 sm:space-x-8">
              <router-link to="/" class="border-red-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">Explore</router-link>
              <router-link v-if="isLoggedIn" to="/create" class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">Sell</router-link>
              <router-link to="/community" class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">Community</router-link>
              <router-link v-if="!isLoggedIn" to="/login" class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium">Login</router-link>
            </div>
          </div>

          <!-- Icons (Mobile visible, but compressed) -->
          
          <div class="flex items-center space-x-3 md:hidden">
            <router-link v-if="isLoggedIn" to="/offers" class="text-gray-400 hover:text-gray-500">
              <Inbox class="h-6 w-6" />
            </router-link>
            <button v-if="isLoggedIn" @click="toggleNotifications" class="text-gray-400 hover:text-gray-500 relative">
              <Bell class="h-6 w-6" />
              <span v-if="unreadCount > 0" class="absolute top-0 right-0 block h-2.5 w-2.5 rounded-full bg-red-500 ring-2 ring-white"></span>
            </button>
            <router-link v-if="isLoggedIn" to="/profile" class="flex text-sm border-2 border-transparent rounded-full focus:outline-none">
              <div class="h-8 w-8 rounded-full bg-gray-200 hover:bg-gray-300 flex items-center justify-center text-gray-600 overflow-hidden">
                  <img v-if="currentUser?.avatar_url" :src="currentUser.avatar_url" class="w-full h-full object-cover" />
                  <User v-else class="h-5 w-5"/>
              </div>
            </router-link>
          </div>

        </div>

        <!-- Search Bar (Row 2 on Mobile, Center on Desktop) -->
        <div class="flex-1 flex items-center justify-center w-full md:px-2 lg:ml-6 lg:justify-end order-3 md:order-2">
          <div class="max-w-lg w-full lg:max-w-xs relative">
            <label for="search" class="sr-only">Search</label>
            <div class="relative">
              <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                <Search class="h-5 w-5 text-gray-400" />
              </div>
              <input id="search" v-model="searchQuery" @keyup.enter="handleSearch" name="search" class="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-full leading-5 bg-gray-50 placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-red-500 focus:border-red-500 sm:text-sm transition duration-150 ease-in-out" placeholder="Search Charizard, Pikachu, Vintage..." type="search">
            </div>
          </div>
        </div>

        <!-- Desktop Right Icons -->
        <div class="hidden md:flex md:items-center space-x-4 order-2 md:order-3">
          <router-link v-if="isLoggedIn" to="/offers" class="p-1 rounded-full text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
            <span class="sr-only">Offers</span>
            <Inbox class="h-6 w-6" />
          </router-link>
          
          <div class="relative">
            <button @click="toggleNotifications" class="p-1 rounded-full text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 relative">
              <span class="sr-only">View notifications</span>
              <Bell class="h-6 w-6" />
              <span v-if="unreadCount > 0" class="absolute top-0 right-0 block h-2.5 w-2.5 rounded-full bg-red-500 ring-2 ring-white"></span>
            </button>
            
            <!-- Notifications Dropdown -->
            <div v-if="showNotifications" class="origin-top-right absolute right-0 mt-2 w-80 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none z-50">
              <div class="py-1">
                <div class="px-4 py-2 border-b border-gray-100 flex justify-between items-center">
                    <span class="font-bold text-gray-700">Notifications</span>
                    <span v-if="unreadCount > 0" class="text-xs bg-red-100 text-red-600 px-2 py-0.5 rounded-full">{{ unreadCount }} new</span>
                </div>
                <div class="max-h-96 overflow-y-auto">
                    <div v-if="notifications.length === 0" class="px-4 py-6 text-sm text-center text-gray-500">
                        No notifications yet.
                    </div>
                    <div v-for="notif in notifications" :key="notif.id" class="px-4 py-3 hover:bg-gray-50 border-b border-gray-50 last:border-0" :class="{'bg-blue-50': !notif.is_read}">
                        <p class="text-sm text-gray-800">{{ notif.message }}</p>
                        <p class="text-xs text-gray-400 mt-1">{{ new Date(notif.created_at).toLocaleDateString() }}</p>
                    </div>
                </div>
              </div>
            </div>
          </div>
          <router-link v-if="isLoggedIn" to="/favorites" class="p-1 rounded-full text-gray-400 hover:text-red-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 transition-colors">
             <Heart class="h-6 w-6" />
          </router-link>
          <div class="relative ml-3">
            <router-link v-if="isLoggedIn" to="/profile" class="flex text-sm border-2 border-transparent rounded-full focus:outline-none focus:border-gray-300 transition duration-150 ease-in-out">
              <div class="h-8 w-8 rounded-full bg-gray-200 hover:bg-gray-300 flex items-center justify-center text-gray-600 transition overflow-hidden">
                  <img v-if="currentUser?.avatar_url" :src="currentUser.avatar_url" class="w-full h-full object-cover" />
                  <User v-else class="h-5 w-5"/>
              </div>
            </router-link>
          </div>
        </div>

      </div>
    </div>
  </nav>
</template>
