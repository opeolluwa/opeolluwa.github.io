<template>
  <section class="min-h-screen pt-16 flex items-center bg-[#FAFAFA]">
    <div class="max-w-6xl mx-auto px-6 w-full py-20">
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">

        <!-- Left: Text -->
        <div class="flex flex-col gap-5">
          <p class="text-gray-400 text-base font-medium tracking-wide">Hey, This is</p>

          <h1 class="text-5xl xl:text-6xl font-extrabold leading-[1.1] tracking-tight">
            Adeoye<br>
            <span class="text-[#E8562A] relative inline-block">
              Adefemi
              <!-- underline squiggle decoration -->
              <svg class="absolute -bottom-1 left-0 w-full" height="6" viewBox="0 0 200 6" fill="none" preserveAspectRatio="none">
                <path d="M0 3 Q25 0 50 3 Q75 6 100 3 Q125 0 150 3 Q175 6 200 3" stroke="#E8562A" stroke-width="2" stroke-opacity="0.4" fill="none"/>
              </svg>
            </span>
          </h1>

          <p class="text-xl text-gray-500 font-medium">a Backend Engineer</p>

          <p class="text-gray-500 max-w-md leading-relaxed text-sm">
            I build robust, high-performance systems in Rust, Go, and TypeScript.
            Passionate about correctness, observability, and APIs that don't lie.
          </p>

          <div class="flex items-center gap-4 mt-2">
            <a
              href="#projects"
              class="bg-[#E8562A] text-white px-6 py-3 rounded-full text-sm font-semibold hover:bg-[#cf4c25] transition-colors inline-flex items-center gap-2"
            >
              View Projects <span class="text-base">→</span>
            </a>
            <a
              href="https://github.com/adeoye"
              target="_blank"
              rel="noopener"
              class="text-sm text-gray-500 hover:text-gray-900 transition-colors font-medium flex items-center gap-1"
            >
              GitHub ↗
            </a>
          </div>

          <!-- Social links -->
          <div class="flex items-center gap-4 mt-2 pt-4 border-t border-gray-100">
            <a href="https://linkedin.com/in/adeoye" target="_blank" class="text-xs text-gray-400 hover:text-gray-700 transition-colors">LinkedIn</a>
            <span class="text-gray-200">·</span>
            <a href="https://twitter.com" target="_blank" class="text-xs text-gray-400 hover:text-gray-700 transition-colors">Twitter / X</a>
            <span class="text-gray-200">·</span>
            <a href="mailto:adeoye@example.com" class="text-xs text-gray-400 hover:text-gray-700 transition-colors">Email</a>
          </div>
        </div>

        <!-- Right: Profile visual -->
        <div class="relative flex items-center justify-center lg:justify-end">
          <div class="relative">

            <!-- Profile card -->
            <div class="w-64 h-72 xl:w-72 xl:h-80 bg-[#111827] rounded-3xl overflow-hidden flex items-center justify-center shadow-2xl">
              <!-- Replace src with your actual photo: public/avatar.jpg -->
              <div class="text-white/10 text-8xl font-black select-none">AA</div>
            </div>

            <!-- Experience bubble (bottom-left) -->
            <div class="absolute -bottom-5 -left-10 bg-white shadow-xl rounded-2xl px-4 py-3 flex items-center gap-3 border border-gray-100">
              <div class="w-9 h-9 bg-[#E8562A]/10 rounded-xl flex items-center justify-center">
                <span class="text-[#E8562A] text-base">⚡</span>
              </div>
              <div>
                <div class="font-bold text-base leading-none">5+ yrs</div>
                <div class="text-xs text-gray-400 mt-0.5">Experience</div>
              </div>
            </div>

            <!-- Tech bubble (top-right) -->
            <div class="absolute -top-5 -right-8 bg-white shadow-lg rounded-xl px-3 py-2 border border-gray-100 flex items-center gap-1.5">
              <span class="text-base">🦀</span>
              <span class="text-xs font-semibold text-gray-700">Rust</span>
            </div>

            <!-- Decorative dots -->
            <div class="absolute -top-8 left-1/3 w-3 h-3 bg-[#E8562A] rounded-full opacity-60"></div>
            <div class="absolute -bottom-8 right-1/4 w-2 h-2 bg-[#E8562A] rounded-full opacity-40"></div>
            <div class="absolute top-1/2 -left-12 text-gray-200 text-3xl font-thin">+</div>
            <div class="absolute top-8 -left-6 w-4 h-4 border-2 border-gray-200 rounded-sm rotate-12"></div>
          </div>
        </div>

      </div>
    </div>
  </section>
</template>
