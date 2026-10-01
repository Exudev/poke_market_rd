<script setup lang="ts">
import { ref, onMounted } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'

const router = useRouter()
const offers = ref<any[]>([])
const loading = ref(true)
const errorMessage = ref('')

const fetchOffers = async () => {
    try {
        loading.value = true
        const token = localStorage.getItem('token')
        if (!token) {
            router.push('/login')
            return
        }
        
        const res = await axios.get('api/users/me/offers', {
            headers: { Authorization: `Bearer ${token}` }
        })
        offers.value = res.data
    } catch (error: any) {
        errorMessage.value = error.response?.data || "Failed to load offers."
    } finally {
        loading.value = false
    }
}

const updateStatus = async (id: string, status: string) => {
    try {
        const token = localStorage.getItem('token')
        await axios.put(`api/offers/${id}`, { status }, {
            headers: { Authorization: `Bearer ${token}` }
        })
        await fetchOffers()
    } catch (e: any) {
        console.error(e)
        alert(e.response?.data || "Failed to update offer.")
    }
}

onMounted(() => {
    fetchOffers()
})
</script>

<template>
  <div class="max-w-5xl mx-auto py-8 px-4">
    <h1 class="text-3xl font-black text-gray-900 mb-8">My Offers</h1>
    
    <div v-if="loading" class="text-center py-12 text-gray-500">Loading offers...</div>
    <div v-else-if="errorMessage" class="bg-red-50 text-red-700 p-4 rounded-xl border border-red-200">{{ errorMessage }}</div>
    <div v-else-if="offers.length === 0" class="text-center py-12 text-gray-500">No offers found.</div>
    
    <div v-else class="space-y-4">
        <div v-for="offer in offers" :key="offer.id" class="bg-white p-6 rounded-xl border border-gray-200 shadow-sm flex flex-col md:flex-row gap-6 items-center">
            
            <img v-if="offer.image_url" :src="offer.image_url" class="w-24 h-32 object-contain bg-gray-50 rounded-lg border border-gray-100" />
            <div v-else class="w-24 h-32 bg-gradient-to-br from-yellow-200 to-yellow-500 rounded-lg flex items-center justify-center text-yellow-800 font-bold text-xs opacity-50 text-center px-1">
                {{ offer.pokemon_name }}
            </div>

            <div class="flex-1">
                <div class="flex justify-between items-start">
                    <div>
                        <h3 class="font-bold text-lg text-gray-900">
                            <router-link :to="`/posts/${offer.post_id}`" class="hover:text-blue-600">{{ offer.pokemon_name }}</router-link>
                        </h3>
                        <p class="text-gray-500 text-sm mt-1">
                            {{ offer.type_str === 'sent' ? 'Sent to seller' : 'Received from ' + offer.buyer_username }}
                        </p>
                    </div>
                    <div class="text-right">
                        <div class="text-2xl font-black text-gray-900">
                            {{ offer.currency === 'DOP' ? 'RD$' : '$' }}{{ (offer.amount_cents / 100).toFixed(2) }}
                        </div>
                        <span class="inline-block mt-1 px-3 py-1 rounded-full text-xs font-bold uppercase" 
                              :class="{
                                'bg-yellow-100 text-yellow-800': offer.status === 'pending',
                                'bg-green-100 text-green-800': offer.status === 'accepted',
                                'bg-red-100 text-red-800': offer.status === 'rejected'
                              }">
                            {{ offer.status }}
                        </span>
                    </div>
                </div>

                <div v-if="offer.type_str === 'received' && offer.status === 'pending'" class="mt-4 flex gap-3">
                    <button @click="updateStatus(offer.id, 'accepted')" class="flex-1 bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded-lg transition">Accept</button>
                    <button @click="updateStatus(offer.id, 'rejected')" class="flex-1 bg-white border border-gray-300 hover:bg-gray-50 text-gray-700 font-bold py-2 px-4 rounded-lg transition">Reject</button>
                </div>
            </div>
        </div>
    </div>
  </div>
</template>
