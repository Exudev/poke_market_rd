<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router'
import { ref, watch, onMounted } from 'vue'
import NavBar from './components/NavBar.vue'
import ToastContainer from './components/ToastContainer.vue'
import { Home, Search, PlusSquare, Inbox, User } from 'lucide-vue-next'

const isLoggedIn = ref(false)
const route = useRoute()

const checkAuth = () => {
  isLoggedIn.value = !!localStorage.getItem('token')
}

onMounted(checkAuth)
watch(() => route.path, checkAuth)
</script>

<template>
  <div class="min-h-screen flex flex-col font-sans pb-16 md:pb-0 bg-gray-50">
    <NavBar />
    
    <main class="flex-1 max-w-7xl w-full mx-auto p-4 sm:p-6 lg:p-8">
      <RouterView />
    </main>
    
    <footer class="text-center py-6 text-sm text-gray-500 flex flex-wrap justify-center gap-4 mb-4 md:mb-0">
      <span>&copy; 2026 PokéMart</span>
      <router-link to="/terms" class="hover:text-red-500">Terms of Service</router-link>
      <router-link to="/privacy" class="hover:text-red-500">Privacy Policy</router-link>
    </footer>

    <ToastContainer />

    <!-- Mobile Bottom Navigation -->
    <div class="md:hidden fixed bottom-0 left-0 w-full bg-white border-t border-gray-200 flex justify-around items-center py-3 px-2 z-50 safe-area-bottom shadow-[0_-2px_10px_rgba(0,0,0,0.05)]">
      <router-link to="/" class="flex flex-col items-center p-1" :class="route.path === '/' ? 'text-red-500' : 'text-gray-400 hover:text-gray-600'">
        <Home class="w-6 h-6" />
        <span class="text-[10px] mt-1 font-medium">Home</span>
      </router-link>
      
      <router-link to="/community" class="flex flex-col items-center p-1" :class="route.path === '/community' ? 'text-red-500' : 'text-gray-400 hover:text-gray-600'">
        <Search class="w-6 h-6" />
        <span class="text-[10px] mt-1 font-medium">Feed</span>
      </router-link>

      <router-link v-if="isLoggedIn" to="/create" class="flex flex-col items-center p-1 -mt-5 relative">
        <div class="bg-red-500 text-white rounded-full p-3 shadow-lg border-4 border-gray-50">
          <PlusSquare class="w-6 h-6" />
        </div>
        <span class="text-[10px] mt-1 font-medium text-gray-600">Sell</span>
      </router-link>

      <router-link v-if="isLoggedIn" to="/offers" class="flex flex-col items-center p-1" :class="route.path === '/offers' ? 'text-red-500' : 'text-gray-400 hover:text-gray-600'">
        <Inbox class="w-6 h-6" />
        <span class="text-[10px] mt-1 font-medium">Inbox</span>
      </router-link>
      <router-link v-else to="/login" class="flex flex-col items-center p-1" :class="route.path === '/login' ? 'text-red-500' : 'text-gray-400 hover:text-gray-600'">
        <User class="w-6 h-6" />
        <span class="text-[10px] mt-1 font-medium">Login</span>
      </router-link>

      <router-link v-if="isLoggedIn" to="/profile" class="flex flex-col items-center p-1" :class="route.path === '/profile' ? 'text-red-500' : 'text-gray-400 hover:text-gray-600'">
        <User class="w-6 h-6" />
        <span class="text-[10px] mt-1 font-medium">Profile</span>
      </router-link>
    </div>
  </div>
</template>

<style>
.safe-area-bottom {
  padding-bottom: env(safe-area-inset-bottom);
}
</style>
