# Nomad Site - Data Sources Research

> Document for tracking API integrations needed for the Nomad Site MVP and future phases.

## Current Status
- ✅ Backend: Running on port 3000
- ✅ Frontend: Running on port 5174
- ✅ Database: SQLite with mock data for Lisbon & Bali
- ❌ API integrations: Not yet implemented

---

## Data Sources Needed

### 1. Cost of Living

| Source | URL | Cost | Cities | Status |
|--------|-----|------|--------|--------|
| **Numbeo API** | https://www.numbeo.com/common/api.jsp | Paid (~$$month) | 600+ | **Recommended** |
| Kaggle Datasets | https://www.kaggle.com/datasets/debdutta/cost-of-living-index-by-country | Free | ~500 | Alternative |
| C2ER COL Index | https://www.coli.org/ | Paid | 300+ | US-focused |

**Recommendation:** Start with Numbeo for cost indices. For MVP, can manually scrape/key in data for top 50 cities.

---

### 2. Internet Speed

| Source | URL | Cost | Coverage | Status |
|--------|-----|------|----------|--------|
| **Speedtest Global Index** | https://www.speedtest.net/global-index | Free (limited) | Countries + some cities | **Recommended** |
| Ookla / Speedtest.net | https://www.ookla.com/ | Enterprise | Global | Enterprise only |
| Fair Internet Report API | https://fairinternetreport.com/broadband-data-api | Paid | Cities (daily updates) | Alternative |
| Kaggle | https://www.kaggle.com/datasets/prashertk/internet-broadband-and-mobile-speeds-by-country | Free | Countries only | Fallback |

**Recommendation:** Use Speedtest Global Index data (free). City-level data requires paid API.

---

### 3. Air Quality

| Source | URL | Cost | Coverage | Status |
|--------|-----|------|----------|--------|
| **OpenWeatherMap Air Pollution API** | https://openweathermap.org/api/air-pollution | Free tier | Global | **Recommended** |
| IQAir AirVisual | https://www.iqair.com/air-quality-api | Freemium | Global | Alternative |
| WAQI (World Air Quality Index) | https://waqi.info/ | Free (limited) | 1000+ cities | Alternative |

**Recommendation:** OpenWeatherMap free tier is sufficient for MVP.

---

### 4. Walkability

| Source | URL | Cost | Coverage | Status |
|--------|-----|------|----------|--------|
| **Walk Score API** | https://www.walkscore.com/solution/ap | Paid | US cities | **Recommended** |
| Numbeo | https://www.numbeo.com/property-investment/ | Paid (via API) | 200+ cities | Alternative |
| Google Maps | N/A | Enterprise | Global | Not accessible |

**Recommendation:** Use Numbeo safety/walkability indices (included in their API).

---

### 5. Safety / Crime

| Source | URL | Cost | Coverage | Status |
|--------|-----|------|----------|--------|
| **Numbeo Crime Index** | https://www.numbeo.com/crime/ | Paid | 400+ cities | **Recommended** |
| Numbeo Safety Index | https://www.numbeo.com/crime/rankings_current.jsp | Paid | Cities | Same as above |

**Recommendation:** Numbeo provides both crime and safety indices.

---

### 6. Weather

| Source | URL | Cost | Coverage | Status |
|--------|-----|------|----------|--------|
| **Open-Meteo** | https://open-meteo.com/ | **FREE** | Global | ✅ Ready to integrate |
| OpenWeatherMap | https://openweathermap.org/api | Freemium | Global | Alternative |
| WeatherAPI | https://www.weatherapi.com/ | Free tier | Global | Alternative |

**Recommendation:** Open-Meteo is completely free, no API key required. Perfect for MVP.

---

### 7. Coworking Spaces

| Source | URL | Cost | Coverage | Status |
|--------|-----|------|----------|--------|
| **Coworker API** | https://www.coworker.com/coworker-api | Paid | 25,000+ spaces | **Recommended** |
| Coworking Map API | https://coworkingmap.org/api/ | Free | ~3,000 spaces | Alternative |
| Scrape from Google Maps | N/A | Free (labor) | Global | Manual fallback |

**Recommendation:** Coworker API for counts by city. Free tier might have limits.

---

### 8. Fun / Nightlife

| Source | URL | Cost | Coverage | Status |
|--------|-----|------|----------|--------|
| **Numbeo Quality of Life** | https://www.numbeo.com/quality-of-life/ | Paid | 300+ cities | **Recommended** |
| TripAdvisor | N/A | N/A | Global | Scraping required |
| Yelp | N/A | N/A | US | Scraping required |

**Recommendation:** Numbeo has "Leisure & Culture" indices. For MVP, derive "fun" score from a combination of nightlife, restaurants, and cultural indices.

---

## Priority Order for Integration

### Phase 1 - MVP (Current)
- [x] Database schema with all fields
- [x] Mock data for Lisbon, Bali
- [x] Frontend with all tabs
- [ ] Add more destinations (10-20)

### Phase 2 - Basic API (This Week)
1. **Open-Meteo** - Weather data (free, no key)
2. **Numbeo** - Cost indices, safety, walkability (paid, city-level)

### Phase 3 - Enhanced Data (Next Sprint)
3. **Speedtest Global Index** - Internet speeds
4. **Coworker API** - Coworking counts
5. **OpenWeatherMap** - Air quality

### Phase 4 - Polish
6. Add remaining destinations (100+ cities)
7. Real-time data refresh
8. User contributions/reviews

---

## API Key Requirements

| API | Key Required? | Free Tier? | Signup URL |
|-----|--------------|------------|------------|
| Open-Meteo | ❌ No | ✅ Yes | https://open-meteo.com/ |
| Numbeo | ✅ Yes | ❌ No | https://www.numbeo.com/common/api.jsp |
| OpenWeatherMap | ✅ Yes | ✅ Yes | https://openweathermap.org/api |
| Coworker | ✅ Yes | ❌ No | https://www.coworker.com/coworker-api |
| Speedtest Index | ✅ Limited | ✅ Yes | https://www.speedtest.net/global-index |

---

## Notes

- **Numbeo** is the most comprehensive but is paid. Consider requesting academic API access or manually curating top cities.
- **Open-Meteo** is ideal for weather - no authentication needed.
- For MVP, focus on getting the UI right with mock data, then integrate one API at a time starting with Open-Meteo.

---

*Last updated: March 4, 2026*
