<template>
  <nav class="fixed top-0 left-0 right-0 z-50 bg-[#FAFAFA]/80 backdrop-blur-md border-b border-gray-100">
    <div class="max-w-6xl mx-auto px-6 h-16 flex items-center justify-between">
      <!-- Logo -->
      <div class="text-lg font-bold tracking-tight">
        <span class="text-[#E8562A]">A</span>A
      </div>

      <!-- Nav Links -->
      <div class="hidden md:flex items-center gap-8">
        <a
          v-for="link in links"
          :key="link.label"
          :href="link.href"
          class="text-sm text-gray-500 hover:text-gray-900 transition-colors font-medium"
        >
          {{ link.label }}
        </a>
      </div>

      <!-- CTA -->
      <a
        href="mailto:adeoye@example.com"
        class="bg-[#0A0A0A] text-white text-sm px-5 py-2.5 rounded-full font-medium hover:bg-[#E8562A] transition-colors"
      >
        Contact
      </a>
    </div>
  </nav>
</template>

<script setup lang="ts">
const links = [
  { label: 'Experience', href: '#experience' },
  { label: 'Skills', href: '#skills' },
  { label: 'Projects', href: '#projects' },
  { label: 'About', href: '#about' },
]
</script>
