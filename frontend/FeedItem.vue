<template>
  <div class="feed-item">
    <div class="author">
      <img src="http://i.pravatar.cc/60"/>
    </div>
    <div class="content">
      <div class="header">
        <span class="name">Richard Ramsden</span>
        <span class="time">13h</span>
      </div>
      <div class="body">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse a faucibus magna. Nulla egestas, nibh non congue posuere, enim diam egestas ante, eget volutpat augue justo id ipsum. Vestibulum facilisis dignissim velit, a suscipit ante. Nulla convallis sed lectus sed tempor. Vestibulum ligula urna, sodales maximus nis
      </div>
      <div class="actions">
      </div>
    </div>
  </div>
</template>

<style lang="scss">
.feed-item {
  padding: 1rem 0;
  display: flex;
  border-bottom: 1px solid #dadada;
}

.feed-item .author {
  img {
    border-radius: 50%;
    padding: 0 1rem;
  }    
}

.feed-item .header {
  margin-bottom: 0.5rem;

  .name {
    font-weight: bold;
    margin-right: 0.2rem;
  }
  .time {
    margin-left: 0.2rem;

    &:before {
      content: "\00b7";
      margin-right: 0.2rem;
    }
  }
}
</style>
