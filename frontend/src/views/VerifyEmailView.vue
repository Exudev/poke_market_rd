<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import axios from 'axios'
import { CheckCircle, XCircle } from 'lucide-vue-next'

const route = useRoute()
const status = ref<'loading' | 'success' | 'error'>('loading')
const message = ref('Verifying your email...')

onMounted(async () => {
    const token = route.query.token
    if (!token) {
        status.value = 'error'
        message.value = 'No token provided.'
        return
    }

    try {
        const res = await axios.get(`api/auth/verify-email?token=${token}`)
        status.value = 'success'
        message.value = res.data
    } catch (e: any) {
        status.value = 'error'
        message.value = e.response?.data || 'Verification failed.'
    }
})
</script>

<template>
  <div class="max-w-md mx-auto py-24 px-6 text-center">
    <div v-if="status === 'loading'" class="animate-pulse">
        <div class="w-16 h-16 bg-gray-200 rounded-full mx-auto mb-4"></div>
        <h2 class="text-xl font-bold text-gray-700">{{ message }}</h2>
    </div>

    <div v-else-if="status === 'success'" class="space-y-4">
        <CheckCircle class="w-20 h-20 text-green-500 mx-auto" />
        <h2 class="text-2xl font-black text-gray-900">Verified!</h2>
        <p class="text-gray-600">{{ message }}</p>
        <router-link to="/" class="mt-6 inline-block bg-red-500 text-white font-bold py-2 px-6 rounded-full hover:bg-red-600 transition">
            Go to PokéMart
        </router-link>
    </div>

    <div v-else class="space-y-4">
        <XCircle class="w-20 h-20 text-red-500 mx-auto" />
        <h2 class="text-2xl font-black text-gray-900">Verification Failed</h2>
        <p class="text-gray-600">{{ message }}</p>
        <router-link to="/" class="mt-6 inline-block bg-gray-200 text-gray-800 font-bold py-2 px-6 rounded-full hover:bg-gray-300 transition">
            Back to Home
        </router-link>
    </div>
  </div>
</template>
