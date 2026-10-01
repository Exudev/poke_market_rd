<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import axios from 'axios'
import { useToast } from '../composables/useToast'

const route = useRoute()
const router = useRouter()
const post = ref<any>(null)
const loading = ref(true)
const errorMessage = ref('')
const currentUserId = ref<string | null>(null)
const toast = useToast()
const isFavorited = ref(false)

const isOwner = computed(() => {
    return currentUserId.value && post.value && currentUserId.value === post.value.user_id
})

const checkFavorite = async (token: string) => {
    try {
        const res = await axios.get('api/users/me/favorites', {
            headers: { Authorization: `Bearer ${token}` }
        })
        isFavorited.value = res.data.some((p: any) => p.id === route.params.id)
    } catch(e) {}
}

const toggleFavorite = async () => {
    const token = localStorage.getItem('token')
    if (!token) {
        alert("Please log in to favorite cards")
        return
    }
    try {
        const res = await axios.post(`api/posts/${post.value.id}/favorite`, {}, {
            headers: { Authorization: `Bearer ${token}` }
        })
        isFavorited.value = res.data.favorited
    } catch (e) {
        toast.error("Failed to update favorite")
    }
}

const showDeleteModal = ref(false)
const actionError = ref('')

const confirmDelete = () => {
    actionError.value = ''
    showDeleteModal.value = true
}

const executeDelete = async () => {
    try {
        const token = localStorage.getItem('token')
        await axios.delete(`api/posts/${post.value.id}/delete`, {
            headers: { Authorization: `Bearer ${token}` }
        })
        showDeleteModal.value = false
        router.push('/profile')
    } catch(e: any) {
        console.error(e)
        actionError.value = e.response?.data || "Failed to delete post. Please try again."
    }
}


const openWhatsApp = async () => {
    if (post.value && post.value.seller_phone) {
        // Track click on the backend
        try {
            const token = localStorage.getItem('token')
            if (token) {
                await axios.post(`api/posts/${post.value.id}/whatsapp_click`, {}, {
                    headers: { Authorization: `Bearer ${token}` }
                })
            }
        } catch (e) {
            console.error("Failed to track whatsapp click", e)
        }

        // Strip non-numeric characters for the whatsapp link
        const phone = post.value.seller_phone.replace(/\D/g, '')
        const message = encodeURIComponent(`Hi ${post.value.seller_username}, I'm interested in your ${post.value.pokemon_name} listed on PokéMart!`)
        window.open(`https://wa.me/${phone}?text=${message}`, '_blank')
    }
}

const showOfferModal = ref(false)
const offerAmount = ref('')
const offerActionError = ref('')

const openOfferModal = () => {
    offerActionError.value = ''
    offerAmount.value = ''
    showOfferModal.value = true
}

const submitOffer = async () => {
    if (!offerAmount.value || isNaN(Number(offerAmount.value))) {
        offerActionError.value = "Please enter a valid amount."
        return
    }

    try {
        const token = localStorage.getItem('token')
        if (!token) {
            offerActionError.value = "Please log in to make an offer."
            return
        }

        await axios.post(`api/posts/${post.value.id}/offers`, {
            amount: parseFloat(offerAmount.value),
            currency: post.value.currency
        }, {
            headers: { Authorization: `Bearer ${token}` }
        })
        
        offerStatus.value = 'pending'
        showOfferModal.value = false
        toast.success('Offer submitted successfully!')
    } catch(e: any) {
        console.error(e)
        offerActionError.value = e.response?.data || "Failed to submit offer."
        toast.error(offerActionError.value)
    }
}

const offerStatus = ref<string | null>(null)
const marketPrice = ref<any>(null)

onMounted(async () => {
  try {
    const token = localStorage.getItem('token')
    
    // Fetch post details first
    const response = await axios.get(`api/posts/${route.params.id}`)
    post.value = response.data

    // Fetch market price
    try {
        const mpResponse = await axios.get(`api/market-price`, {
            params: { name: post.value.pokemon_name, set: post.value.set_name }
        })
        marketPrice.value = mpResponse.data
    } catch (e) {
        console.error("Failed to fetch market price", e)
    }

    if (token) {
        try {
            const payload = JSON.parse(atob(token.split('.')[1]))
            currentUserId.value = payload.sub
            checkFavorite(token)

            // Check if user has an offer on this post
            if (currentUserId.value !== post.value.user_id) {
                const offersRes = await axios.get(`api/users/me/offers`, {
                    headers: { Authorization: `Bearer ${token}` }
                })
                const myOffer = offersRes.data.find((o: any) => o.post_id === post.value.id && o.type_str === 'sent')
                if (myOffer) {
                    offerStatus.value = myOffer.status
                }
            }
        } catch (e) {
            console.error("Failed to fetch user offers or token", e)
        }
    }
  } catch (error: any) {
    errorMessage.value = error.response?.data || "Failed to load listing."
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="max-w-5xl mx-auto py-8">
    <div v-if="loading" class="text-center py-12 text-gray-500">Loading...</div>
    <div v-else-if="errorMessage" class="text-center py-12 text-red-500">{{ errorMessage }}</div>
    
    <div v-else class="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden flex flex-col md:flex-row">
      <!-- Left: Image -->
      <div class="w-full md:w-1/2 bg-gray-100 min-h-[400px] flex items-center justify-center p-8 relative overflow-hidden">
        <img v-if="post.image_url" :src="post.image_url" class="absolute inset-0 w-full h-full object-contain p-4" />
        <div v-else class="w-full aspect-[3/4] max-w-sm">
            <div class="w-full h-full bg-gradient-to-br from-yellow-200 to-yellow-500 rounded-lg shadow-xl border-[10px] border-yellow-300 flex items-center justify-center text-yellow-800 font-bold text-2xl opacity-50 text-center px-4">
                {{ post.pokemon_name }}
            </div>
        </div>
      </div>
      
      <!-- Right: Details -->
      <div class="w-full md:w-1/2 p-8 flex flex-col">
        <!-- Seller Info -->
        <div class="flex items-center gap-3 mb-6 pb-6 border-b border-gray-100">
            <img :src="post.seller_avatar_url || 'https://ui-avatars.com/api/?name=' + post.seller_username + '&background=random'" class="w-10 h-10 rounded-full" />
            <div>
                <p class="text-sm text-gray-500">Listed by</p>
                <p class="font-bold text-gray-900">{{ post.seller_username }}</p>
            </div>
        </div>

        <div class="flex items-start justify-between gap-4 mb-2">
            <h1 class="text-3xl font-bold text-gray-900">{{ post.pokemon_name }}</h1>
            <button @click="toggleFavorite" class="p-2 rounded-full border-2 transition-colors flex-shrink-0" :class="isFavorited ? 'border-red-500 bg-red-50' : 'border-gray-200 hover:border-gray-300'">
                <svg class="w-6 h-6 transition-colors" :class="isFavorited ? 'text-red-500 fill-current' : 'text-gray-400'" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path></svg>
            </button>
        </div>
        <p class="text-lg text-gray-500 mb-6">{{ post.set_name }} &bull; {{ post.series }}</p>
        
        <div class="bg-gray-50 rounded-xl p-6 mb-6">
            <div class="flex justify-between items-center mb-2">
                <span class="text-gray-500">Condition</span>
                <span class="font-bold text-green-600 px-3 py-1 bg-green-100 rounded-full text-sm">{{ post.condition }}</span>
            </div>
            
            <div v-if="marketPrice && marketPrice.average_sell_price" class="flex justify-between items-center mt-4 pt-4 border-t border-gray-200">
                <span class="text-gray-500 flex items-center gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-blue-500"><path d="M22 12h-4l-3 9L9 3l-3 9H2"/></svg>
                    Market Price (USD)
                </span>
                <span class="font-bold text-gray-700">
                    ${{ marketPrice.average_sell_price.toFixed(2) }}
                </span>
            </div>

            <div class="flex justify-between items-end mt-4 pt-4 border-t border-gray-200">
                <span class="text-gray-500">Asking Price</span>
                <span class="text-4xl font-black text-gray-900">
                    {{ post.currency === 'DOP' ? 'RD$' : '$' }}{{ (post.price_cents / 100).toFixed(2) }}
                </span>
            </div>
        </div>
        
        <div v-if="!isOwner" class="mb-8 space-y-3">
            <template v-if="offerStatus === 'accepted'">
                <div class="p-4 bg-green-50 border border-green-200 rounded-xl mb-3 text-green-800 text-sm">
                    <strong>Offer Accepted!</strong> The seller has accepted your offer. You can now contact them on WhatsApp to arrange the deal.
                </div>
                <button v-if="post.seller_phone" @click="openWhatsApp" class="w-full bg-green-500 hover:bg-green-600 text-white font-bold py-4 rounded-xl flex items-center justify-center gap-2 transition shadow-sm">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5"><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>
                    Take me to WhatsApp
                </button>
                <div v-else class="text-center text-sm text-gray-500 py-2">
                    This seller hasn't provided a phone number.
                </div>
            </template>
            <template v-else-if="offerStatus === 'pending'">
                <button disabled class="w-full bg-gray-100 text-gray-500 font-bold py-4 rounded-xl border border-gray-200 cursor-not-allowed">
                    Offer Pending
                </button>
            </template>
            <template v-else-if="offerStatus === 'rejected'">
                <div class="p-4 bg-red-50 border border-red-200 rounded-xl mb-3 text-red-800 text-sm">
                    Your offer was rejected.
                </div>
                <button @click="openOfferModal" class="w-full bg-white hover:bg-gray-50 text-gray-700 font-bold py-4 rounded-xl border border-gray-300 transition shadow-sm">
                    Make a New Offer
                </button>
            </template>
            <template v-else>
                <button @click="openOfferModal" class="w-full bg-white hover:bg-gray-50 text-gray-700 font-bold py-4 rounded-xl border border-gray-300 transition shadow-sm">
                    Make an Offer
                </button>
            </template>
        </div>
        <div v-else class="mb-8 flex gap-3">
            <router-link :to="`/posts/${post.id}/edit`" class="flex-1 bg-white hover:bg-gray-50 text-gray-700 font-bold py-4 rounded-xl border border-gray-300 transition shadow-sm text-center">
                Edit Listing
            </router-link>
            <button @click="confirmDelete" class="flex-1 bg-red-100 hover:bg-red-200 text-red-700 font-bold py-4 rounded-xl transition shadow-sm">
                Delete Listing
            </button>
        </div>
        
        <div v-if="post.description">
            <h3 class="font-bold text-gray-900 mb-2">Description</h3>
            <p class="text-gray-700 leading-relaxed whitespace-pre-wrap">{{ post.description }}</p>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteModal" class="fixed inset-0 z-50 overflow-y-auto">
        <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
            <div class="fixed inset-0 transition-opacity" aria-hidden="true">
                <div class="absolute inset-0 bg-gray-500 opacity-75" @click="showDeleteModal = false"></div>
            </div>
            
            <span class="hidden sm:inline-block sm:align-middle sm:h-screen" aria-hidden="true">&#8203;</span>
            
            <div class="inline-block align-bottom bg-white rounded-xl text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-md w-full relative z-10">
                <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
                    <div class="sm:flex sm:items-start">
                        <div class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10">
                            <svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                            </svg>
                        </div>
                        <div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left">
                            <h3 class="text-lg leading-6 font-medium text-gray-900">Delete Listing</h3>
                            <div class="mt-2">
                                <p class="text-sm text-gray-500">
                                    Are you sure you want to delete this listing? This action cannot be undone.
                                </p>
                            </div>
                            <div v-if="actionError" class="mt-4 bg-red-50 border-l-4 border-red-500 p-4 text-sm text-red-700">
                                {{ actionError }}
                            </div>
                        </div>
                    </div>
                </div>
                <div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
                    <button @click="executeDelete" type="button" class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-red-600 text-base font-medium text-white hover:bg-red-700 focus:outline-none sm:ml-3 sm:w-auto sm:text-sm">
                        Delete
                    </button>
                    <button @click="showDeleteModal = false" type="button" class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm">
                        Cancel
                    </button>
                </div>
            </div>
        </div>
    </div>

    <!-- Make Offer Modal -->
    <div v-if="showOfferModal" class="fixed inset-0 z-50 overflow-y-auto">
        <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
            <div class="fixed inset-0 transition-opacity" aria-hidden="true">
                <div class="absolute inset-0 bg-gray-500 opacity-75" @click="showOfferModal = false"></div>
            </div>
            
            <span class="hidden sm:inline-block sm:align-middle sm:h-screen" aria-hidden="true">&#8203;</span>
            
            <div class="inline-block align-bottom bg-white rounded-xl text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-md w-full relative z-10">
                <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
                    <div class="sm:flex sm:items-start">
                        <div class="mt-3 text-center sm:mt-0 sm:text-left w-full">
                            <h3 class="text-lg leading-6 font-medium text-gray-900 mb-4">Make an Offer</h3>
                            
                            <div v-if="offerActionError" class="mb-4 bg-red-50 border-l-4 border-red-500 p-4 text-sm text-red-700">
                                {{ offerActionError }}
                            </div>

                            <div>
                                <label class="block text-sm font-medium text-gray-700 mb-1">Your Offer ({{ post.currency }})</label>
                                <div class="mt-1 relative rounded-md shadow-sm">
                                    <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                        <span class="text-gray-500 sm:text-sm">{{ post.currency === 'DOP' ? 'RD$' : '$' }}</span>
                                    </div>
                                    <input type="number" step="0.01" v-model="offerAmount" class="focus:ring-blue-500 focus:border-blue-500 block w-full pl-12 pr-12 sm:text-sm border-gray-300 rounded-md py-3 border" placeholder="0.00" />
                                </div>
                                <p class="mt-2 text-sm text-gray-500">Asking price: {{ post.currency === 'DOP' ? 'RD$' : '$' }}{{ (post.price_cents / 100).toFixed(2) }}</p>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
                    <button @click="submitOffer" type="button" class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-blue-600 text-base font-medium text-white hover:bg-blue-700 focus:outline-none sm:ml-3 sm:w-auto sm:text-sm">
                        Submit Offer
                    </button>
                    <button @click="showOfferModal = false" type="button" class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm">
                        Cancel
                    </button>
                </div>
            </div>
        </div>
    </div>

  </div>
</template>
