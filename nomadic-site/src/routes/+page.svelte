<script lang="ts">
  import { onMount } from 'svelte';
  import { auth } from '$lib/auth';
  
  let activeTab = $state('home');
  let domainSearch = $state('');
  let visaSearch = $state('');
  let domains: any[] = $state([]);
  let visas: any[] = $state([]);
  let destinations: any[] = $state([]);
  let blogPosts: any[] = $state([]);
  let loadingDomains = $state(false);
  let loadingVisas = $state(false);
  
  import { API } from '$lib';
  
  let user = $derived($auth.user);
  let token = $derived($auth.token);
  
  async function searchDomains() {
    if (!domainSearch.trim()) return;
    loadingDomains = true;
    const res = await fetch(`${API}/api/domains?q=${domainSearch}`);
    domains = await res.json();
    loadingDomains = false;
  }
  
  async function searchVisas() {
    loadingVisas = true;
    const q = visaSearch || 'nomad';
    const res = await fetch(`${API}/api/visas?q=${q}`);
    visas = await res.json();
    loadingVisas = false;
  }
  
  async function loadData() {
    const [dests, posts] = await Promise.all([
      fetch(`${API}/api/destinations`).then(r => r.json()),
      fetch(`${API}/api/blog`).then(r => r.json())
    ]);
    destinations = dests;
    blogPosts = posts;
  }
  
  onMount(() => {
    searchVisas();
    loadData();
  });
</script>



<main>
  {#if activeTab === 'home'}
    <section class="hero">
      <h1>Live Anywhere. Work Everywhere. 🌎</h1>
      <p>Your complete guide to the digital nomad lifestyle.</p>
      
      <div class="quick-stats">
        <div class="stat">
          <span class="num">15+</span>
          <span class="label">Visa Programs</span>
        </div>
        <div class="stat">
          <span class="num">50+</span>
          <span class="label">Destinations</span>
        </div>
        <div class="stat">
          <span class="num">∞</span>
          <span class="label">Possibilities</span>
        </div>
      </div>
    </section>
    
  {:else if activeTab === 'domains'}
    <section>
      <h2>🔍 Find Your Domain</h2>
      <p>Search for the perfect domain for your nomad project.</p>
      
      <div class="search-box">
        <input 
          type="text" 
          bind:value={domainSearch} 
          placeholder="Try: nomad, wander, roam..."
          on:keydown={(e) => e.key === 'Enter' && searchDomains()}
        />
        <button on:click={searchDomains} disabled={loadingDomains}>
          {loadingDomains ? 'Searching...' : 'Search'}
        </button>
      </div>
      
      {#if domains.length > 0}
        <div class="results">
          {#each domains as domain}
            <div class="domain-card" class:available={domain.status === 'available'}>
              <span class="name">{domain.name}</span>
              <span class="status">{domain.status}</span>
              {#if domain.price}
                <span class="price">${domain.price}/yr</span>
              {/if}
            </div>
          {/each}
        </div>
      {:else if domainSearch}
        <p class="empty">No domains found. Try a different search.</p>
      {/if}
    </section>
    
  {:else if activeTab === 'destinations'}
    <section id="destinations">
      <h2>🗺️ Top Destinations</h2>
      <p>Find your next base. Sorted by nomad score.</p>
      
      <div class="destinations-grid">
        {#each destinations.sort((a,b) => b.nomad_score - a.nomad_score) as city}
          <a href="/destination/{city.id}" class="destination-card">
            <div class="city-image">
              <img src={city.image} alt={city.name} />
              <span class="score">{city.nomad_score}</span>
            </div>
            <div class="city-info">
              <h3>{city.name}, {city.country}</h3>
              <div class="stats">
                <span>💰 ${city.cost}/mo</span>
                <span>📶 {city.internet}%</span>
              </div>
            </div>
          </a>
        {/each}
      </div>
    </section>
    
  {:else if activeTab === 'visas'}
    <section id="visas">
      <h2>🛂 Nomad Visa Guide</h2>
      <p>Find the best visa programs for digital nomads.</p>
      
      <div class="search-box">
        <input 
          type="text" 
          bind:value={visaSearch} 
          placeholder="Search countries..."
          on:keydown={(e) => e.key === 'Enter' && searchVisas()}
        />
        <button on:click={searchVisas}>Search</button>
      </div>
      
      {#if loadingVisas}
        <p>Loading...</p>
      {:else}
        <div class="visas-list">
          {#each visas as visa}
            <div class="visa-card">
              <div class="visa-header">
                <span class="flag">{visa.flag}</span>
                <h3>{visa.country}</h3>
                {#if visa.nomadVisa}
                  <span class="badge">Nomad Visa</span>
                {/if}
              </div>
              <div class="visa-details">
                <p><strong>Max stay:</strong> {visa.maxStay}</p>
                <p><strong>Income:</strong> {visa.income}</p>
                <p><strong>Tax:</strong> {visa.tax}</p>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </section>
    
  {:else if activeTab === 'blog'}
    <section id="blog">
      <h2>📝 Latest Articles</h2>
      <p>Tips, guides, and insights for nomads.</p>
      
      <div class="blog-grid">
        {#each blogPosts as post}
          <article class="blog-card">
            <span class="category">{post.category}</span>
            <h3>{post.title}</h3>
            <p>{post.excerpt}</p>
            <div class="meta">
              <span>{post.date}</span>
              <span>{post.readTime} min read</span>
            </div>
          </article>
        {/each}
      </div>
    </section>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: system-ui, -apple-system, sans-serif;
    background: #f5f5f7;
    color: #1d1d1f;
  }
  
  nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
    background: #ffffff;
    border-bottom: 1px solid #e5e5e7;
    box-shadow: 0 1px 3px rgba(0,0,0,0.05);
  }
  
  .logo {
    font-size: 1.5rem;
    font-weight: bold;
    color: #1d1d1f;
  }
  
  .links button {
    background: none;
    border: none;
    color: #6e6e73;
    padding: 0.5rem 1rem;
    cursor: pointer;
    font-size: 1rem;
    transition: color 0.2s;
  }
  
  .links button:hover, .links button.active {
    color: #0071e3;
  }
  
  main {
    max-width: 1000px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  h1 { font-size: 3rem; margin-bottom: 0.5rem; color: #1d1d1f; }
  h2 { font-size: 2rem; margin-bottom: 0.5rem; color: #1d1d1f; }
  p { color: #6e6e73; }
  
  .hero { text-align: center; padding: 4rem 0; }
  
  .quick-stats {
    display: flex;
    justify-content: center;
    gap: 3rem;
    margin-top: 3rem;
  }
  
  .stat { text-align: center; }
  .stat .num { display: block; font-size: 2.5rem; font-weight: bold; color: #0071e3; }
  .stat .label { color: #6e6e73; }
  
  .search-box {
    display: flex;
    gap: 0.5rem;
    margin: 1.5rem 0;
  }
  
  .search-box input {
    flex: 1;
    padding: 0.75rem 1rem;
    border: 1px solid #d2d2d7;
    border-radius: 8px;
    background: #ffffff;
    color: #1d1d1f;
    font-size: 1rem;
  }
  
  .search-box input:focus {
    outline: none;
    border-color: #0071e3;
    box-shadow: 0 0 0 3px rgba(0,113,227,0.1);
  }
  
  .search-box button {
    padding: 0.75rem 1.5rem;
    background: #0071e3;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 1rem;
    transition: background 0.2s;
  }
  
  .search-box button:hover { background: #0077ed; }
  .search-box button:disabled { opacity: 0.6; }
  
  .results { display: grid; gap: 0.5rem; }
  
  .domain-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: #ffffff;
    border-radius: 8px;
    border: 1px solid #e5e5e7;
  }
  
  .domain-card.available { border-color: #34c759; }
  .domain-card .name { flex: 1; font-weight: bold; color: #1d1d1f; }
  .domain-card .status { color: #ff3b30; }
  .domain-card.available .status { color: #34c759; }
  .domain-card .price { color: #6e6e73; }
  
  .destinations-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 1.5rem;
    margin-top: 1.5rem;
  }
  
  .destination-card {
    background: #ffffff;
    border-radius: 16px;
    overflow: hidden;
    border: 1px solid #e5e5e7;
    text-decoration: none;
    color: inherit;
    display: block;
    transition: box-shadow 0.2s, transform 0.2s;
  }
  
  .destination-card:hover {
    box-shadow: 0 8px 24px rgba(0,0,0,0.1);
    transform: translateY(-4px);
  }
  
  .city-image {
    height: 120px;
    position: relative;
    overflow: hidden;
  }
  
  .city-image img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  
  .score {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    background: #34c759;
    color: #fff;
    font-weight: bold;
    padding: 0.25rem 0.5rem;
    border-radius: 6px;
    font-size: 0.85rem;
  }
  
  .city-info { padding: 1rem; }
  .city-info h3 { margin: 0 0 0.5rem; font-size: 1rem; color: #1d1d1f; }
  .stats { display: flex; gap: 1rem; color: #6e6e73; font-size: 0.85rem; }
  
  .visas-list { display: grid; gap: 1rem; margin-top: 1.5rem; }
  
  .visa-card {
    background: #ffffff;
    border-radius: 12px;
    padding: 1.5rem;
    border: 1px solid #e5e5e7;
  }
  
  .visa-header { display: flex; align-items: center; gap: 0.75rem; margin-bottom: 1rem; }
  .visa-header h3 { margin: 0; flex: 1; color: #1d1d1f; }
  .flag { font-size: 1.5rem; }
  .badge { background: #34c759; color: #fff; padding: 0.25rem 0.5rem; border-radius: 6px; font-size: 0.75rem; font-weight: bold; }
  .visa-details p { margin: 0.25rem 0; color: #6e6e73; font-size: 0.9rem; }
  
  .blog-grid { display: grid; gap: 1rem; margin-top: 1.5rem; }
  
  .blog-card {
    background: #ffffff;
    border-radius: 12px;
    padding: 1.5rem;
    border: 1px solid #e5e5e7;
    cursor: pointer;
    transition: box-shadow 0.2s;
  }
  
  .blog-card:hover { box-shadow: 0 4px 12px rgba(0,0,0,0.08); }
  .blog-card .category { color: #0071e3; font-size: 0.8rem; font-weight: bold; text-transform: uppercase; }
  .blog-card h3 { margin: 0.5rem 0; color: #1d1d1f; }
  .blog-card p { color: #6e6e73; font-size: 0.9rem; }
  .blog-card .meta { display: flex; gap: 1rem; margin-top: 1rem; color: #86868b; font-size: 0.8rem; }
  
  .empty { text-align: center; color: #6e6e73; margin-top: 2rem; }
  
  /* Auth */
  .user-section {
    display: flex;
    align-items: center;
  }
  
  .user-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  
  .avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    object-fit: cover;
  }
  
  .avatar-placeholder {
    background: #0071e3;
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
  }
  
  .user-name {
    font-weight: 500;
    color: #1d1d1f;
  }
  
  .login-btn, .logout-btn {
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .login-btn {
    background: #0071e3;
    color: white;
    border: none;
  }
  
  .login-btn:hover {
    background: #0077ed;
  }
  
  .logout-btn {
    background: transparent;
    color: #6e6e73;
    border: 1px solid #d2d2d7;
  }
  
  .logout-btn:hover {
    background: #f5f5f7;
  }
  
  .profile-btn {
    background: transparent;
    color: #0071e3;
    border: 1px solid #0071e3;
  }
  
  .profile-btn:hover {
    background: rgba(0, 113, 227, 0.1);
  }
  
  .name-row {
    display: flex;
    gap: 0.5rem;
  }
  
  .name-row input {
    flex: 1;
  }
  
  /* Modal */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0,0,0,0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }
  
  .modal {
    background: #ffffff;
    padding: 2rem;
    border-radius: 16px;
    width: 90%;
    max-width: 400px;
    box-shadow: 0 20px 60px rgba(0,0,0,0.2);
  }
  
  .modal h2 {
    margin: 0 0 1.5rem;
    text-align: center;
    color: #1d1d1f;
  }
  
  .modal form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .modal input {
    padding: 0.75rem 1rem;
    border: 1px solid #d2d2d7;
    border-radius: 8px;
    font-size: 1rem;
  }
  
  .modal input:focus {
    outline: none;
    border-color: #0071e3;
    box-shadow: 0 0 0 3px rgba(0,113,227,0.1);
  }
  
  .modal button[type="submit"] {
    padding: 0.75rem;
    background: #0071e3;
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    cursor: pointer;
    transition: background 0.2s;
  }
  
  .modal button[type="submit"]:hover:not(:disabled) {
    background: #0077ed;
  }
  
  .modal button[type="submit"]:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .modal .error {
    color: #ff3b30;
    font-size: 0.9rem;
    text-align: center;
    margin: 0;
  }
  
  .switch-mode {
    text-align: center;
    margin-top: 1rem;
    color: #6e6e73;
  }
  
  .switch-mode button {
    background: none;
    border: none;
    color: #0071e3;
    cursor: pointer;
    font-size: inherit;
  }
</style>
