<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  
  let destination: any = null;
  let weather: any[] = [];
  let pros: any[] = [];
  let cons: any[] = [];
  let reviews: any[] = [];
  let loading = true;
  let error = '';
  let activeTab = 'scores';
  
  const API = 'http://localhost:3000';
  const months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];
  
  onMount(async () => {
    const id = $page.params.id;
    try {
      const [destRes, weatherRes, prosRes, consRes, reviewsRes] = await Promise.all([
        fetch(`${API}/api/destinations/${id}`),
        fetch(`${API}/api/destinations/${id}/weather/live`),
        fetch(`${API}/api/destinations/${id}/pros`),
        fetch(`${API}/api/destinations/${id}/cons`),
        fetch(`${API}/api/destinations/${id}/reviews`)
      ]);
      
      if (!destRes.ok) throw new Error('Destination not found');
      destination = await destRes.json();
      weather = await weatherRes.json();
      pros = await prosRes.json();
      cons = await consRes.json();
      reviews = await reviewsRes.json();
    } catch (e) {
      error = 'Failed to load destination';
    } finally {
      loading = false;
    }
  });
  
  function getScoreColor(score: number): string {
    if (score >= 80) return '#34c759';
    if (score >= 60) return '#ff9500';
    return '#ff3b30';
  }
  
  function getMonthName(m: number): string {
    return months[m - 1] || '';
  }
</script>

<nav>
  <a href="/" class="back">← Back</a>
  <div class="logo">🏝️ NomadHub</div>
  <button class="favorite-btn">♥ Favorite</button>
</nav>

<main>
  {#if loading}
    <div class="loading">Loading...</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else if destination}
    <!-- Hero -->
    <div class="hero">
      <img src={destination.image} alt={destination.name} />
      <div class="hero-overlay">
        <div class="hero-content">
          <div class="location">
            <h1>{destination.name}, {destination.country}</h1>
          </div>
          <div class="score-badge">{destination.nomad_score}/100</div>
        </div>
      </div>
    </div>
    
    <!-- Rating & Reviews -->
    <div class="rating-bar">
      <span class="stars">★★★★★</span>
      <span class="rating">{destination.nomad_score >= 90 ? '4.5' : '4.0'}/5</span>
      <span class="review-count">({reviews.length} reviews)</span>
    </div>
    
    <!-- Tabs -->
    <div class="tabs">
      <button class:active={activeTab === 'scores'} on:click={() => activeTab = 'scores'}>Scores</button>
      <button class:active={activeTab === 'proscons'} on:click={() => activeTab = 'proscons'}>Pros and Cons</button>
      <button class:active={activeTab === 'cost'} on:click={() => activeTab = 'cost'}>Cost of Living</button>
      <button class:active={activeTab === 'weather'} on:click={() => activeTab = 'weather'}>Weather</button>
      <button class:active={activeTab === 'reviews'} on:click={() => activeTab = 'reviews'}>Reviews</button>
    </div>
    
    <!-- Tab Content -->
    <div class="tab-content">
      {#if activeTab === 'scores'}
        <!-- Scores Tab - Progress Bars -->
        <div class="scores-grid">
          <div class="score-card">
            <div class="score-header">
              <span class="score-label">Fun</span>
              <span class="score-value">{destination.fun_score || 70}%</span>
            </div>
            <div class="progress-bar">
              <div class="progress-fill" style="width: {destination.fun_score || 70}%; background: {getScoreColor(destination.fun_score || 70)}"></div>
            </div>
          </div>
          
          <div class="score-card">
            <div class="score-header">
              <span class="score-label">Internet</span>
              <span class="score-value">{destination.internet}%</span>
            </div>
            <div class="progress-bar">
              <div class="progress-fill" style="width: {destination.internet}%; background: {getScoreColor(destination.internet)}"></div>
            </div>
          </div>
          
          <div class="score-card">
            <div class="score-header">
              <span class="score-label">Air Quality</span>
              <span class="score-value">{destination.air_quality_score || 80}%</span>
            </div>
            <div class="progress-bar">
              <div class="progress-fill" style="width: {destination.air_quality_score || 80}%; background: {getScoreColor(destination.air_quality_score || 80)}"></div>
            </div>
          </div>
          
          <div class="score-card">
            <div class="score-header">
              <span class="score-label">Walkability</span>
              <span class="score-value">{destination.walkability_score || 70}%</span>
            </div>
            <div class="progress-bar">
              <div class="progress-fill" style="width: {destination.walkability_score || 70}%; background: {getScoreColor(destination.walkability_score || 70)}"></div>
            </div>
          </div>
          
          <div class="score-card">
            <div class="score-header">
              <span class="score-label">Safety</span>
              <span class="score-value">{destination.safety_score || 80}%</span>
            </div>
            <div class="progress-bar">
              <div class="progress-fill" style="width: {destination.safety_score || 80}%; background: {getScoreColor(destination.safety_score || 80)}"></div>
            </div>
          </div>
        </div>
      
      {:else if activeTab === 'proscons'}
        <!-- Pros and Cons -->
        <div class="pros-cons">
          <div class="pros">
            <h3>✅ Pros</h3>
            {#each pros as pro}
              <div class="item">{pro.label}</div>
            {/each}
          </div>
          <div class="cons">
            <h3>❌ Cons</h3>
            {#each cons as con}
              <div class="item">{con.label}</div>
            {/each}
          </div>
        </div>
      
      {:else if activeTab === 'cost'}
        <!-- Cost of Living -->
        <div class="cost-section">
          <div class="cost-budgets">
            <div class="budget-card">
              <span class="budget-label">Nomad</span>
              <span class="budget-value">${destination.cost_nomad || 2000}</span>
            </div>
            <div class="budget-card">
              <span class="budget-label">Expat</span>
              <span class="budget-value">${destination.cost_expat || 1500}</span>
            </div>
            <div class="budget-card">
              <span class="budget-label">Family</span>
              <span class="budget-value">${destination.cost_family || 4000}</span>
            </div>
            <div class="budget-card">
              <span class="budget-label">Local</span>
              <span class="budget-value">${destination.cost_local || 1000}</span>
            </div>
          </div>
          
          <div class="cost-table">
            <h3>Monthly Costs</h3>
            <div class="table-row">
              <span>🏨 Hotel (monthly)</span>
              <span>${destination.hotel_price || 120}/mo</span>
            </div>
            <div class="table-row">
              <span>🏠 Airbnb (monthly)</span>
              <span>${destination.airbnb_price || 1500}/mo</span>
            </div>
            <div class="table-row">
              <span>🏢 Studio (center)</span>
              <span>${destination.rent_studio || 1000}/mo</span>
            </div>
            <div class="table-row">
              <span>🏠 1BR (center)</span>
              <span>${destination.rent_1br || 1200}/mo</span>
            </div>
            <div class="table-row">
              <span>💻 Coworking</span>
              <span>${destination.coworking_price || 200}/mo</span>
            </div>
            <div class="table-row">
              <span>📱 Mobile Data</span>
              <span>${destination.mobile_data_price || 20}/mo</span>
            </div>
          </div>
          
          <div class="cost-table">
            <h3>Daily Costs</h3>
            <div class="table-row">
              <span>🍽️ Dinner</span>
              <span>${destination.dinner_price || 15}</span>
            </div>
            <div class="table-row">
              <span>☕ Coffee</span>
              <span>${destination.coffee_price || 4}</span>
            </div>
            <div class="table-row">
              <span>🍺 Beer</span>
              <span>${destination.beer_price || 6}</span>
            </div>
            <div class="table-row">
              <span>🚕 Taxi</span>
              <span>${destination.taxi_price || 5}</span>
            </div>
          </div>
        </div>
      
      {:else if activeTab === 'weather'}
        <!-- Weather Grid -->
        <div class="weather-grid">
          <div class="weather-header">
            {#each months as m}
              <div class="weather-col month">{m}</div>
            {/each}
          </div>
          
          <div class="weather-row">
            <div class="weather-label">Feels</div>
            {#each weather as w}
              <div class="weather-col">{w.temp_feels}°</div>
            {/each}
          </div>
          
          <div class="weather-row">
            <div class="weather-label">Real</div>
            {#each weather as w}
              <div class="weather-col">{w.temp_real}°</div>
            {/each}
          </div>
          
          <div class="weather-row">
            <div class="weather-label">Humidity</div>
            {#each weather as w}
              <div class="weather-col" class:humid={w.humidity > 70}>{w.humidity}%</div>
            {/each}
          </div>
          
          <div class="weather-row">
            <div class="weather-label">Rain</div>
            {#each weather as w}
              <div class="weather-col" class:rainy={w.rain_mm > 100}>{w.rain_mm}mm</div>
            {/each}
          </div>
          
          <div class="weather-row">
            <div class="weather-label">UV Index</div>
            {#each weather as w}
              <div class="weather-col" class:sunny={w.uv_index > 6}>{w.uv_index}</div>
            {/each}
          </div>
          
          <div class="weather-row">
            <div class="weather-label">Remote</div>
            {#each weather as w}
              <div class="weather-col">{w.remote_workers.toLocaleString()}</div>
            {/each}
          </div>
        </div>
      
      {:else if activeTab === 'reviews'}
        <!-- Reviews -->
        <div class="reviews">
          {#each reviews as review}
            <div class="review-card">
              <div class="review-header">
                <img src={review.reviewer_avatar || 'https://i.pravatar.cc/150'} alt={review.reviewer_name} class="avatar" />
                <div>
                  <div class="reviewer-name">{review.reviewer_name}</div>
                  <div class="review-rating">{'★'.repeat(Math.round(review.rating))}</div>
                </div>
              </div>
              <p class="review-comment">{review.comment}</p>
              <div class="review-date">{review.visit_date}</div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
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
    position: sticky;
    top: 0;
    z-index: 100;
  }
  
  .back {
    color: #0071e3;
    text-decoration: none;
    font-weight: 500;
  }
  
  .logo {
    font-size: 1.25rem;
    font-weight: 700;
  }
  
  .favorite-btn {
    background: #ff3b30;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-weight: 600;
    cursor: pointer;
  }
  
  main {
    max-width: 100%;
    margin: 0 auto;
    padding-bottom: 2rem;
  }
  
  .loading, .error {
    text-align: center;
    padding: 4rem;
    color: #6e6e73;
  }
  
  .error { color: #ff3b30; }
  
  /* Hero */
  .hero {
    position: relative;
    height: 300px;
    overflow: hidden;
  }
  
  .hero img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  
  .hero-overlay {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    background: linear-gradient(transparent, rgba(0,0,0,0.7));
    padding: 2rem;
  }
  
  .hero-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  h1 {
    margin: 0;
    font-size: 2rem;
    color: #fff;
    text-shadow: 0 2px 4px rgba(0,0,0,0.3);
  }
  
  .score-badge {
    background: #34c759;
    color: #fff;
    font-weight: bold;
    padding: 0.5rem 1rem;
    border-radius: 8px;
    font-size: 1.25rem;
  }
  
  /* Rating Bar */
  .rating-bar {
    background: #ffffff;
    padding: 0.75rem 2rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    border-bottom: 1px solid #e5e5e7;
  }
  
  .stars { color: #ffd60a; }
  .rating { font-weight: 600; }
  .review-count { color: #6e6e73; }
  
  /* Tabs */
  .tabs {
    display: flex;
    background: #ffffff;
    padding: 0 2rem;
    border-bottom: 1px solid #e5e5e7;
    overflow-x: auto;
  }
  
  .tabs button {
    background: none;
    border: none;
    padding: 1rem 1.25rem;
    font-size: 0.95rem;
    font-weight: 500;
    color: #6e6e73;
    cursor: pointer;
    border-bottom: 2px solid transparent;
    white-space: nowrap;
  }
  
  .tabs button.active {
    color: #ff3b30;
    border-bottom-color: #ff3b30;
  }
  
  /* Tab Content */
  .tab-content {
    padding: 1.5rem;
    background: #ffffff;
    margin: 0;
  }
  
  /* Scores Grid */
  .scores-grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 600px;
  }
  
  .score-card {
    background: #f5f5f7;
    border-radius: 10px;
    padding: 0.75rem 1rem;
  }
  
  .score-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.5rem;
  }
  
  .score-label { font-weight: 600; }
  .score-value { font-weight: 700; }
  
  .progress-bar {
    height: 8px;
    background: #e5e5e7;
    border-radius: 4px;
    overflow: hidden;
  }
  
  .progress-fill {
    height: 100%;
    border-radius: 4px;
    transition: width 0.3s ease;
  }
  
  /* Pros & Cons */
  .pros-cons {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
  }
  
  @media (max-width: 600px) {
    .pros-cons { grid-template-columns: 1fr; }
  }
  
  .pros h3, .cons h3 {
    margin: 0 0 1rem;
    font-size: 1.1rem;
  }
  
  .pros h3 { color: #34c759; }
  .cons h3 { color: #ff3b30; }
  
  .pros .item, .cons .item {
    padding: 0.5rem 0;
    border-bottom: 1px solid #f0f0f0;
  }
  
  /* Cost */
  .cost-section {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
  
  .cost-budgets {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 1rem;
  }
  
  @media (max-width: 600px) {
    .cost-budgets { grid-template-columns: repeat(2, 1fr); }
  }
  
  .budget-card {
    background: #f5f5f7;
    border-radius: 10px;
    padding: 1rem;
    text-align: center;
  }
  
  .budget-label {
    display: block;
    font-size: 0.85rem;
    color: #6e6e73;
    margin-bottom: 0.25rem;
  }
  
  .budget-value {
    font-size: 1.5rem;
    font-weight: 700;
    color: #0071e3;
  }
  
  .cost-table {
    background: #f5f5f7;
    border-radius: 10px;
    padding: 1rem;
  }
  
  .cost-table h3 {
    margin: 0 0 1rem;
    font-size: 1rem;
  }
  
  .table-row {
    display: flex;
    justify-content: space-between;
    padding: 0.5rem 0;
    border-bottom: 1px solid #e5e5e7;
  }
  
  .table-row:last-child { border-bottom: none; }
  
  /* Weather Grid */
  .weather-grid {
    overflow-x: auto;
  }
  
  .weather-header, .weather-row {
    display: grid;
    grid-template-columns: 80px repeat(12, 1fr);
    gap: 2px;
  }
  
  .weather-col {
    padding: 0.5rem 0.25rem;
    text-align: center;
    font-size: 0.85rem;
    background: #f5f5f7;
    border-radius: 4px;
  }
  
  .weather-col.month {
    font-weight: 600;
    background: #e5e5e7;
  }
  
  .weather-label {
    padding: 0.5rem;
    font-weight: 600;
    background: transparent;
  }
  
  .weather-col.humid { background: #ff9500; color: white; }
  .weather-col.rainy { background: #0071e3; color: white; }
  .weather-col.sunny { background: #ff3b30; color: white; }
  
  /* Reviews */
  .reviews {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .review-card {
    background: #f5f5f7;
    border-radius: 10px;
    padding: 1rem;
  }
  
  .review-header {
    display: flex;
    gap: 1rem;
    align-items: center;
    margin-bottom: 0.5rem;
  }
  
  .avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
  }
  
  .reviewer-name { font-weight: 600; }
  .review-rating { color: #ffd60a; }
  .review-comment { margin: 0.5rem 0; }
  .review-date { font-size: 0.85rem; color: #6e6e73; }
</style>
