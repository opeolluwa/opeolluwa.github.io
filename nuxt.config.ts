// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  modules: ['@nuxt/hints', '@nuxt/image', '@nuxt/ui'],
  app: {
    head: {
      title: 'Adeoye Adefemi — Backend Engineer',
      meta: [
        { name: 'description', content: 'Backend Engineer specializing in Rust, Go, and TypeScript. Building performant, reliable systems.' }
      ],
      link: [
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
        { rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&display=swap' }
      ]
    }
  },
  css: ['~/assets/css/main.css'],
})
