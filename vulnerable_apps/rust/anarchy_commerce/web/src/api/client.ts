/**
 * API Client - Web → Gateway communication
 *
 * All frontend requests go through the Gateway (BFF pattern).
 * The Gateway then routes to appropriate backend services.
 *
 * TAINT FLOWS (realistic):
 * 1. User input → Gateway → Users Service (Python) → PostgreSQL
 * 2. Payment data → Gateway → Payments Service (Rust) → Stripe
 * 3. Search query → Gateway → Search Service (Rust) → Elasticsearch
 */

import axios from 'axios';
import type { User, Product, PaymentIntent, SearchResult } from '@anarchy/types';

const GATEWAY_URL = import.meta.env.VITE_GATEWAY_URL || 'http://localhost:4000';

const client = axios.create({
  baseURL: GATEWAY_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add auth token to requests
client.interceptors.request.use((config) => {
  const token = localStorage.getItem('auth_token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// -----------------------------------------------------------------------------
// User Service (via Gateway → Python)
// -----------------------------------------------------------------------------

export async function registerUser(data: {
  email: string;
  password: string;
  name: string;
}): Promise<User> {
  // TAINT: User input flows to Python service → database
  const response = await client.post('/api/users/register', data);
  return response.data;
}

export async function loginUser(email: string, password: string): Promise<{ token: string; user: User }> {
  const response = await client.post('/api/users/login', { email, password });
  localStorage.setItem('auth_token', response.data.token);
  return response.data;
}

export async function getUserProfile(userId: string): Promise<User> {
  // TAINT: userId flows to Python service (potential IDOR)
  const response = await client.get(`/api/users/${userId}`);
  return response.data;
}

export async function updateUserProfile(userId: string, data: Partial<User>): Promise<User> {
  // TAINT: User-controlled data flows to Python service → database
  const response = await client.put(`/api/users/${userId}`, data);
  return response.data;
}

// -----------------------------------------------------------------------------
// Payments Service (via Gateway → Rust)
// -----------------------------------------------------------------------------

export async function createPaymentIntent(data: {
  amount: number;
  currency: string;
  orderId: string;
}): Promise<PaymentIntent> {
  // TAINT: Payment data flows to Rust service → Stripe API
  const response = await client.post('/api/payments/intent', data);
  return response.data;
}

export async function processPayment(paymentIntentId: string, paymentMethodId: string): Promise<PaymentIntent> {
  // TAINT: Payment IDs flow to Rust service
  const response = await client.post('/api/payments/process', {
    paymentIntentId,
    paymentMethodId,
  });
  return response.data;
}

export async function refundPayment(paymentId: string, reason: string): Promise<PaymentIntent> {
  // TAINT: reason flows to Rust service → Stripe → logs
  const response = await client.post(`/api/payments/${paymentId}/refund`, { reason });
  return response.data;
}

// -----------------------------------------------------------------------------
// Search Service (via Gateway → Rust)
// -----------------------------------------------------------------------------

export async function searchProducts(query: string, filters?: {
  category?: string;
  minPrice?: number;
  maxPrice?: number;
  sortBy?: string;
}): Promise<SearchResult<Product>> {
  // TAINT: query flows to Rust service → Elasticsearch
  // Potential for query injection if not properly escaped
  const response = await client.get('/api/search/products', {
    params: { q: query, ...filters },
  });
  return response.data;
}

export async function searchSuggestions(query: string): Promise<string[]> {
  // TAINT: query flows to Rust autocomplete
  const response = await client.get('/api/search/suggestions', {
    params: { q: query },
  });
  return response.data;
}

// -----------------------------------------------------------------------------
// Recommendations Service (via Gateway → Python)
// -----------------------------------------------------------------------------

export async function getRecommendations(userId: string, limit?: number): Promise<Product[]> {
  // TAINT: userId flows to Python ML service
  const response = await client.get(`/api/recommendations/${userId}`, {
    params: { limit },
  });
  return response.data;
}

export async function getSimilarProducts(productId: string): Promise<Product[]> {
  const response = await client.get(`/api/recommendations/similar/${productId}`);
  return response.data;
}

export default client;
