<script setup lang="ts">
import { ArrowLeft } from 'lucide-vue-next'
</script>

<template>
  <div class="max-w-3xl mx-auto py-12 px-6">
    <router-link to="/" class="inline-flex items-center text-red-500 hover:text-red-700 font-medium mb-8">
      <ArrowLeft class="w-4 h-4 mr-2" /> Back to Home
    </router-link>
    
    <h1 class="text-4xl font-black text-gray-900 mb-6">Terms of Service</h1>
    <p class="text-gray-500 mb-8">Last Updated: September 2026</p>
    
    <div class="prose prose-red max-w-none text-gray-700 space-y-6">
      <section>
        <h2 class="text-2xl font-bold text-gray-900 mb-3">1. Acceptance of Terms</h2>
        <p>By accessing or using PokéMart ("the Platform"), you agree to be bound by these Terms of Service. If you do not agree, you must not use the Platform.</p>
      </section>

      <section>
        <h2 class="text-2xl font-bold text-gray-900 mb-3">2. Description of Service</h2>
        <p>PokéMart is a classifieds board designed to connect buyers and sellers of Pokémon Trading Card Game items in the Dominican Republic. We provide a platform for discovering listings and negotiating prices via our Offer system.</p>
      </section>

      <section>
        <h2 class="text-2xl font-bold text-gray-900 mb-3">3. No Liability for Transactions</h2>
        <p><strong>PokéMart is not a party to any transaction.</strong> All payments, trades, shipping, and physical meetings are conducted off-platform (e.g., via WhatsApp) at your own risk. We do not offer buyer protection, escrow services, or refunds. Always exercise caution when meeting strangers to conduct a trade.</p>
      </section>

      <section>
        <h2 class="text-2xl font-bold text-gray-900 mb-3">4. User Conduct & Content Moderation</h2>
        <p>You agree to only upload appropriate content related to the Pokémon TCG. Any uploads containing NSFW material, gore, or illegal content will be automatically blocked by our moderation algorithms, and your account will be permanently banned.</p>
      </section>

      <section>
        <h2 class="text-2xl font-bold text-gray-900 mb-3">5. Age Requirement</h2>
        <p>You must be at least 13 years old to use the Platform. If you are under 18, you must have permission from a parent or legal guardian to conduct trades or sales.</p>
      </section>
    </div>
  </div>
</template>
