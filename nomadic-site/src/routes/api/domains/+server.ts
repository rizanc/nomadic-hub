// Domain search API route
import { json } from '@sveltejs/kit';

const DOMAINS = [
  { name: 'nomad.cool', tld: 'cool', status: 'available', price: 19.99 },
  { name: 'wander.io', tld: 'io', status: 'taken', price: null },
  { name: 'roam.io', tld: 'io', status: 'available', price: 49.99 },
  { name: 'nomad.io', tld: 'io', status: 'taken', price: null },
  { name: 'drift.io', tld: 'io', status: 'available', price: 39.99 },
  { name: 'digitalnomad.tools', tld: 'tools', status: 'available', price: 12.99 },
  { name: 'nomadlife.io', tld: 'io', status: 'available', price: 44.99 },
  { name: 'vanlife.express', tld: 'express', status: 'available', price: 24.99 },
  { name: 'thenomad.io', tld: 'io', status: 'available', price: 34.99 },
  { name: 'nomadhub.io', tld: 'io', status: 'taken', price: null },
  { name: 'nomadpass.com', tld: 'com', status: 'available', price: 29.99 },
  { name: 'wanderwork.io', tld: 'io', status: 'available', price: 39.99 },
  { name: 'globedrift.com', tld: 'com', status: 'available', price: 19.99 },
  { name: 'roamable.com', tld: 'com', status: 'available', price: 24.99 },
  { name: 'stayanywhere.io', tld: 'io', status: 'available', price: 34.99 },
];

export async function GET({ url }) {
  const query = url.searchParams.get('q')?.toLowerCase() || '';
  
  // Simulate API delay
  await new Promise(r => setTimeout(r, 500));
  
  const results = DOMAINS.filter(d => 
    d.name.includes(query) || d.tld.includes(query)
  );
  
  return json({ results, query });
}
