<script>
import axios from "axios";
export default {
  data() {
    return {
      posts: []
    };
  },
  mounted() {
    axios.get("https://jsonplaceholder.typicode.com/posts")
      .then(response => {
        this.posts = response.data;
      });
  }
};
</script>

<template>
  <div class="container">
    <h1 class="header">Posts</h1>
    <ul class="post-list">
      <li v-for="post in posts" :key="post.id" class="post-item">
        <h2 class="post-title">{{ post.title }}</h2>
        <p class="post-body">{{ post.body }}</p>
      </li>
    </ul>
  </div>
</template>

<style scoped>
/* General Styling */
.container {
  max-width: 800px;
  margin: 0 auto;
  font-family: Arial, sans-serif;
  padding: 20px;
  background-color: #f9f9f9;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

/* Header Styling */
.header {
  text-align: center;
  color: #2c3e50;
  font-size: 2.5em;
  margin-bottom: 20px;
}

/* Post List Styling */
.post-list {
  list-style: none;
  padding: 0;
}

.post-item {
  margin-bottom: 20px;
  padding: 20px;
  background-color: #fff;
  border: 1px solid #ddd;
  border-radius: 8px;
  transition: box-shadow 0.3s ease;
}

.post-item:hover {
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

/* Post Title Styling */
.post-title {
  color: #34495e;
  font-size: 1.5em;
  margin-bottom: 10px;
}

/* Post Body Styling */
.post-body {
  color: #7f8c8d;
  line-height: 1.6;
}
</style>
