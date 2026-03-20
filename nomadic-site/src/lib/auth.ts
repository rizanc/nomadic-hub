import { writable } from 'svelte/store';

interface User {
  id: number;
  email: string;
  first_name: string;
  last_name: string;
  avatar_url: string | null;
  location_city: string | null;
  location_country: string | null;
  created_at: string;
}

const API = 'http://localhost:3000';

function createAuthStore() {
  const { subscribe, set, update } = writable<{
    user: User | null;
    token: string | null;
    loading: boolean;
  }>({
    user: null,
    token: null,
    loading: true
  });

  return {
    subscribe,
    
    init: async () => {
      if (typeof window === 'undefined') return;
      
      const token = localStorage.getItem('nomadic_token');
      if (!token) {
        set({ user: null, token: null, loading: false });
        return;
      }

      try {
        const res = await fetch(`${API}/api/auth/me`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(token)
        });
        
        if (res.ok) {
          const user = await res.json();
          set({ user, token, loading: false });
        } else {
          localStorage.removeItem('nomadic_token');
          set({ user: null, token: null, loading: false });
        }
      } catch (e) {
        set({ user: null, token: null, loading: false });
      }
    },

    register: async (email: string, password: string, firstName: string, lastName: string) => {
      const res = await fetch(`${API}/api/auth/register`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password, first_name: firstName, last_name: lastName })
      });
      
      if (!res.ok) {
        const err = await res.json();
        throw new Error(err.error || 'Registration failed');
      }
      
      const data = await res.json();
      localStorage.setItem('nomadic_token', data.token);
      set({ user: data.user, token: data.token, loading: false });
      return data;
    },

    login: async (email: string, password: string) => {
      const res = await fetch(`${API}/api/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password })
      });
      
      if (!res.ok) {
        throw new Error('Invalid email or password');
      }
      
      const data = await res.json();
      localStorage.setItem('nomadic_token', data.token);
      set({ user: data.user, token: data.token, loading: false });
      return data;
    },

    logout: () => {
      localStorage.removeItem('nomadic_token');
      set({ user: null, token: null, loading: false });
    },

    updateProfile: async (updates: { 
      first_name?: string; 
      last_name?: string;
      avatar_url?: string;
      location_city?: string;
      location_country?: string;
    }) => {
      let currentToken: string | null = null;
      const unsubscribe = subscribe(s => currentToken = s.token);
      unsubscribe();
      
      const res = await fetch(`${API}/api/auth/profile`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify([currentToken, updates])
      });
      
      if (!res.ok) {
        throw new Error('Failed to update profile');
      }
      
      const user = await res.json();
      update(s => ({ ...s, user }));
      return user;
    },

    changePassword: async (currentPassword: string, newPassword: string) => {
      let currentToken: string | null = null;
      const unsubscribe = subscribe(s => currentToken = s.token);
      unsubscribe();
      
      const res = await fetch(`${API}/api/auth/password`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify([currentToken, { current_password: currentPassword, new_password: newPassword }])
      });
      
      if (!res.ok) {
        const err = await res.json();
        throw new Error(err.error || 'Failed to change password');
      }
      
      return await res.json();
    },

    validateLocation: async (city: string, country: string) => {
      const res = await fetch(`${API}/api/auth/validate-location`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ city, country })
      });
      
      if (!res.ok) {
        throw new Error('Failed to validate location');
      }
      
      return await res.json();
    }
  };
}

export const auth = createAuthStore();
