<script>
import axios from 'axios';

export default {
  name: 'HelloWorld',
  data() {
    return {
      rssData: null,
    };
  },
  async mounted() {
    const response = await axios.get('http://localhost:3030/rss');
    this.rssData = response.data;
    console.log(this.rssData);  // Add this line
  },
};
</script>

<template>
  <div class="hello container mx-auto px-4 text-left">
    <h1 class="my-4 text-4xl font-bold text-blue-500">{{ msg }}</h1>
    <ul class="grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
      <li class="mb-4 p-4 border rounded shadow-lg bg-white" v-for="(item, index) in rssData" :key="index">
        <h2 class="text-2xl mb-2 font-semibold text-gray-700">{{ item.title }}</h2>
        <p class="mb-2 text-gray-600">{{ item.description }}</p>
        <p class="mb-2 text-gray-500">Published at: {{ item.pub_date }}</p>
        <p class="mb-2">Matched keywords: <span class="inline-block bg-blue-500 text-white rounded-full px-2 py-1 text-xs font-bold mr-3" v-for="(keyword, i) in item.matched_keywords" :key="i">{{ keyword }}</span></p>
        <a :href="item.link" class="inline-block bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110">Read more</a>
      </li>
    </ul>
  </div>
</template>
