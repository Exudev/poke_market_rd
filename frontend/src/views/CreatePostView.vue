<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { pokemonSeriesSets } from '../utils/pokemonSets'
import { useRouter } from 'vue-router'

import axios from 'axios'
import { useToast } from '../composables/useToast'

const router = useRouter()
const toast = useToast()

const pokemonName = ref('')
const price = ref('')
const currency = ref('USD')
const condition = ref('Near Mint (NM)')
const series = ref('')
const setName = ref('')
const description = ref('')
const imageFile = ref<File | null>(null)
const imageUrl = ref('')
const uploading = ref(false)
const errorMessage = ref('')

const seriesSets = pokemonSeriesSets

const availableSets = computed(() => seriesSets[series.value] || [])

watch(series, () => {
    setName.value = ''
})


const handleFileUpload = async (event: any) => {
    const file = event.target.files[0]
    if (!file) return

    imageFile.value = file
    uploading.value = true
    
    const formData = new FormData()
    formData.append('file', file)

    try {
        const token = localStorage.getItem('token')
        const response = await axios.post('api/upload', formData, {
            headers: {
                'Authorization': `Bearer ${token}`,
                'Content-Type': 'multipart/form-data'
            }
        })
        imageUrl.value = response.data.url
    } catch (error) {
        console.error('Upload failed:', error)
        errorMessage.value = "Failed to upload image."
    } finally {
        uploading.value = false
    }
}

const marketPrice = ref<any>(null)
const fetchingPrice = ref(false)

const checkMarketPrice = async () => {
    if (!pokemonName.value) {
        errorMessage.value = "Enter a Pokémon name first to check market price."
        return
    }
    fetchingPrice.value = true
    errorMessage.value = ''
    try {
        const response = await axios.get(`api/market-price`, {
            params: { name: pokemonName.value, set: setName.value }
        })
        marketPrice.value = response.data
    } catch (e) {
        console.error(e)
        errorMessage.value = "Could not fetch market price."
    } finally {
        fetchingPrice.value = false
    }
}

const createPost = async () => {
    try {
        const token = localStorage.getItem('token')
        
        let priceCents = 0
        if (price.value) {
            priceCents = Math.round(parseFloat(price.value) * 100)
        }

        await axios.post('api/posts', {
            pokemon_name: pokemonName.value,
            series: series.value,
            set_name: setName.value,
            condition: condition.value,
            price_cents: priceCents,
            currency: currency.value,
            description: description.value,
            image_url: imageUrl.value
        }, {
            headers: { Authorization: `Bearer ${token}` }
        })
        
        toast.success('Card listed successfully!'); router.push('/')
    } catch (error: any) {
        errorMessage.value = error.response?.data || "Failed to create listing."
    }
}
</script>

<template>
  <div class="max-w-2xl mx-auto mt-10 bg-white p-8 border border-gray-200 rounded-xl shadow-sm">
    <div class="text-center mb-8">
      <h2 class="text-2xl font-bold text-gray-900">Create Your Card Listing</h2>
      <p class="text-gray-500 text-sm mt-1">Fill in the details below to sell your Pokemon card.</p>
    </div>

    <div v-if="errorMessage" class="mb-4 bg-red-50 border-l-4 border-red-500 p-4 text-sm text-red-700">
      {{ errorMessage }}
    </div>

    <form @submit.prevent="createPost" class="space-y-6">
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
          <div>
            <label class="block text-sm font-medium text-gray-700">Card Name</label>
            <input v-model="pokemonName" type="text" placeholder="e.g., Charizard (Base Set)" required class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-red-500 focus:border-red-500 sm:text-sm">
          </div>
          <div>
            <div class="flex justify-between items-center mb-1">
                <label class="block text-sm font-medium text-gray-700">Price</label>
                <button type="button" @click="checkMarketPrice" :disabled="fetchingPrice" class="text-xs text-blue-600 font-bold hover:text-blue-800 transition">
                    {{ fetchingPrice ? 'Checking...' : 'Check Market Price' }}
                </button>
            </div>
            <div class="mt-1 flex rounded-md shadow-sm">
                <select v-model="currency" class="inline-flex items-center px-3 rounded-l-md border border-r-0 border-gray-300 bg-gray-50 text-gray-500 sm:text-sm">
                    <option value="DOP">DOP</option>
                    <option value="USD">USD</option>
                </select>
                <input v-model="price" type="number" step="0.01" min="0" required placeholder="0.00" class="flex-1 min-w-0 block w-full px-3 py-2 rounded-none rounded-r-md border border-gray-300 focus:ring-red-500 focus:border-red-500 sm:text-sm">
            </div>
            <div v-if="marketPrice" class="mt-2 text-xs text-green-700 bg-green-50 p-2 rounded border border-green-200">
                <p v-if="marketPrice.average_sell_price">Average Market Price: <strong>${{ marketPrice.average_sell_price.toFixed(2) }}</strong></p>
                <p v-else-if="marketPrice.low_price">Low Price: <strong>${{ marketPrice.low_price.toFixed(2) }}</strong></p>
                <p v-else>No market data found for this card.</p>
            </div>
          </div>
      </div>
      
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
          <div>
            <label class="block text-sm font-medium text-gray-700">Series</label>
            <select v-model="series" required class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-red-500 focus:border-red-500 sm:text-sm">
              <option disabled value="">Select a series</option>
              <option v-for="s in Object.keys(seriesSets)" :key="s" :value="s">{{ s }}</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">Set / Expansion</label>
            <select v-model="setName" :disabled="!series" required class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-red-500 focus:border-red-500 sm:text-sm">
              <option disabled value="">{{ series ? 'Select a set' : 'Select a series first' }}</option>
              <option v-for="setOpt in availableSets" :key="setOpt" :value="setOpt">{{ setOpt }}</option>
            </select>
          </div>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700">Condition</label>
        <select v-model="condition" required class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-red-500 focus:border-red-500 sm:text-sm">
            <option>Pristine (PSA 10)</option>
            <option>Near Mint (NM)</option>
            <option>Lightly Played (LP)</option>
            <option>Moderately Played (MP)</option>
            <option>Heavily Played (HP)</option>
            <option>Damaged</option>
        </select>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700">Card Image</label>
        <div class="mt-1 flex items-center gap-4">
            <input type="file" @change="handleFileUpload" accept="image/*" class="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-red-50 file:text-red-700 hover:file:bg-red-100 cursor-pointer border border-gray-300 rounded-md">
            <div v-if="uploading" class="text-sm text-gray-500 flex-shrink-0">Uploading...</div>
        </div>
        <div v-if="imageUrl" class="mt-3">
            <img :src="imageUrl" alt="Preview" class="h-32 object-contain rounded-md border border-gray-200">
        </div>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700">Description (Optional)</label>
        <textarea v-model="description" rows="4" class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-red-500 focus:border-red-500 sm:text-sm"></textarea>
      </div>

      <div class="pt-4">
        <button type="submit" :disabled="uploading" class="w-full flex justify-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-bold text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50">
          {{ uploading ? 'UPLOADING...' : 'CREATE LISTING' }}
        </button>
      </div>
    </form>
  </div>
</template>
