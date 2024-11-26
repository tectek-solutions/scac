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

</style>