<script setup lang="ts">
import { useToast } from '../composables/useToast'
import { CheckCircle, XCircle, AlertCircle, Info } from 'lucide-vue-next'

const { toasts, removeToast } = useToast()
</script>

<template>
  <div class="fixed bottom-20 md:bottom-6 right-4 z-50 flex flex-col gap-2 pointer-events-none">
    <TransitionGroup 
      enter-active-class="transition duration-300 ease-out"
      enter-from-class="transform translate-y-4 opacity-0"
      enter-to-class="transform translate-y-0 opacity-100"
      leave-active-class="transition duration-200 ease-in"
      leave-from-class="transform translate-y-0 opacity-100"
      leave-to-class="transform translate-y-2 opacity-0"
    >
      <div 
        v-for="toast in toasts" 
        :key="toast.id"
        class="pointer-events-auto flex items-center gap-3 px-4 py-3 rounded-xl shadow-lg border w-80 backdrop-blur-sm"
        :class="{
          'bg-green-50/90 border-green-200 text-green-800': toast.type === 'success',
          'bg-red-50/90 border-red-200 text-red-800': toast.type === 'error',
          'bg-yellow-50/90 border-yellow-200 text-yellow-800': toast.type === 'warning',
          'bg-blue-50/90 border-blue-200 text-blue-800': toast.type === 'info',
        }"
      >
        <CheckCircle v-if="toast.type === 'success'" class="w-5 h-5 flex-shrink-0 text-green-500" />
        <XCircle v-if="toast.type === 'error'" class="w-5 h-5 flex-shrink-0 text-red-500" />
        <AlertCircle v-if="toast.type === 'warning'" class="w-5 h-5 flex-shrink-0 text-yellow-500" />
        <Info v-if="toast.type === 'info'" class="w-5 h-5 flex-shrink-0 text-blue-500" />
        
        <p class="text-sm font-medium flex-1">{{ toast.message }}</p>
        
        <button @click="removeToast(toast.id)" class="opacity-50 hover:opacity-100 transition">
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>
