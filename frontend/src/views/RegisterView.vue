<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'
import { useToast } from '../composables/useToast'

const username = ref('')
const email = ref('')
const password = ref('')
const confirmPassword = ref('')
const errorMessage = ref('')
const router = useRouter()
const toast = useToast()
const googleButtonContainer = ref(null)

const register = async () => {
    if (password.value !== confirmPassword.value) {
        errorMessage.value = "Passwords do not match."
        return
    }
    
    try {
        const response = await axios.post('api/auth/register', {
            username: username.value,
            email: email.value,
            password: password.value
        })
        localStorage.setItem('token', response.data.token)
        toast.success('Account created successfully!'); router.push('/')
    } catch (error: any) {
        errorMessage.value = error.response?.data || "Registration failed. Please try again."
    }
}

const handleGoogleCallback = async (response: any) => {
    try {
        const res = await axios.post('api/auth/google', {
            token: response.credential
        })
        localStorage.setItem('token', res.data.token)
        toast.success('Account created successfully!'); router.push('/')
    } catch (error: any) {
        errorMessage.value = error.response?.data || "Google login failed."
    }
}

onMounted(() => {
    const initGoogle = () => {
        if (window.google) {
            window.google.accounts.id.initialize({
                client_id: "984045651321-is49jggismfrm13bh501tbr7njvsu3vr.apps.googleusercontent.com",
                callback: handleGoogleCallback
            });
            window.google.accounts.id.renderButton(
                googleButtonContainer.value,
                { theme: "outline", size: "large", width: "100%" }
            );
        } else {
            setTimeout(initGoogle, 100);
        }
    }
    initGoogle();
})
</script>

<template>
  <div class="max-w-md mx-auto mt-10 bg-white p-8 border border-gray-200 rounded-xl shadow-sm">
    <div class="text-center mb-8">
      <h2 class="text-2xl font-bold text-gray-900">Create an account</h2>
    </div>
    
    <div v-if="errorMessage" class="mb-4 bg-red-50 border-l-4 border-red-500 p-4">
      <p class="text-sm text-red-700">{{ errorMessage }}</p>
    </div>

    <form @submit.prevent="register" class="space-y-6">
      <div>
        <label for="username" class="block text-sm font-medium text-gray-700">Username</label>
        <div class="mt-1">
          <input id="username" v-model="username" type="text" required class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-red-500 focus:border-red-500 sm:text-sm">
        </div>
      </div>
      <div>
        <label for="email" class="block text-sm font-medium text-gray-700">Email Address</label>
        <div class="mt-1">
          <input id="email" v-model="email" type="email" required class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-red-500 focus:border-red-500 sm:text-sm">
        </div>
      </div>
      <div>
        <label for="password" class="block text-sm font-medium text-gray-700">Password</label>
        <div class="mt-1">
          <input id="password" v-model="password" type="password" required class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-red-500 focus:border-red-500 sm:text-sm">
        </div>
      </div>
      <div>
        <label for="confirm-password" class="block text-sm font-medium text-gray-700">Confirm Password</label>
        <div class="mt-1">
          <input id="confirm-password" v-model="confirmPassword" type="password" required class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-red-500 focus:border-red-500 sm:text-sm">
        </div>
      </div>
      <div>
        <button type="submit" class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500">
          Create account
        </button>
      </div>
    </form>
    
    <div class="mt-6">
      <div class="relative">
        <div class="absolute inset-0 flex items-center">
          <div class="w-full border-t border-gray-300"></div>
        </div>
        <div class="relative flex justify-center text-sm">
          <span class="px-2 bg-white text-gray-500">Or continue with</span>
        </div>
      </div>

      <div class="mt-6 flex justify-center w-full">
        <div ref="googleButtonContainer" class="w-full flex justify-center"></div>
      </div>
    </div>

    <div class="mt-6 text-center text-sm text-gray-600">
      Already have an account? <router-link to="/login" class="font-medium text-red-600 hover:text-red-500">Sign in</router-link>
    </div>
  </div>
</template>
