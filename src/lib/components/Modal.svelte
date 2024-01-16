<script lang="ts">
  export let header = "Test";
  export let width = 20;
  export let height = 20;
  export let open = false;
  export let onClosed = () => {};

  const clickHandler = () => {
    open = false;
    onClosed();
  }

  const childClickHandler = (e: MouseEvent) => {
    e.stopPropagation();
  }
</script>

<button class="modal reset-button" style="display: {open ? 'flex' : 'none'}" on:click={clickHandler}>
  <button class="modal-content reset-button" style="width: {width}em; height: {height}em" on:click={childClickHandler}>
    <span>{header}</span>
    <div class="modal-slot">
      <slot />
    </div>
  </button>
</button>

<style>
  .modal {
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0,0,0,0.5);
    backdrop-filter: blur(5px);
    z-index: 100;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .modal-content {
    background: var(--color-primary);
    border-radius: 0.5em;
    padding: 1em;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: flex-start;
  }

  .modal-content span {
    padding-bottom: .5em;
    border-bottom: 1px solid var(--color-secondary);
    width: 100%;
    text-align: center;
  }

  .modal-slot {
    height: 100%;
    width: 100%;
  }
</style>