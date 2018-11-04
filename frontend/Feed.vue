<template>
  <div class="feed">
    <FeedItem></FeedItem>
    <FeedItem></FeedItem>
    <FeedItem></FeedItem>
  </div>
</template>

<script>
import FeedItem from './FeedItem.vue'

export default {
  components: { FeedItem }
}
</script>

<style lang="scss">
.feed {
  background: white;
}
</style>
