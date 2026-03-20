<script lang="ts">
  import { onMount } from 'svelte';
  import { auth } from '$lib/auth';
  import Search from '$lib/components/Search.svelte';
  
  let { children } = $props();
  
  let activeTab = $state('home');
  let showAuthModal = $state(false);
  let authMode: 'login' | 'register' = $state('login');
  let authEmail = $state('');
  let authPassword = $state('');
  let authFirstName = $state('');
  let authLastName = $state('');
  let authError = $state('');
  let authLoading = $state(false);
  
  import { API } from '$lib';
  
  let user = $derived($auth.user);
  
  async function handleAuth() {
    authError = '';
    authLoading = true;
    
    try {
      if (authMode === 'login') {
        await auth.login(authEmail, authPassword);
      } else {
        await auth.register(authEmail, authPassword, authFirstName, authLastName);
      }
      showAuthModal = false;
    } catch (e: any) {
      authError = e.message;
    } finally {
      authLoading = false;
    }
  }
  
  function logout() {
    auth.logout();
  }
  
  onMount(() => {
    auth.init();
  });
</script>

<nav>
  <div class="nav-left">
    <a href="/" class="logo">🏝️ NomadHub</a>
    <Search />
  </div>
  
  <div class="nav-links">
    <a href="/" class:active={activeTab === 'home'}>Home</a>
    <a href="/#destinations" class:active={activeTab === 'destinations'}>Destinations</a>
    <a href="/#visas" class:active={activeTab === 'visas'}>Visas</a>
    <a href="/#blog" class:active={activeTab === 'blog'}>Blog</a>
  </div>
  
  <div class="user-section">
    {#if user}
      <div class="user-info">
        {#if user.avatar_url}
          <img src={user.avatar_url} alt={user.first_name} class="avatar" />
        {:else}
          <div class="avatar avatar-placeholder">{user.first_name?.charAt(0).toUpperCase() || '?'}</div>
        {/if}
        <span class="user-name">{user.first_name}</span>
        <a href="/profile" class="nav-btn">Profile</a>
        <button class="nav-btn logout" on:click={logout}>Logout</button>
      </div>
    {:else}
      <button class="login-btn" on:click={() => { showAuthModal = true; authMode = 'login'; }}>
        Sign In
      </button>
    {/if}
  </div>
</nav>

{#if showAuthModal}
  <div class="modal-overlay" on:click={() => showAuthModal = false}>
    <div class="modal" on:click|stopPropagation>
      <h2>{authMode === 'login' ? 'Welcome Back' : 'Create Account'}</h2>
      
      <form on:submit|preventDefault={handleAuth}>
        {#if authMode === 'register'}
          <div class="name-row">
            <input type="text" bind:value={authFirstName} placeholder="First name" required />
            <input type="text" bind:value={authLastName} placeholder="Last name" required />
          </div>
        {/if}
        <input type="email" bind:value={authEmail} placeholder="Email" required />
        <input type="password" bind:value={authPassword} placeholder="Password" required />
        
        {#if authError}
          <p class="error">{authError}</p>
        {/if}
        
        <button type="submit" disabled={authLoading}>
          {authLoading ? 'Loading...' : (authMode === 'login' ? 'Sign In' : 'Create Account')}
        </button>
      </form>
      
      <p class="switch-mode">
        {#if authMode === 'login'}
          Don't have an account? <button on:click={() => authMode = 'register'}>Sign up</button>
        {:else}
          Already have an account? <button on:click={() => authMode = 'login'}>Sign in</button>
        {/if}
      </p>
    </div>
  </div>
{/if}

<main>
  {@render children()}
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
    padding: 0.75rem 2rem;
    background: #ffffff;
    border-bottom: 1px solid #e5e5e7;
    box-shadow: 0 1px 3px rgba(0,0,0,0.05);
    position: sticky;
    top: 0;
    z-index: 100;
  }
  
  .nav-left {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex: 1;
  }
  
  .logo {
    font-size: 1.25rem;
    font-weight: bold;
    color: #1d1d1f;
    text-decoration: none;
  }
  
  .nav-links {
    display: flex;
    gap: 0.5rem;
  }
  
  .nav-links a {
    padding: 0.5rem 1rem;
    color: #6e6e73;
    text-decoration: none;
    border-radius: 8px;
    transition: all 0.2s;
  }
  
  .nav-links a:hover, .nav-links a.active {
    color: #0071e3;
    background: rgba(0,113,227,0.1);
  }
  
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
  
  .nav-btn {
    padding: 0.4rem 0.75rem;
    border-radius: 6px;
    font-size: 0.85rem;
    cursor: pointer;
    text-decoration: none;
    border: none;
    background: transparent;
    color: #6e6e73;
  }
  
  .nav-btn:hover {
    background: #f5f5f7;
  }
  
  .nav-btn.logout {
    color: #ff3b30;
  }
  
  .login-btn {
    padding: 0.5rem 1rem;
    background: #0071e3;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
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
  }
  
  .modal button[type="submit"] {
    padding: 0.75rem;
    background: #0071e3;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
  }
  
  .modal button[type="submit"]:disabled {
    opacity: 0.6;
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
  }
  
  .name-row {
    display: flex;
    gap: 0.5rem;
  }
  
  .name-row input {
    flex: 1;
  }
  
  main {
    min-height: calc(100vh - 60px);
  }
</style>
