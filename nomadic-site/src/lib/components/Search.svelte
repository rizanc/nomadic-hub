<script lang="ts">
  import { onMount } from 'svelte';
  
  let searchQuery = '';
  let results: any = { destinations: [], visas: [], blog: [] };
  let searching = false;
  let showResults = false;
  
  import { API } from '$lib';
  
  async function search() {
    if (!searchQuery.trim()) {
      showResults = false;
      return;
    }
    searching = true;
    showResults = true;
    
    try {
      const res = await fetch(`${API}/api/search?q=${encodeURIComponent(searchQuery)}`);
      results = await res.json();
    } catch (e) {
      console.error(e);
    } finally {
      searching = false;
    }
  }
  
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      search();
    }
  }
  
  function clearSearch() {
    searchQuery = '';
    showResults = false;
  }
</script>

<div class="search-container">
  <div class="search-box">
    <input 
      type="text" 
      bind:value={searchQuery} 
      placeholder="Search destinations, visas, articles..."
      on:keydown={handleKeydown}
      on:blur={() => setTimeout(() => showResults = false, 200)}
    />
    <button on:click={search} disabled={searching}>
      {searching ? '...' : '🔍'}
    </button>
  </div>
  
  {#if showResults}
    <div class="results-dropdown">
      {#if results.destinations?.length > 0}
        <div class="result-section">
          <h4>📍 Destinations</h4>
          {#each results.destinations as dest}
            <a href="/destination/{dest.id}" class="result-item" on:click={clearSearch}>
              <img src={dest.image} alt={dest.name} />
              <div>
                <span class="name">{dest.name}</span>
                <span class="meta">{dest.country} • Score: {dest.score}</span>
              </div>
            </a>
          {/each}
        </div>
      {/if}
      
      {#if results.visas?.length > 0}
        <div class="result-section">
          <h4>🛂 Visas</h4>
          {#each results.visas as visa}
            <div class="result-item">
              <span class="flag">{visa.flag}</span>
              <div>
                <span class="name">{visa.country}</span>
                <span class="meta">{visa.max_stay}</span>
              </div>
            </div>
          {/each}
        </div>
      {/if}
      
      {#if results.blog?.length > 0}
        <div class="result-section">
          <h4>📝 Articles</h4>
          {#each results.blog as post}
            <a href="/blog/{post.id}" class="result-item" on:click={clearSearch}>
              <div>
                <span class="name">{post.title}</span>
                <span class="meta">{post.category}</span>
              </div>
            </a>
          {/each}
        </div>
      {/if}
      
      {#if !results.destinations?.length && !results.visas?.length && !results.blog?.length}
        <div class="no-results">No results found</div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .search-container {
    position: relative;
    flex: 1;
    max-width: 400px;
    margin: 0 1rem;
  }
  
  .search-box {
    display: flex;
    gap: 0.5rem;
  }
  
  .search-box input {
    flex: 1;
    padding: 0.5rem 1rem;
    border: 1px solid #d2d2d7;
    border-radius: 20px;
    font-size: 0.9rem;
    background: #f5f5f7;
  }
  
  .search-box input:focus {
    outline: none;
    border-color: #0071e3;
    background: #fff;
  }
  
  .search-box button {
    padding: 0.5rem 1rem;
    border: none;
    background: #0071e3;
    color: white;
    border-radius: 20px;
    cursor: pointer;
  }
  
  .results-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: white;
    border-radius: 12px;
    box-shadow: 0 8px 24px rgba(0,0,0,0.15);
    max-height: 400px;
    overflow-y: auto;
    z-index: 1000;
    margin-top: 0.5rem;
  }
  
  .result-section {
    padding: 0.5rem 0;
    border-bottom: 1px solid #f0f0f0;
  }
  
  .result-section:last-child {
    border-bottom: none;
  }
  
  .result-section h4 {
    margin: 0;
    padding: 0.5rem 1rem;
    font-size: 0.75rem;
    color: #6e6e73;
    text-transform: uppercase;
  }
  
  .result-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 1rem;
    text-decoration: none;
    color: inherit;
    cursor: pointer;
  }
  
  .result-item:hover {
    background: #f5f5f7;
  }
  
  .result-item img {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    object-fit: cover;
  }
  
  .result-item .name {
    display: block;
    font-weight: 500;
  }
  
  .result-item .meta {
    display: block;
    font-size: 0.8rem;
    color: #6e6e73;
  }
  
  .flag {
    font-size: 1.5rem;
  }
  
  .no-results {
    padding: 2rem;
    text-align: center;
    color: #6e6e73;
  }
</style>
