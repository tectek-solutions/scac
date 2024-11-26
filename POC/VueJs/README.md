# Test of Vuejs

This project will show you some basic implementation of API requests in Vue.js.

## Example Code

### GET Request Example

The following code demonstrates how to fetch a list of posts using a GET request in Vue.js:

```vue
<!-- Here is your script -->
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

<!-- Here is your template -->
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

<!-- Here is your style -->
<style scoped>
/* General Styling */
.container {
  max-width: 800px;
  margin: 0 auto;
  font-family: Arial, sans-serif;
  padding: 20px;
  background-color: #f9f9f9;
  border-radius: 8px;
}
</style>
```

### POST request example
``` vue

<script>
import axios from 'axios';

export default {
  data() {
    return {
      posts: []
    }
  },
  mounted() {
    axios.post('https://jsonplaceholder.typicode.com/posts',
    {
      title: 'test_post',
      body: 'test_body',
      userId: 1
    })
    .then(response => {
      this.posts = response.data;
      console.log(response.data);
    });
  }
}
</script>

<template>
  <div class="container">
    <h1 class="header">A new post has been created with</h1>
    <p>title:  {{ this.posts.title }}</p>
    <p>body:  {{ this.posts.body }}</p>
  </div>
</template>

<style>
/* Add your styles here */
</style>
```
### PUT request example
```vue
<script>
import axios from 'axios';

export default {
  data() {
    return {
      posts: []
    }
  },
  mounted() {
    axios.put('https://jsonplaceholder.typicode.com/posts/1',
    {
      id: 1,
      title: 'test_put',
      body: 'test_body',
      userId: 1
    })
    .then(response => {
      this.posts = response.data;
      console.log(response.data);
    });
  }
}
</script>

<template>
  <div class="container">
    <h1 class="header">A post has been updated with</h1>
    <p>id:  {{ this.posts.id }}</p>
    <p>title:  {{ this.posts.title }}</p>
    <p>body:  {{ this.posts.body }}</p>
  </div>
</template>

<style>
/* Add your styles here */
</style>
```
### GET request with id example
``` vue
<script>
    import axios from 'axios';
    export default {
        data() {
            return {
                posts: []
            }
        },
        mounted() {
            axios.get('https://jsonplaceholder.typicode.com/posts/1')
            .then(response => {
                this.posts = response.data;
                console.log(response.data);
            });
        }
    }
</script>

<template>
    <div class="container">
        <h1 class="header">Post with id {{ this.posts.id }}</h1>
        <p>title:  {{ this.posts.title }}</p>
        <p>body:  {{ this.posts.body }}</p>
    </div>
</template>

<style>
</style>
```
### Delete request example
``` vue
<script>
    import axios from 'axios';

    export default {
        data() {
            return {
                posts: []
            }
        },
        mounted() {
            axios.delete('https://jsonplaceholder.typicode.com/posts/1')
            .then(response => {
                this.posts = response.data;
                console.log("----->", response.data);
            });
        }
    }

</script>

<template>
    <div class="container">
        <h1 class="header">The post with the id 1 has been deleted</h1>
    </div>
</template>

<style>
</style>
```

### How it works

The code above demonstrates how to request an api with three different methods: GET, POST, and PUT. The `axios` library is used to make the HTTP requests. In the `mounted` lifecycle hook, the `axios.get`, `axios.post`, and `axios.put` methods are used to fetch data from the API. The response data is then stored in the `posts` array, which is used to render the list of posts in the template.

### How to call

``` vue
<script>
import { createRouter, createWebHistory } from 'vue-router'
import JsonApiView from '@/views/JsonApiView.vue'
import PostJsonApiView from '@/views/PostJsonApiView.vue'
import PutJsonApiView from '@/views/PutJsonApiView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/get',
      name: 'get',
      component: JsonApiView,
    },
    {
      path: '/post',
      name: 'post',
      component: PostJsonApiView
    },
    {
      path: '/put',
      name: 'put',
      component: PutJsonApiView
    }
  ],
})

export default router
</script>
```

This code demonstrates how to create a router in Vue.js and define routes for different API requests. The `createRouter` function is used to create a router instance, and the `createWebHistory` function is used to create a history instance. The `routes` array defines the routes for the different API requests, and the `path` property specifies the URL path for each route. The `component` property specifies the component to render when the route is matched.


### Finally

``` vue
<script setup>
import { RouterLink, RouterView } from 'vue-router'
import JsonApiView from './views/JsonApiView.vue';
</script>

<template>
  <header>
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="125" height="125" />

    <div class="wrapper">

      <nav>
        <RouterLink to="/get">All post</RouterLink>
        <RouterLink to="/post">Try Post data</RouterLink>
        <RouterLink to="/put">Try Put data</RouterLink>
      </nav>
    </div>
  </header>

  <RouterView />
</template>
```

In the App.vue file, the `RouterLink` component is used to create links to the different API request routes. The `RouterView` component is used to render the component associated with the current route. When a link is clicked, the associated component is rendered in the `RouterView` component.