<template>
  <section id="experience" class="py-28 bg-white">
    <div class="max-w-6xl mx-auto px-6">

      <p class="text-[#E8562A] text-xs font-semibold tracking-widest uppercase mb-3">Background</p>
      <h2 class="text-4xl font-extrabold tracking-tight mb-16">Experience</h2>

      <div class="max-w-2xl flex flex-col gap-0">
        <div
          v-for="(item, i) in experience"
          :key="item.company"
          class="relative flex gap-8 pb-12 last:pb-0"
        >
          <!-- Timeline line -->
          <div class="flex flex-col items-center">
            <div class="w-3 h-3 rounded-full bg-[#E8562A] mt-1 shrink-0 z-10"></div>
            <div v-if="i < experience.length - 1" class="w-px flex-1 bg-gray-100 mt-2"></div>
          </div>

          <!-- Content -->
          <div class="pb-0">
            <div class="flex items-center gap-3 mb-1">
              <h3 class="font-bold text-base">{{ item.role }}</h3>
              <span
                v-if="item.current"
                class="text-xs bg-[#E8562A]/10 text-[#E8562A] px-2 py-0.5 rounded-full font-medium"
              >
                Current
              </span>
            </div>
            <div class="flex items-center gap-2 mb-3">
              <span class="text-sm font-medium text-gray-700">{{ item.company }}</span>
              <span class="text-gray-200">·</span>
              <span class="text-sm text-gray-400">{{ item.period }}</span>
            </div>
            <p class="text-sm text-gray-500 leading-relaxed">{{ item.description }}</p>
            <div class="flex flex-wrap gap-1.5 mt-3">
              <span
                v-for="tag in item.tags"
                :key="tag"
                class="text-xs bg-gray-50 border border-gray-100 text-gray-500 px-2.5 py-1 rounded-full"
              >
                {{ tag }}
              </span>
            </div>
          </div>
        </div>
      </div>

    </div>
  </section>
</template>

<script setup lang="ts">
interface Experience {
  role: string
  company: string
  period: string
  description: string
  tags: string[]
  current?: boolean
}

const experience: Experience[] = [
  {
    role: 'Senior Backend Engineer',
    company: 'Your Company',
    period: '2023 – Present',
    description: 'Building high-throughput data pipelines and microservices. Led the migration of core services from Node.js to Rust, achieving a 4× reduction in latency and 60% lower memory usage.',
    tags: ['Rust', 'Go', 'PostgreSQL', 'Kafka', 'gRPC'],
    current: true,
  },
  {
    role: 'Backend Engineer',
    company: 'Previous Company',
    period: '2021 – 2023',
    description: 'Designed and maintained RESTful and GraphQL APIs serving millions of requests per day. Introduced observability tooling with OpenTelemetry across the backend fleet.',
    tags: ['TypeScript', 'Node.js', 'GraphQL', 'Redis', 'Docker'],
  },
  {
    role: 'Software Engineer',
    company: 'Earlier Company',
    period: '2019 – 2021',
    description: 'Full-stack development with a focus on backend services. Built internal tooling and automated deployment pipelines that cut release time from hours to minutes.',
    tags: ['Node.js', 'Vue', 'PostgreSQL', 'CI/CD'],
  },
]
</script>
