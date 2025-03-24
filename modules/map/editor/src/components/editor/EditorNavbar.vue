<template>
  <div class="h-screen w-44 bg-primary-200/25 border-r border-r-primary-200 px-2 py-4">

    <!-- Links -->
    <div class="flex flex-col gap-1 h-full">
      <!-- Entities -->
      <router-link :class="linkClass" :to="{name: 'editor_entities'}">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-white">
          <path
            d="M17 14a5 5 0 0 0 2.71-.81L20 13a3.16 3.16 0 0 0 .45-.37l.21-.2a4.48 4.48 0 0 0 .48-.58l.06-.08a4.28 4.28 0 0 0 .41-.76 1.57 1.57 0 0 0 .09-.23 4.21 4.21 0 0 0 .2-.63l.06-.25A5.5 5.5 0 0 0 22 9V2l-3 3h-4l-3-3v7a5 5 0 0 0 5 5zm2-7a1 1 0 1 1-1 1 1 1 0 0 1 1-1zm-4 0a1 1 0 1 1-1 1 1 1 0 0 1 1-1z"></path>
          <path
            d="M11 22v-5H8v5H5V11.9a3.49 3.49 0 0 1-2.48-1.64A3.59 3.59 0 0 1 2 8.5 3.65 3.65 0 0 1 6 5a1.89 1.89 0 0 0 2-2 1 1 0 0 1 1-1 1 1 0 0 1 1 1 3.89 3.89 0 0 1-4 4C4.19 7 4 8.16 4 8.51S4.18 10 6 10h5.09A6 6 0 0 0 19 14.65V22h-3v-5h-2v5z"></path>
        </svg>
        <p>Entités</p>
      </router-link>

      <!-- Map editor -->
      <router-link :class="linkClass" :to="{name: 'editor_entities'}">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-white">
          <path
            d="m21.447 6.105-6-3a1 1 0 0 0-.895 0L9 5.882 3.447 3.105A1 1 0 0 0 2 4v13c0 .379.214.725.553.895l6 3a1 1 0 0 0 .895 0L15 18.118l5.553 2.776a.992.992 0 0 0 .972-.043c.295-.183.475-.504.475-.851V7c0-.379-.214-.725-.553-.895zM10 7.618l4-2v10.764l-4 2V7.618zm-6-2 4 2v10.764l-4-2V5.618zm16 12.764-4-2V5.618l4 2v10.764z"></path>
        </svg>
        <p>Carte</p>
      </router-link>

      <!-- Settings -->
      <router-link :class="linkClass" :to="{name: 'editor_entities'}">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-white">
          <path
            d="m2.344 15.271 2 3.46a1 1 0 0 0 1.366.365l1.396-.806c.58.457 1.221.832 1.895 1.112V21a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1v-1.598a8.094 8.094 0 0 0 1.895-1.112l1.396.806c.477.275 1.091.11 1.366-.365l2-3.46a1.004 1.004 0 0 0-.365-1.366l-1.372-.793a7.683 7.683 0 0 0-.002-2.224l1.372-.793c.476-.275.641-.89.365-1.366l-2-3.46a1 1 0 0 0-1.366-.365l-1.396.806A8.034 8.034 0 0 0 15 4.598V3a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v1.598A8.094 8.094 0 0 0 7.105 5.71L5.71 4.904a.999.999 0 0 0-1.366.365l-2 3.46a1.004 1.004 0 0 0 .365 1.366l1.372.793a7.683 7.683 0 0 0 0 2.224l-1.372.793c-.476.275-.641.89-.365 1.366zM12 8c2.206 0 4 1.794 4 4s-1.794 4-4 4-4-1.794-4-4 1.794-4 4-4z"></path>
        </svg>
        <p>Paramètres</p>
      </router-link>


      <!-- Save -->
      <div :class="linkClass" class="mt-auto">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="fill-white">
          <path
            d="M5 21h14a2 2 0 0 0 2-2V8l-5-5H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2zM7 5h4v2h2V5h2v4H7V5zm0 8h10v6H7v-6z"></path>
        </svg>
        <p>Sauvegarder</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const linkClass = 'cursor-pointer select-none flex flex-row items-center gap-2 hover:bg-primary-200/40 rounded-lg p-2'
</script>