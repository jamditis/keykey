<script lang="ts">
  const year = new Date().getFullYear();
</script>

<div class="panel">
  <h2 class="panel-title">About</h2>

  <div class="hero">
    <div class="logo-mark">KK</div>
    <div class="hero-text">
      <h1 class="app-name">KeyKey</h1>
      <span class="version">v0.1.0</span>
    </div>
  </div>

  <p class="tagline">A lightweight keystroke visualizer for Windows. Built for streamers, presenters, and anyone who wants to show what they're typing.</p>

  <div class="info-block">
    <div class="info-row">
      <span class="info-key">License</span>
      <span class="info-val">MIT</span>
    </div>
    <div class="info-row">
      <span class="info-key">Built with</span>
      <span class="info-val">Tauri 2, SvelteKit 5, Rust</span>
    </div>
    <div class="info-row">
      <span class="info-key">Platform</span>
      <span class="info-val">Windows 10 / 11</span>
    </div>
  </div>

  <div class="license-box">
    <p class="license-text">
      Copyright {year} KeyKey contributors.<br />
      Permission is hereby granted, free of charge, to any person obtaining a copy of this software
      and associated documentation files, to deal in the Software without restriction, including
      without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
      and/or sell copies of the Software.
    </p>
  </div>
</div>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .panel-title {
    font-family: 'DM Sans', 'Segoe UI', sans-serif;
    font-size: 18px;
    font-weight: 600;
    color: #e0e0e0;
    margin: 0 0 4px 0;
    padding-bottom: 12px;
    border-bottom: 1px solid #2a2a4a;
  }

  .hero {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .logo-mark {
    width: 52px;
    height: 52px;
    background: linear-gradient(135deg, #6655dd, #9944cc);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: 'DM Sans', 'Segoe UI', sans-serif;
    font-size: 18px;
    font-weight: 800;
    color: #fff;
    letter-spacing: -0.05em;
    flex-shrink: 0;
    box-shadow: 0 4px 16px rgba(102, 85, 221, 0.4);
  }

  .hero-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .app-name {
    font-family: 'DM Sans', 'Segoe UI', sans-serif;
    font-size: 24px;
    font-weight: 700;
    color: #e0e0e0;
    margin: 0;
    letter-spacing: -0.02em;
  }

  .version {
    font-size: 12px;
    color: #6655dd;
    font-family: 'Cascadia Code', 'Consolas', monospace;
    background: #2a2a4a;
    border-radius: 4px;
    padding: 1px 7px;
    align-self: flex-start;
  }

  .tagline {
    font-size: 13px;
    color: #8888aa;
    line-height: 1.6;
    margin: 0;
  }

  .info-block {
    display: flex;
    flex-direction: column;
    gap: 0;
    background: #1e1e38;
    border: 1px solid #2a2a4a;
    border-radius: 8px;
    overflow: hidden;
  }

  .info-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 14px;
    border-bottom: 1px solid #2a2a4a;
  }

  .info-row:last-child {
    border-bottom: none;
  }

  .info-key {
    font-size: 12px;
    font-weight: 500;
    color: #6666aa;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    min-width: 90px;
    flex-shrink: 0;
  }

  .info-val {
    font-size: 13px;
    color: #c0c0dd;
  }

  .license-box {
    background: #141428;
    border: 1px solid #2a2a4a;
    border-radius: 8px;
    padding: 14px;
  }

  .license-text {
    font-size: 11px;
    color: #55557a;
    line-height: 1.7;
    margin: 0;
    font-family: 'Cascadia Code', 'Consolas', monospace;
  }
</style>
