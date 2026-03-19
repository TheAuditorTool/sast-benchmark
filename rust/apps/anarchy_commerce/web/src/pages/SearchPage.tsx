/**
 * Search Page
 *
 * TAINT FLOW: User input → Gateway → Search Service (Rust) → Elasticsearch
 */

import React, { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { searchProducts, searchSuggestions } from '../api/client';
import type { Product } from '@anarchy/types';

export default function SearchPage() {
  const [query, setQuery] = useState('');
  const [submittedQuery, setSubmittedQuery] = useState('');
  const [filters, setFilters] = useState({
    category: '',
    minPrice: undefined as number | undefined,
    maxPrice: undefined as number | undefined,
    sortBy: 'relevance',
  });

  // TAINT SOURCE: User input in search query
  const { data: results, isLoading } = useQuery({
    queryKey: ['search', submittedQuery, filters],
    queryFn: () => searchProducts(submittedQuery, filters),
    enabled: !!submittedQuery,
  });

  // TAINT SOURCE: User input for autocomplete
  const { data: suggestions } = useQuery({
    queryKey: ['suggestions', query],
    queryFn: () => searchSuggestions(query),
    enabled: query.length > 2,
  });

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    // User input flows to search service
    setSubmittedQuery(query);
  };

  return (
    <div className="search-page">
      <h1>Search Products</h1>

      {/* TAINT SOURCE: Search input */}
      <form onSubmit={handleSearch}>
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search products..."
        />
        <button type="submit">Search</button>
      </form>

      {/* Autocomplete suggestions */}
      {suggestions && suggestions.length > 0 && (
        <ul className="suggestions">
          {suggestions.map((s, i) => (
            <li key={i} onClick={() => { setQuery(s); setSubmittedQuery(s); }}>
              {s}
            </li>
          ))}
        </ul>
      )}

      {/* Filters */}
      <div className="filters">
        <select
          value={filters.category}
          onChange={(e) => setFilters({ ...filters, category: e.target.value })}
        >
          <option value="">All Categories</option>
          <option value="electronics">Electronics</option>
          <option value="clothing">Clothing</option>
          <option value="home">Home & Garden</option>
        </select>

        <input
          type="number"
          placeholder="Min Price"
          onChange={(e) => setFilters({ ...filters, minPrice: Number(e.target.value) || undefined })}
        />
        <input
          type="number"
          placeholder="Max Price"
          onChange={(e) => setFilters({ ...filters, maxPrice: Number(e.target.value) || undefined })}
        />

        <select
          value={filters.sortBy}
          onChange={(e) => setFilters({ ...filters, sortBy: e.target.value })}
        >
          <option value="relevance">Relevance</option>
          <option value="price_asc">Price: Low to High</option>
          <option value="price_desc">Price: High to Low</option>
          <option value="newest">Newest</option>
        </select>
      </div>

      {/* Results */}
      {isLoading && <p>Loading...</p>}

      {results && (
        <div className="results">
          <p>{results.total} results for "{submittedQuery}"</p>
          <div className="product-grid">
            {results.items.map((product: Product) => (
              <div key={product.id} className="product-card">
                <img src={product.imageUrl} alt={product.name} />
                <h3>{product.name}</h3>
                <p className="price">${product.price}</p>
                {/* TAINT SINK: Product description might contain XSS if not sanitized */}
                <p className="description">{product.description}</p>
                <a href={`/product/${product.id}`}>View Details</a>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
