<template>
  <footer id="about" class="py-20 bg-[#0A0A0A] text-white">
    <div class="max-w-6xl mx-auto px-6">
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center mb-16">

        <!-- Left: CTA -->
        <div>
          <p class="text-gray-500 text-sm mb-3">Open to opportunities</p>
          <h2 class="text-4xl font-extrabold tracking-tight mb-6">
            Let's build something<br>
            <span class="text-[#E8562A]">together.</span>
          </h2>
          <p class="text-gray-400 text-sm leading-relaxed max-w-sm">
            Whether you're looking for a backend engineer, want to collaborate on an
            open-source project, or just want to talk systems and compilers — I'm in.
          </p>
          <a
            href="mailto:adeoye@example.com"
            class="mt-8 inline-flex items-center gap-2 bg-[#E8562A] text-white px-6 py-3 rounded-full text-sm font-semibold hover:bg-[#cf4c25] transition-colors"
          >
            Send me a message <span>→</span>
          </a>
        </div>

        <!-- Right: Info -->
        <div class="lg:pl-16 border-l border-white/5">
          <div class="flex flex-col gap-8">
            <div>
              <p class="text-xs text-gray-500 uppercase tracking-widest mb-2">Based in</p>
              <p class="text-sm font-medium">Nigeria 🇳🇬</p>
            </div>
            <div>
              <p class="text-xs text-gray-500 uppercase tracking-widest mb-3">Find me online</p>
              <div class="flex flex-col gap-2">
                <a href="https://github.com/adeoye" target="_blank" class="text-sm text-gray-300 hover:text-white transition-colors flex items-center gap-2">
                  <span class="w-4 text-center">⌥</span> github.com/adeoye
                </a>
                <a href="https://linkedin.com/in/adeoye" target="_blank" class="text-sm text-gray-300 hover:text-white transition-colors flex items-center gap-2">
                  <span class="w-4 text-center">in</span> linkedin.com/in/adeoye
                </a>
                <a href="mailto:adeoye@example.com" class="text-sm text-gray-300 hover:text-white transition-colors flex items-center gap-2">
                  <span class="w-4 text-center">@</span> adeoye@example.com
                </a>
              </div>
            </div>
          </div>
        </div>

      </div>

      <!-- Bottom bar -->
      <div class="border-t border-white/5 pt-8 flex flex-col sm:flex-row items-center justify-between gap-4">
        <p class="text-xs text-gray-600">© 2026 Adeoye Adefemi. Built with Nuxt &amp; Tailwind.</p>
        <div class="flex items-center gap-1 text-xs text-gray-600">
          <span>Crafted in</span>
          <span class="text-[#E8562A]">Rust</span>
          <span>of spirit, Vue of soul.</span>
        </div>
      </div>
    </div>
  </footer>
</template>
