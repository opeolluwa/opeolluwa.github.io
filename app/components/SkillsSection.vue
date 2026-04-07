<template>
  <section id="skills" class="py-28 bg-gray-50">
    <div class="max-w-6xl mx-auto px-6">

      <p class="text-[#E8562A] text-xs font-semibold tracking-widest uppercase mb-3">Expertise</p>
      <h2 class="text-4xl font-extrabold tracking-tight mb-16">Tech Stack</h2>

      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-5">
        <div
          v-for="category in skills"
          :key="category.title"
          class="bg-white rounded-2xl p-6 border border-gray-100"
        >
          <h3 class="text-xs text-gray-400 font-semibold uppercase tracking-widest mb-5">
            {{ category.title }}
          </h3>
          <div class="flex flex-col gap-4">
            <div v-for="skill in category.items" :key="skill.name" class="flex items-center gap-3">
              <div
                class="w-8 h-8 rounded-lg flex items-center justify-center text-base shrink-0"
                :style="{ background: skill.bg }"
              >
                {{ skill.icon }}
              </div>
              <div>
                <div class="text-sm font-semibold leading-none">{{ skill.name }}</div>
                <div v-if="skill.note" class="text-xs text-gray-400 mt-0.5 italic">{{ skill.note }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>

    </div>
  </section>
</template>

<script setup lang="ts">
interface Skill {
  name: string
  icon: string
  bg: string
  note?: string
}

interface Category {
  title: string
  items: Skill[]
}

const skills: Category[] = [
  {
    title: 'Systems',
    items: [
      { name: 'Rust', icon: '🦀', bg: 'rgba(232, 86, 42, 0.10)' },
      { name: 'Go', icon: '🐹', bg: 'rgba(0, 173, 216, 0.10)' },
      { name: 'OCaml', icon: '🐪', bg: 'rgba(234, 88, 12, 0.08)', note: 'exploring' },
    ],
  },
  {
    title: 'Web & APIs',
    items: [
      { name: 'TypeScript', icon: '📘', bg: 'rgba(49, 120, 198, 0.10)' },
      { name: 'Node.js', icon: '🟢', bg: 'rgba(104, 160, 99, 0.10)' },
      { name: 'GraphQL', icon: '◈', bg: 'rgba(225, 0, 152, 0.10)' },
      { name: 'REST', icon: '🔗', bg: 'rgba(100, 116, 139, 0.10)' },
    ],
  },
  {
    title: 'Frontend',
    items: [
      { name: 'Vue 3', icon: '💚', bg: 'rgba(66, 184, 131, 0.10)' },
      { name: 'Nuxt 4', icon: '▲', bg: 'rgba(0, 220, 130, 0.08)' },
    ],
  },
  {
    title: 'Infrastructure',
    items: [
      { name: 'PostgreSQL', icon: '🐘', bg: 'rgba(51, 103, 145, 0.10)' },
      { name: 'Redis', icon: '🔴', bg: 'rgba(220, 50, 47, 0.10)' },
      { name: 'Docker', icon: '🐳', bg: 'rgba(35, 144, 209, 0.10)' },
      { name: 'gRPC', icon: '⚡', bg: 'rgba(255, 193, 7, 0.08)' },
    ],
  },
]
</script>
