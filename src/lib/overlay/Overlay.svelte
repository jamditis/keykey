<script lang="ts">
  // Placeholder — will listen for key events in Task 5
</script>

<div class="overlay-container">
  <div class="debug-label">KeyKey overlay active</div>
</div>

<style>
  .overlay-container {
    position: fixed;
    bottom: 32px;
    right: 32px;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 4px;
  }

  .debug-label {
    background: rgba(0, 0, 0, 0.7);
    color: #fff;
    padding: 8px 16px;
    border-radius: 6px;
    font-family: system-ui, sans-serif;
    font-size: 14px;
  }
</style>
