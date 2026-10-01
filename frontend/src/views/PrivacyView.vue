<script setup lang="ts">
import { ArrowLeft } from 'lucide-vue-next'
</script>

<template>
  <div class="max-w-3xl mx-auto py-12 px-6">
    <router-link to="/" class="inline-flex items-center text-red-500 hover:text-red-700 font-medium mb-8">
      <ArrowLeft class="w-4 h-4 mr-2" /> Back to Home
    </router-link>
    
    <h1 class="text-4xl font-black text-gray-900 mb-6">Privacy Policy</h1>
    <p class="text-gray-500 mb-8">Last Updated: September 2026</p>
    
    <div class="prose prose-red max-w-none text-gray-700 space-y-6">
      <section>
        <h2 class="text-2xl font-bold text-gray-900 mb-3">1. Information We Collect</h2>
        <p>When you register for PokéMart, we collect your email address, a chosen username, and a hashed version of your password. If you choose to add a phone number to facilitate WhatsApp deals, we store that as well.</p>
      </section>

      <section>
        <h2 class="text-2xl font-bold text-gray-900 mb-3">2. How We Use Your Information</h2>
        <p>Your information is used strictly to provide the PokéMart service. We use your email to verify your account and recover lost passwords. Your phone number is only revealed to buyers after you have explicitly accepted their offer on a listing.</p>
      </section>

      <section>
        <h2 class="text-2xl font-bold text-gray-900 mb-3">3. Image Processing</h2>
        <p>Images uploaded to the platform are securely processed and hosted via Cloudinary. All images undergo automated scanning via AWS Rekognition to prevent inappropriate content from appearing on our platform.</p>
      </section>

      <section>
        <h2 class="text-2xl font-bold text-gray-900 mb-3">4. Data Sharing</h2>
        <p>We do not sell your personal data to third parties. We only share data when required by law or to protect the safety of our users.</p>
      </section>

      <section>
        <h2 class="text-2xl font-bold text-gray-900 mb-3">5. Data Deletion</h2>
        <p>You can request the deletion of your account and all associated data at any time by contacting the platform administrators. Once deleted, it cannot be recovered.</p>
      </section>
    </div>
  </div>
</template>
