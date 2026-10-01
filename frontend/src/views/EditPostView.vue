<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { pokemonSeriesSets } from '../utils/pokemonSets'
import { useRouter, useRoute } from 'vue-router'
import { Upload } from 'lucide-vue-next'
import axios from 'axios'

const router = useRouter()
const route = useRoute()

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
const loading = ref(true)

const seriesSets = pokemonSeriesSets

const availableSets = computed(() => seriesSets[series.value] || [])

watch(series, (_, oldVal) => {
    if (oldVal) {
        setName.value = ''
    }
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
        console.error("Failed to upload image", error)
        errorMessage.value = "Failed to upload image. Please try again."
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

const submitForm = async () => {
    if (!pokemonName.value || !series.value || !setName.value || !condition.value || !price.value) {
        errorMessage.value = "Please fill in all required fields."
        return
    }

    try {
        const token = localStorage.getItem('token')
        await axios.put(`api/posts/${route.params.id}/edit`, {
            price: parseFloat(price.value),
            currency: currency.value,
            condition: condition.value,
            description: description.value,
            image_url: imageUrl.value
        }, {
            headers: { Authorization: `Bearer ${token}` }
        })
        router.push(`/posts/${route.params.id}`)
    } catch (error) {
        console.error("Failed to update post", error)
        errorMessage.value = "Failed to update listing. Please try again."
    }
}

onMounted(async () => {
    try {
        const token = localStorage.getItem('token')
        if (!token) {
            router.push('/login')
            return
        }
        
        const res = await axios.get(`api/posts/${route.params.id}`)
        const post = res.data
        
        pokemonName.value = post.pokemon_name
        price.value = (post.price_cents / 100).toString()
        currency.value = post.currency
        condition.value = post.condition
        series.value = post.series
        // We set a small timeout so the watcher doesn't clear the set name immediately
        setTimeout(() => {
            setName.value = post.set_name
        }, 50)
        description.value = post.description || ''
        imageUrl.value = post.image_url || ''
    } catch (error) {
        console.error("Failed to fetch post", error)
        errorMessage.value = "Failed to load listing for editing."
    } finally {
        loading.value = false
    }
})
</script>

<template>
  <div class="max-w-2xl mx-auto mt-10 bg-white p-8 border border-gray-200 rounded-xl shadow-sm">
    <div class="text-center mb-8">
      <h2 class="text-2xl font-bold text-gray-900">Edit Your Listing</h2>
      <p class="text-gray-500 text-sm mt-1">Update the price, condition, or description below.</p>
    </div>

    <div v-if="errorMessage" class="mb-4 bg-red-50 border-l-4 border-red-500 p-4 text-sm text-red-700">
      {{ errorMessage }}
    </div>

    <form @submit.prevent="submitForm" class="space-y-6">
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-6">
          <div>
            <label class="block text-sm font-medium text-gray-700">Card Name</label>
            <input v-model="pokemonName" type="text" disabled class="mt-1 block w-full px-3 py-2 border border-gray-300 bg-gray-100 rounded-md shadow-sm sm:text-sm">
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
            <select v-model="series" disabled class="mt-1 block w-full px-3 py-2 border border-gray-300 bg-gray-100 rounded-md shadow-sm sm:text-sm">
              <option disabled value="">Select a series</option>
              <option v-for="s in Object.keys(seriesSets)" :key="s" :value="s">{{ s }}</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700">Set / Expansion</label>
            <select v-model="setName" disabled class="mt-1 block w-full px-3 py-2 border border-gray-300 bg-gray-100 rounded-md shadow-sm sm:text-sm">
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
        <label class="block text-sm font-medium text-gray-700 mb-2">Card Image</label>
        
        <div v-if="imageUrl" class="relative w-40 h-56 rounded-lg overflow-hidden border border-gray-300 shadow-sm bg-gray-50">
            <img :src="imageUrl" class="w-full h-full object-cover" />
            <button @click.prevent="imageUrl = ''; imageFile = null" class="absolute top-1 right-1 bg-white rounded-full p-1 shadow hover:bg-gray-100">
                <svg class="w-4 h-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path></svg>
            </button>
        </div>
        
        <div v-else class="flex justify-center px-6 pt-5 pb-6 border-2 border-gray-300 border-dashed rounded-md hover:bg-gray-50 transition relative">
            <input type="file" @change="handleFileUpload" accept="image/*" class="absolute inset-0 w-full h-full opacity-0 cursor-pointer" :disabled="uploading">
            <div class="space-y-1 text-center flex flex-col items-center justify-center">
                <Upload v-if="!uploading" class="h-10 w-10 text-gray-400" />
                <div v-else class="h-10 w-10 border-4 border-red-500 border-t-transparent rounded-full animate-spin"></div>
                <div class="flex text-sm text-gray-600">
                    <span class="font-medium text-red-600 hover:text-red-500">Upload a file</span>
                    <p class="pl-1">or drag and drop</p>
                </div>
                <p class="text-xs text-gray-500">PNG, JPG, GIF up to 5MB</p>
            </div>
        </div>
      </div>

      <div>
        <label class="block text-sm font-medium text-gray-700">Description (Optional)</label>
        <textarea v-model="description" rows="4" class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-red-500 focus:border-red-500 sm:text-sm"></textarea>
      </div>

      <div class="pt-4">
        <button type="submit" class="w-full flex justify-center py-3 px-4 border border-transparent rounded-md shadow-sm text-sm font-bold text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
          UPDATE LISTING
        </button>
      </div>
    </form>
  </div>
</template>
