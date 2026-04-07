<template>
  <section id="projects" class="py-28 bg-white">
    <div class="max-w-6xl mx-auto px-6">

      <!-- Header -->
      <div class="flex flex-col sm:flex-row sm:items-end sm:justify-between mb-16 gap-4">
        <div>
          <p class="text-[#E8562A] text-xs font-semibold tracking-widest uppercase mb-3">Portfolio</p>
          <h2 class="text-4xl font-extrabold tracking-tight">Selected Projects</h2>
        </div>
        <a
          href="https://github.com/adeoye"
          target="_blank"
          rel="noopener"
          class="text-sm text-gray-400 hover:text-gray-900 transition-colors font-medium self-start sm:self-auto"
        >
          All on GitHub ↗
        </a>
      </div>

      <!-- Projects Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
        <a
          v-for="project in projects"
          :key="project.name"
          :href="project.href"
          target="_blank"
          rel="noopener"
          class="group bg-gray-50 rounded-2xl p-6 hover:bg-[#E8562A]/[0.04] hover:border-[#E8562A]/20 border border-transparent transition-all duration-200 cursor-pointer flex flex-col gap-4"
        >
          <!-- Icon + arrow -->
          <div class="flex items-start justify-between">
            <div
              class="w-10 h-10 rounded-xl flex items-center justify-center text-xl"
              :style="{ background: project.iconBg }"
            >
              {{ project.icon }}
            </div>
            <span class="text-gray-300 group-hover:text-[#E8562A] transition-colors text-lg">↗</span>
          </div>

          <!-- Name + description -->
          <div>
            <h3 class="font-bold text-base mb-1.5">{{ project.name }}</h3>
            <p class="text-gray-500 text-sm leading-relaxed">{{ project.description }}</p>
          </div>

          <!-- Tags -->
          <div class="flex flex-wrap gap-1.5 mt-auto">
            <span
              v-for="tag in project.tags"
              :key="tag"
              class="text-xs bg-white border border-gray-200 text-gray-500 px-2.5 py-1 rounded-full font-medium"
            >
              {{ tag }}
            </span>
          </div>
        </a>
      </div>

    </div>
  </section>
</template>

<script setup lang="ts">
interface Project {
  name: string
  description: string
  icon: string
  iconBg: string
  tags: string[]
  href: string
}

const projects: Project[] = [
  {
    name: 'Ferrite',
    description: 'High-performance async HTTP server built with Rust, Tokio, and Axum. Handles 100k+ req/s with a minimal memory footprint.',
    icon: '🦀',
    iconBg: 'rgba(232, 86, 42, 0.10)',
    tags: ['Rust', 'Tokio', 'Axum', 'PostgreSQL'],
    href: 'https://github.com/adeoye',
  },
  {
    name: 'Gortex',
    description: 'Distributed task queue system in Go with at-least-once delivery, exponential backoff retries, and a real-time monitoring dashboard.',
    icon: '⚙️',
    iconBg: 'rgba(0, 173, 216, 0.10)',
    tags: ['Go', 'Redis', 'gRPC', 'Docker'],
    href: 'https://github.com/adeoye',
  },
  {
    name: 'Nodelith',
    description: 'Opinionated Node.js GraphQL API framework with built-in auth, rate limiting, and OpenTelemetry observability out of the box.',
    icon: '⚡',
    iconBg: 'rgba(255, 193, 7, 0.10)',
    tags: ['TypeScript', 'Node.js', 'GraphQL', 'PostgreSQL'],
    href: 'https://github.com/adeoye',
  },
  {
    name: 'Vault CLI',
    description: 'Secrets management CLI built in Rust. Encrypts and syncs credentials across environments using ChaCha20-Poly1305.',
    icon: '🔐',
    iconBg: 'rgba(99, 102, 241, 0.10)',
    tags: ['Rust', 'CLI', 'Cryptography', 'WASM'],
    href: 'https://github.com/adeoye',
  },
  {
    name: 'Camlstream',
    description: 'Functional stream processing library in OCaml, exploring algebraic effects for composable, lazy, and zero-cost data pipelines.',
    icon: '🐪',
    iconBg: 'rgba(234, 88, 12, 0.07)',
    tags: ['OCaml', 'Functional', 'Streaming'],
    href: 'https://github.com/adeoye',
  },
  {
    name: 'Devwatch',
    description: 'Developer analytics dashboard aggregating GitHub, CI/CD, and deployment metrics. Built with a Go backend and Nuxt frontend.',
    icon: '📊',
    iconBg: 'rgba(16, 185, 129, 0.10)',
    tags: ['Go', 'Vue', 'Nuxt', 'TypeScript'],
    href: 'https://github.com/adeoye',
  },
]
</script>
