<script lang="ts">
  let tab = 0;
  let tabs = ["General", "Locations", "Appearance", "About"];
</script>

<div class="settings">
  <div class="tabs">
    {#each tabs as t, i}
      <button class="tab-button reset-button {tab === i ? 'selected' : ''}" on:click={() => tab = i}>
        <span>{t}</span>
      </button>
    {/each}
  </div>
</div>

<style>
  .settings {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: row;
    color: var(--color-text);
  }

  .tabs {
    display: flex;
    flex-direction: column;
    width: 10em;
    height: 100%;
    background-color: var(--color-primary-dark);
    border-right: 1px solid var(--color-secondary-dark);
    margin-top: .5em;
    margin-left: 1em;
  }

  .tab-button {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 2em;
  }

  .selected {
    border-bottom: 1px solid var(--color-secondary);
  }
</style>