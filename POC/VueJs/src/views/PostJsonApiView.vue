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

</style>