<script setup lang="ts">
import { ref } from 'vue'
import axios from 'axios'
import { Mail, ArrowLeft } from 'lucide-vue-next'

const email = ref('')
const loading = ref(false)
const success = ref(false)
const errorMessage = ref('')

const submit = async () => {
    loading.value = true
    errorMessage.value = ''
    try {
        await axios.post('/api/auth/forgot-password', { email: email.value })
        success.value = true
    } catch (e: any) {
        errorMessage.value = e.response?.data?.error || 'Failed to send reset link'
    } finally {
        loading.value = false
    }
}
</script>

<template>
  <div class="max-w-md mx-auto mt-20 p-6 bg-white rounded-2xl shadow-xl">
    <div class="text-center mb-8">
      <div class="bg-red-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
        <Mail class="w-8 h-8 text-red-500" />
      </div>
      <h2 class="text-3xl font-black text-gray-900">Forgot Password</h2>
      <p class="text-gray-500 mt-2">Enter your email and we'll send you a link to reset your password.</p>
    </div>

    <div v-if="success" class="text-center space-y-6">
      <div class="p-4 bg-green-50 text-green-700 rounded-lg border border-green-200">
        If an account exists for that email, we have sent password reset instructions.
      </div>
      <router-link to="/login" class="inline-block text-red-500 font-bold hover:text-red-600 transition">
        Return to Login
      </router-link>
    </div>

    <form v-else @submit.prevent="submit" class="space-y-5">
      <div>
        <label class="block text-sm font-bold text-gray-700 mb-1">Email Address</label>
        <input v-model="email" type="email" required
          class="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-red-500 focus:border-transparent transition"
          placeholder="ash@pallet.town" />
      </div>

      <div v-if="errorMessage" class="p-3 bg-red-50 text-red-600 text-sm rounded-lg border border-red-100">
        {{ errorMessage }}
      </div>

      <button type="submit" :disabled="loading"
        class="w-full bg-red-500 text-white font-bold py-3 px-4 rounded-xl hover:bg-red-600 transition flex justify-center items-center gap-2 shadow-lg shadow-red-200">
        <span v-if="loading" class="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></span>
        <span>Send Reset Link</span>
      </button>

      <div class="text-center mt-6">
        <router-link to="/login" class="text-gray-500 hover:text-gray-700 font-medium inline-flex items-center gap-2 transition">
          <ArrowLeft class="w-4 h-4" /> Back to Login
        </router-link>
      </div>
    </form>
  </div>
</template>
