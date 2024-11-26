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