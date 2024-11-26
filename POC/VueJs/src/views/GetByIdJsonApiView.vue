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