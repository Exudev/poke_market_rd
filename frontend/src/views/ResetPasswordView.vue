<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import axios from 'axios'
import { Lock, CheckCircle } from 'lucide-vue-next'

const route = useRoute()
const router = useRouter()
const token = computed(() => route.query.token as string)

const newPassword = ref('')
const confirmPassword = ref('')
const loading = ref(false)
const success = ref(false)
const errorMessage = ref('')

const submit = async () => {
    if (newPassword.value !== confirmPassword.value) {
        errorMessage.value = 'Passwords do not match'
        return
    }

    loading.value = true
    errorMessage.value = ''
    try {
        await axios.post('/api/auth/reset-password', { 
            token: token.value, 
            new_password: newPassword.value 
        })
        success.value = true
        setTimeout(() => router.push('/login'), 3000)
    } catch (e: any) {
        errorMessage.value = e.response?.data?.error || 'Failed to reset password'
    } finally {
        loading.value = false
    }
}
</script>

<template>
  <div class="max-w-md mx-auto mt-20 p-6 bg-white rounded-2xl shadow-xl">
    <div class="text-center mb-8">
      <div class="bg-red-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
        <Lock class="w-8 h-8 text-red-500" />
      </div>
      <h2 class="text-3xl font-black text-gray-900">Reset Password</h2>
      <p class="text-gray-500 mt-2">Create a new secure password for your account.</p>
    </div>

    <div v-if="success" class="text-center space-y-4">
      <CheckCircle class="w-16 h-16 text-green-500 mx-auto" />
      <h3 class="text-xl font-bold text-gray-900">Password Reset Successfully!</h3>
      <p class="text-gray-600">Redirecting you to login...</p>
    </div>

    <form v-else @submit.prevent="submit" class="space-y-5">
      <div>
        <label class="block text-sm font-bold text-gray-700 mb-1">New Password</label>
        <input v-model="newPassword" type="password" required minlength="6"
          class="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-red-500 focus:border-transparent transition" />
      </div>

      <div>
        <label class="block text-sm font-bold text-gray-700 mb-1">Confirm New Password</label>
        <input v-model="confirmPassword" type="password" required minlength="6"
          class="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-red-500 focus:border-transparent transition" />
      </div>

      <div v-if="errorMessage" class="p-3 bg-red-50 text-red-600 text-sm rounded-lg border border-red-100">
        {{ errorMessage }}
      </div>

      <button type="submit" :disabled="loading"
        class="w-full bg-red-500 text-white font-bold py-3 px-4 rounded-xl hover:bg-red-600 transition flex justify-center items-center gap-2 shadow-lg shadow-red-200">
        <span v-if="loading" class="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></span>
        <span>Reset Password</span>
      </button>
    </form>
  </div>
</template>
