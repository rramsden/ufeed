<template>
  <div id="status-bar">
    <div class="inner">
      <div class="author">
        <img src="http://i.pravatar.cc/30"/>
      </div>
      <div class="post-toggle">
        <span class="message">What are you working on?</span>
      </div>
    </div>
  </div>
</template>

<style lang="scss">
#status-bar {
  background: white;
  border-bottom: 1px solid #dadada;

  .inner {
    padding: 1rem 1rem;
    display: flex;
    align-items: center;
  }

  .author {
    width: 50px;
    img {
      border-radius: 50%;
    }
  }

  .post-toggle {
    border: 1px solid #dadada;
    border-radius: 8px;
    width: 100%;
    height: 40px;
    display: flex;
    align-items: center;

    .message {
      opacity: 0.5;
      margin-left: 1rem;
    }
  }
}
</style>
