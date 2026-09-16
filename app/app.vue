<script setup>
useHead({
    meta: [{ name: "viewport", content: "width=device-width, initial-scale=1" }],
    link: [{ rel: "icon", href: "/favicon.ico" }],
    htmlAttrs: {
        lang: "en",
    },
});

const title = "Adefemi Adeoye";
const description = "Backend and Systems Engineer";

useSeoMeta({
    title,
    description,
    ogTitle: title,
    ogDescription: description,
    ogImage: "https://ui.nuxt.com/assets/templates/nuxt/starter-light.png",
    twitterCard: "summary_large_image",
});
</script>

<template>
    <UApp>
        <UHeader>
            <template #left>
                <NuxtLink
                    to="/"
                    class="focus-visible:outline-3 outline-primary/25 rounded-md p-1 -ms-1"
                >
                    <AppLogo class="w-auto h-6 shrink-0" />
                </NuxtLink>
            </template>

            <template #right>
                <UButton
                    to="https://github.com/opeolluwa"
                    target="_blank"
                    icon="i-simple-icons-github"
                    aria-label="GitHub"
                    color="neutral"
                    variant="ghost"
                />
            </template>
        </UHeader>

        <UMain>
            <NuxtPage />
        </UMain>

        <USeparator icon="i-simple-icons-nuxtdotjs" />

        <UFooter>
            <template #right>
                <UButton
                    to="https://x.com/_opeolluwa"
                    target="_blank"
                    icon="i-simple-icons-github"
                    aria-label="GitHub"
                    color="neutral"
                    variant="ghost"
                />
            </template>
        </UFooter>
    </UApp>
</template>
