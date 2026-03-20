// Visa requirements API
import { json } from '@sveltejs/kit';

const VISAS = [
  { country: 'Portugal', flag: '🇵🇹', nomadVisa: true, maxStay: '2 years', income: '€760/month', tax: 'NHR program' },
  { country: 'Spain', flag: '🇪🇸', nomadVisa: true, maxStay: '3 years', income: '€2,334/month', tax: 'Beckham law' },
  { country: 'Estonia', flag: '🇪🇪', nomadVisa: true, maxStay: '1 year', income: '€4,500/month', tax: 'None first 6mo' },
  { country: 'Croatia', flag: '🇭🇷', nomadVisa: true, maxStay: '1 year', income: '€2,230/month', tax: 'None' },
  { country: 'Germany', flag: '🇩🇪', nomadVisa: true, maxStay: '3 years', income: '€2,334/month', tax: 'varies' },
  { country: 'Mexico', flag: '🇲🇽', nomadVisa: true, maxStay: '4 years', income: '$2,600/month', tax: 'No tax on <$100k' },
  { country: 'Colombia', flag: '🇨🇴', nomadVisa: true, maxStay: '2 years', income: '$700/month', tax: 'No tax on foreign income' },
  { country: 'Indonesia', flag: '🇮🇩', nomadVisa: true, maxStay: '6 months', income: '$1,300/month', tax: 'None first 4mo' },
  { country: 'Thailand', flag: '🇹🇭', nomadVisa: true, maxStay: '5 years (LTR)', income: '$1,600/month', tax: 'None 10y' },
  { country: 'UAE', flag: '🇦🇪', nomadVisa: true, maxStay: '1 year', income: '$5,000/month', tax: 'No income tax' },
  { country: 'Japan', flag: '🇯🇵', nomadVisa: true, maxStay: '6 months', income: '¥1.5M/year', tax: 'varies' },
  { country: 'Costa Rica', flag: '🇨🇷', nomadVisa: true, maxStay: '2 years', income: '$2,500/month', tax: 'No foreign income' },
  { country: 'Georgia', flag: '🇬🇪', nomadVisa: true, maxStay: '1 year', income: '€1,700/month', tax: 'None' },
  { country: 'Albania', flag: '🇦🇱', nomadVisa: true, maxStay: '1 year', income: '€1,500/month', tax: 'None' },
  { country: 'Malaysia', flag: '🇲🇾', nomadVisa: true, maxStay: '1 year', income: 'RM 10k/month', tax: 'None' },
];

export async function GET({ url }) {
  const query = url.searchParams.get('q')?.toLowerCase() || '';
  
  const results = VISAS.filter(v => 
    v.country.toLowerCase().includes(query) || 
    (v.nomadVisa && query === 'nomad')
  );
  
  return json({ results, total: results.length });
}
