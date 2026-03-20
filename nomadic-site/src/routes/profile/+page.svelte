<script lang="ts">
  import { onMount } from 'svelte';
  import { auth } from '$lib/auth';
  
  let loading = true;
  let saving = false;
  let message = '';
  let messageType: 'success' | 'error' = 'success';
  
  // Form fields
  let firstName = '';
  let lastName = '';
  let locationCity = '';
  let locationCountry = '';
  let avatarUrl = '';
  
  // Password change
  let showPasswordForm = false;
  let currentPassword = '';
  let newPassword = '';
  let confirmPassword = '';
  let passwordError = '';
  let passwordSuccess = false;
  
  // Location validation
  let validatingLocation = false;
  let locationError = '';
  
  $: user = $auth.user;
  
  onMount(async () => {
    await auth.init();
    if (!user) {
      window.location.href = '/';
      return;
    }
    
    firstName = user?.first_name || '';
    lastName = user?.last_name || '';
    locationCity = user?.location_city || '';
    locationCountry = user?.location_country || '';
    avatarUrl = user?.avatar_url || '';
    loading = false;
  });
  
  async function saveProfile() {
    saving = true;
    message = '';
    
    try {
      await auth.updateProfile({
        first_name: firstName,
        last_name: lastName,
        location_city: locationCity,
        location_country: locationCountry,
        avatar_url: avatarUrl || undefined
      });
      message = 'Profile updated successfully!';
      messageType = 'success';
    } catch (e: any) {
      message = e.message;
      messageType = 'error';
    } finally {
      saving = false;
    }
  }
  
  async function validateLocation() {
    if (!locationCity.trim() || !locationCountry.trim()) {
      locationError = 'Please enter both city and country';
      return;
    }
    
    validatingLocation = true;
    locationError = '';
    
    try {
      const result = await auth.validateLocation(locationCity, locationCountry);
      
      if (result.valid) {
        locationCity = result.city || locationCity;
        locationCountry = result.country || locationCountry;
        message = `Location validated: ${result.city}, ${result.country}`;
        messageType = 'success';
      } else {
        locationError = result.message;
      }
    } catch (e: any) {
      locationError = e.message;
    } finally {
      validatingLocation = false;
    }
  }
  
  async function changePassword() {
    passwordError = '';
    passwordSuccess = false;
    
    if (newPassword !== confirmPassword) {
      passwordError = 'Passwords do not match';
      return;
    }
    
    if (newPassword.length < 6) {
      passwordError = 'Password must be at least 6 characters';
      return;
    }
    
    try {
      await auth.changePassword(currentPassword, newPassword);
      passwordSuccess = true;
      currentPassword = '';
      newPassword = '';
      confirmPassword = '';
      showPasswordForm = false;
    } catch (e: any) {
      passwordError = e.message;
    }
  }
</script>

<svelte:head>
  <title>Profile - NomadHub</title>
</svelte:head>

<div class="profile-page">
  <header>
    <a href="/" class="back-link">← Back to Home</a>
    <h1>Your Profile</h1>
    <a href="/friends" class="friends-link">👥 Friends</a>
  </header>
  
  {#if loading}
    <div class="loading">Loading...</div>
  {:else}
    <div class="profile-grid">
      <!-- Profile Info -->
      <section class="card">
        <h2>👤 Personal Information</h2>
        
        <div class="form-group">
          <label for="email">Email</label>
          <input type="email" id="email" value={user?.email || ''} disabled />
          <span class="hint">Email cannot be changed</span>
        </div>
        
        <div class="form-row">
          <div class="form-group">
            <label for="firstName">First Name</label>
            <input type="text" id="firstName" bind:value={firstName} required />
          </div>
          
          <div class="form-group">
            <label for="lastName">Last Name</label>
            <input type="text" id="lastName" bind:value={lastName} required />
          </div>
        </div>
        
        <div class="form-group">
          <label for="avatar">Avatar URL</label>
          <input type="url" id="avatar" bind:value={avatarUrl} placeholder="https://..." />
        </div>
        
        {#if avatarUrl}
          <div class="avatar-preview">
            <img src={avatarUrl} alt="Avatar preview" />
          </div>
        {/if}
        
        <button class="btn-primary" on:click={saveProfile} disabled={saving}>
          {saving ? 'Saving...' : 'Save Changes'}
        </button>
      </section>
      
      <!-- Location -->
      <section class="card">
        <h2>📍 Current Location</h2>
        <p class="card-desc">Where are you currently based?</p>
        
        <div class="form-row">
          <div class="form-group">
            <label for="city">City</label>
            <input type="text" id="city" bind:value={locationCity} placeholder="e.g., Berlin" />
          </div>
          
          <div class="form-group">
            <label for="country">Country</label>
            <input type="text" id="country" bind:value={locationCountry} placeholder="e.g., Germany" />
          </div>
        </div>
        
        {#if locationError}
          <p class="error">{locationError}</p>
        {/if}
        
        <button class="btn-secondary" on:click={validateLocation} disabled={validatingLocation}>
          {validatingLocation ? 'Validating...' : 'Validate Location'}
        </button>
        
        <p class="hint">We use OpenStreetMap to validate your location.</p>
      </section>
      
      <!-- Password -->
      <section class="card">
        <h2>🔐 Change Password</h2>
        
        {#if !showPasswordForm}
          <button class="btn-secondary" on:click={() => showPasswordForm = true}>
            Change Password
          </button>
        {:else}
          <div class="password-form">
            <div class="form-group">
              <label for="currentPassword">Current Password</label>
              <input type="password" id="currentPassword" bind:value={currentPassword} />
            </div>
            
            <div class="form-group">
              <label for="newPassword">New Password</label>
              <input type="password" id="newPassword" bind:value={newPassword} />
            </div>
            
            <div class="form-group">
              <label for="confirmPassword">Confirm New Password</label>
              <input type="password" id="confirmPassword" bind:value={confirmPassword} />
            </div>
            
            {#if passwordError}
              <p class="error">{passwordError}</p>
            {/if}
            
            {#if passwordSuccess}
              <p class="success">Password changed successfully!</p>
            {/if}
            
            <div class="btn-row">
              <button class="btn-primary" on:click={changePassword}>Update Password</button>
              <button class="btn-text" on:click={() => showPasswordForm = false}>Cancel</button>
            </div>
          </div>
        {/if}
      </section>
      
      <!-- Message -->
      {#if message}
        <div class="message" class:success={messageType === 'success'} class:error={messageType === 'error'}>
          {message}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .profile-page {
    min-height: 100vh;
    background: #f5f5f7;
    padding: 2rem;
  }
  
  header {
    max-width: 800px;
    margin: 0 auto 2rem;
  }
  
  .back-link {
    color: #0071e3;
    text-decoration: none;
    font-size: 0.9rem;
  }
  
  .back-link:hover {
    text-decoration: underline;
  }
  
  .friends-link {
    color: #0071e3;
    text-decoration: none;
    font-size: 0.9rem;
  }
  
  .friends-link:hover {
    text-decoration: underline;
  }
  
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  h1 {
    margin: 0.5rem 0 0;
    color: #1d1d1f;
  }
  
  .loading {
    text-align: center;
    padding: 3rem;
    color: #6e6e73;
  }
  
  .profile-grid {
    max-width: 800px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .card {
    background: #ffffff;
    border-radius: 16px;
    padding: 1.5rem;
    border: 1px solid #e5e5e7;
  }
  
  .card h2 {
    margin: 0 0 0.5rem;
    font-size: 1.25rem;
    color: #1d1d1f;
  }
  
  .card-desc {
    margin: 0 0 1rem;
    color: #6e6e73;
    font-size: 0.9rem;
  }
  
  .form-group {
    margin-bottom: 1rem;
  }
  
  .form-row {
    display: flex;
    gap: 1rem;
  }
  
  .form-row .form-group {
    flex: 1;
  }
  
  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #1d1d1f;
    font-size: 0.9rem;
  }
  
  input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid #d2d2d7;
    border-radius: 8px;
    font-size: 1rem;
    box-sizing: border-box;
    transition: border-color 0.2s, box-shadow 0.2s;
  }
  
  input:focus {
    outline: none;
    border-color: #0071e3;
    box-shadow: 0 0 0 3px rgba(0,113,227,0.1);
  }
  
  input:disabled {
    background: #f5f5f7;
    color: #6e6e73;
  }
  
  .hint {
    display: block;
    margin-top: 0.25rem;
    font-size: 0.8rem;
    color: #86868b;
  }
  
  .avatar-preview {
    margin-bottom: 1rem;
  }
  
  .avatar-preview img {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    object-fit: cover;
    border: 2px solid #e5e5e7;
  }
  
  .btn-primary, .btn-secondary {
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.2s;
    border: none;
  }
  
  .btn-primary {
    background: #0071e3;
    color: white;
  }
  
  .btn-primary:hover:not(:disabled) {
    background: #0077ed;
  }
  
  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .btn-secondary {
    background: transparent;
    color: #0071e3;
    border: 1px solid #0071e3;
  }
  
  .btn-secondary:hover:not(:disabled) {
    background: rgba(0,113,227,0.1);
  }
  
  .btn-text {
    background: none;
    border: none;
    color: #6e6e73;
    cursor: pointer;
    padding: 0.75rem;
  }
  
  .btn-text:hover {
    color: #1d1d1f;
  }
  
  .btn-row {
    display: flex;
    gap: 1rem;
    align-items: center;
  }
  
  .password-form {
    margin-top: 1rem;
  }
  
  .error {
    color: #ff3b30;
    font-size: 0.9rem;
    margin: 0 0 1rem;
  }
  
  .success {
    color: #34c759;
    font-size: 0.9rem;
    margin: 0 0 1rem;
  }
  
  .message {
    padding: 1rem;
    border-radius: 8px;
    text-align: center;
  }
  
  .message.success {
    background: rgba(52, 199, 89, 0.1);
    color: #34c759;
  }
  
  .message.error {
    background: rgba(255, 59, 48, 0.1);
    color: #ff3b30;
  }
</style>
