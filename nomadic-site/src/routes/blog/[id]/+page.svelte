<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  
  let post: any = null;
  let loading = true;
  let error = '';
  
  import { API } from '$lib';
  
  onMount(async () => {
    const id = $page.params.id;
    try {
      const res = await fetch(`${API}/api/blog/${id}`);
      if (!res.ok) throw new Error('Post not found');
      post = await res.json();
    } catch (e) {
      error = 'Failed to load article';
    } finally {
      loading = false;
    }
  });
</script>

<svelte:head>
  <title>{post?.title || 'Article'} - NomadHub</title>
</svelte:head>

<div class="article-page">
  <a href="/" class="back-link">← Back to Home</a>
  
  {#if loading}
    <div class="loading">Loading...</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else if post}
    <article>
      {#if post.image}
        <img src={post.image} alt={post.title} class="hero-image" />
      {/if}
      
      <div class="content">
        <span class="category">{post.category}</span>
        <h1>{post.title}</h1>
        
        <div class="meta">
          <span>{post.date}</span>
          <span>•</span>
          <span>{post.read_time} min read</span>
        </div>
        
        <div class="body">
          {#if post.content}
            {#each post.content.split('\n\n') as paragraph}
              {#if paragraph.startsWith('## ')}
                <h2>{paragraph.replace('## ', '')}</h2>
              {:else if paragraph.startsWith('### ')}
                <h3>{paragraph.replace('### ', '')}</h3>
              {:else if paragraph.startsWith('- ')}
                <li>{paragraph.replace('- ', '')}</li>
              {:else}
                <p>{paragraph}</p>
              {/if}
            {/each}
          {:else}
            <p>{post.excerpt}</p>
          {/if}
        </div>
        
        <div class="share">
          <span>Share this article:</span>
          <button>Twitter</button>
          <button>Facebook</button>
          <button>LinkedIn</button>
        </div>
      </div>
    </article>
  {/if}
</div>

<style>
  .article-page {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  .back-link {
    color: #0071e3;
    text-decoration: none;
    font-size: 0.9rem;
  }
  
  .back-link:hover {
    text-decoration: underline;
  }
  
  .loading, .error {
    text-align: center;
    padding: 4rem;
    color: #6e6e73;
  }
  
  .error { color: #ff3b30; }
  
  article {
    margin-top: 2rem;
  }
  
  .hero-image {
    width: 100%;
    height: 300px;
    object-fit: cover;
    border-radius: 16px;
    margin-bottom: 2rem;
  }
  
  .content {
    background: white;
    padding: 2rem;
    border-radius: 16px;
  }
  
  .category {
    display: inline-block;
    background: #0071e3;
    color: white;
    padding: 0.25rem 0.75rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
  }
  
  h1 {
    margin: 1rem 0 0.5rem;
    font-size: 2.5rem;
    color: #1d1d1f;
  }
  
  .meta {
    color: #6e6e73;
    font-size: 0.9rem;
    margin-bottom: 2rem;
  }
  
  .meta span {
    margin-right: 0.5rem;
  }
  
  .body {
    line-height: 1.8;
    color: #1d1d1f;
  }
  
  .body p {
    margin: 1rem 0;
  }
  
  .body h2 {
    margin: 2rem 0 1rem;
    font-size: 1.5rem;
  }
  
  .body h3 {
    margin: 1.5rem 0 0.75rem;
    font-size: 1.25rem;
  }
  
  .body li {
    margin: 0.5rem 0;
    padding-left: 1rem;
  }
  
  .share {
    margin-top: 3rem;
    padding-top: 1.5rem;
    border-top: 1px solid #e5e5e7;
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .share span {
    color: #6e6e73;
  }
  
  .share button {
    padding: 0.5rem 1rem;
    border: 1px solid #d2d2d7;
    border-radius: 6px;
    background: white;
    cursor: pointer;
    font-size: 0.85rem;
  }
  
  .share button:hover {
    background: #f5f5f7;
  }
</style>
