<template>
  <div id="app">
    <div class="container">
      <StatusBar></StatusBar>
      <Feed></Feed>
    </div>
  </div>
</template>

<script>
import StatusBar from './StatusBar.vue'
import Feed from './Feed.vue'

export default {
  components: { Feed, StatusBar }
}  

</script>

<style lang="scss">
body {
  font-family: "Helvetica Neue",Helvetica,Arial,sans-serif;
  -webkit-font-smoothing: antialiased;
  font-size: 14px;
  line-height: 20px;
  background: #e6ecf0;
  margin: 0;
}

header {
}

footer {
}

.container {
  width: 600px;
  margin: 0 auto;
}

#feed {
  background: #dadada;
}
</style>
