<script lang="ts">
  import { onMount } from 'svelte';
  import { auth } from '$lib/auth';
  import { goto } from '$app/navigation';
  
  import { API } from '$lib';
  
  let friends: any[] = $state([]);
  let requests: any[] = $state([]);
  let searchResults: any[] = $state([]);
  let searchQuery = $state('');
  let loading = $state(false);
  let activeTab = $state('friends');
  
  let token = $derived($auth.token);
  let user = $derived($auth.user);
  
  async function loadFriends() {
    if (!token) return;
    loading = true;
    try {
      const res = await fetch(`${API}/api/friends`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(token)
      });
      friends = await res.json();
      
      const reqRes = await fetch(`${API}/api/friends/requests`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(token)
      });
      requests = await reqRes.json();
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }
  
  async function searchUsers() {
    if (!searchQuery.trim()) {
      searchResults = [];
      return;
    }
    try {
      const res = await fetch(`${API}/api/users?q=${encodeURIComponent(searchQuery)}`);
      searchResults = await res.json();
    } catch (e) {
      console.error(e);
    }
  }
  
  async function sendRequest(friendId: number) {
    if (!token) return;
    try {
      await fetch(`${API}/api/friends/add`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ token, friend_id: friendId })
      });
      searchResults = searchResults.map(u => 
        u.id === friendId ? { ...u, requestSent: true } : u
      );
    } catch (e) {
      console.error(e);
    }
  }
  
  async function acceptRequest(friendId: number) {
    if (!token) return;
    try {
      await fetch(`${API}/api/friends/accept`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ token, friend_id: friendId })
      });
      await loadFriends();
    } catch (e) {
      console.error(e);
    }
  }
  
  async function declineRequest(friendId: number) {
    if (!token) return;
    try {
      await fetch(`${API}/api/friends/decline`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ token, friend_id: friendId })
      });
      await loadFriends();
    } catch (e) {
      console.error(e);
    }
  }
  
  async function removeFriend(friendId: number) {
    if (!token) return;
    try {
      await fetch(`${API}/api/friends/remove`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ token, friend_id: friendId })
      });
      await loadFriends();
    } catch (e) {
      console.error(e);
    }
  }
  
  onMount(() => {
    if (!user) {
      goto('/');
      return;
    }
    loadFriends();
  });
</script>

<div class="friends-page">
  <div class="header">
    <a href="/" class="back">← Back</a>
    <h1>👥 Friends</h1>
  </div>
  
  <div class="tabs">
    <button class:active={activeTab === 'friends'} on:click={() => activeTab = 'friends'}>
      Friends ({friends.length})
    </button>
    <button class:active={activeTab === 'requests'} on:click={() => activeTab = 'requests'}>
      Requests {#if requests.length > 0}<span class="badge">{requests.length}</span>{/if}
    </button>
    <button class:active={activeTab === 'search'} on:click={() => activeTab = 'search'}>
      Find Friends
    </button>
  </div>
  
  {#if activeTab === 'friends'}
    <div class="content">
      {#if loading}
        <p class="loading">Loading...</p>
      {:else if friends.length === 0}
        <p class="empty">No friends yet. Search for people to add!</p>
      {:else}
        <div class="friend-grid">
          {#each friends as friend}
            <div class="friend-card">
              {#if friend.avatar_url}
                <img src={friend.avatar_url} alt={friend.first_name} />
              {:else}
                <div class="avatar-placeholder">{friend.first_name?.charAt(0)}</div>
              {/if}
              <div class="info">
                <h3>{friend.first_name} {friend.last_name}</h3>
                {#if friend.location_city || friend.location_country}
                  <p class="location">📍 {friend.location_city}, {friend.location_country}</p>
                {/if}
              </div>
              <button class="remove" on:click={() => removeFriend(friend.id)}>Remove</button>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {:else if activeTab === 'requests'}
    <div class="content">
      {#if requests.length === 0}
        <p class="empty">No pending requests</p>
      {:else}
        <div class="request-list">
          {#each requests as req}
            <div class="request-card">
              {#if req.avatar_url}
                <img src={req.avatar_url} alt={req.first_name} />
              {:else}
                <div class="avatar-placeholder">{req.first_name?.charAt(0)}</div>
              {/if}
              <div class="info">
                <h3>{req.first_name} {req.last_name}</h3>
                {#if req.location_city || req.location_country}
                  <p class="location">📍 {req.location_city}, {req.location_country}</p>
                {/if}
              </div>
              <div class="actions">
                <button class="accept" on:click={() => acceptRequest(req.id)}>Accept</button>
                <button class="decline" on:click={() => declineRequest(req.id)}>Decline</button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {:else if activeTab === 'search'}
    <div class="content">
      <div class="search-box">
        <input 
          type="text" 
          bind:value={searchQuery} 
          placeholder="Search by name or email..."
          on:input={searchUsers}
        />
      </div>
      
      {#if searchResults.length > 0}
        <div class="search-results">
          {#each searchResults as result}
            <div class="result-card">
              {#if result.avatar_url}
                <img src={result.avatar_url} alt={result.first_name} />
              {:else}
                <div class="avatar-placeholder">{result.first_name?.charAt(0)}</div>
              {/if}
              <div class="info">
                <h3>{result.first_name} {result.last_name}</h3>
                {#if result.location_city || result.location_country}
                  <p class="location">📍 {result.location_city}, {result.location_country}</p>
                {/if}
              </div>
              {#if result.requestSent}
                <button class="pending" disabled>Request Sent</button>
              {:else}
                <button class="add" on:click={() => sendRequest(result.id)}>Add Friend</button>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .friends-page {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  .header {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 2rem;
  }
  
  .back {
    color: #0071e3;
    text-decoration: none;
    font-size: 0.9rem;
  }
  
  h1 {
    margin: 0;
    font-size: 1.75rem;
  }
  
  .tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
    border-bottom: 1px solid #e5e5e7;
    padding-bottom: 0.5rem;
  }
  
  .tabs button {
    padding: 0.5rem 1rem;
  
  ..5rem 1rem;
    border: none;
    background: none;
    cursor: pointer;
    font-size: 0.95rem;
    border-radius: 8px;
    color: #6e6e73;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  
  .tabs button.active {
    background: #0071e3;
    color: white;
  }
  
  .badge {
    background: #ff3b30;
    color: white;
    padding: 0.1rem 0.4rem;
    border-radius: 10px;
    font-size: 0.75rem;
  }
  
  .content {
    background: white;
    border-radius: 16px;
    padding: 1.5rem;
  }
  
  .loading, .empty {
    text-align: center;
    color: #6e6e73;
    padding: 2rem;
  }
  
  .friend-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 1rem;
  }
  
  .friend-card, .request-card, .result-card {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    border: 1px solid #e5e5e7;
    border-radius: 12px;
  }
  
  .friend-card img, .request-card img, .result-card img {
    width: 50px;
    height: 50px;
    border-radius: 50%;
    object-fit: cover;
  }
  
  .avatar-placeholder {
    width: 50px;
    height: 50px;
    border-radius: 50%;
    background: #0071e3;
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 1.25rem;
  }
  
  .info {
    flex: 1;
  }
  
  .info h3 {
    margin: 0;
    font-size: 1rem;
  }
  
  .location {
    margin: 0.25rem 0 0;
    font-size: 0.8rem;
    color: #6e6e73;
  }
  
  .remove {
    padding: 0.4rem 0.75rem;
    border: 1px solid #ff3b30;
    background: white;
    color: #ff3b30;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.8rem;
  }
  
  .actions {
    display: flex;
    gap: 0.5rem;
  }
  
  .accept {
    padding: 0.4rem 0.75rem;
    border: none;
    background: #34c759;
    color: white;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.8rem;
  }
  
  .decline {
    padding: 0.4rem 0.75rem;
    border: 1px solid #d2d2d7;
    background: white;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.8rem;
  }
  
  .add {
    padding: 0.4rem 0.75rem;
    border: none;
    background: #0071e3;
    color: white;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.8rem;
  }
  
  .pending {
    padding: 0.4rem 0.75rem;
    border: none;
    background: #d2d2d7;
    color: #6e6e73;
    border-radius: 6px;
    font-size: 0.8rem;
  }
  
  .search-box input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid #d2d2d7;
    border-radius: 8px;
    font-size: 1rem;
    box-sizing: border-box;
  }
  
  .search-box input:focus {
    outline: none;
    border-color: #0071e3;
  }
  
  .search-results {
    margin-top: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
</style>
