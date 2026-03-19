/**
 * Shared TypeScript types for Anarchy Commerce
 *
 * Used by:
 * - web/ (React frontend)
 * - services/gateway/ (Node.js BFF)
 *
 * These types define the API contract between services.
 */

// -----------------------------------------------------------------------------
// User Types (Users Service - Python)
// -----------------------------------------------------------------------------

export interface User {
  id: string;
  email: string;
  name: string;
  bio?: string;
  avatarUrl?: string;
  createdAt?: string;
  updatedAt?: string;
}

export interface UserRegisterRequest {
  email: string;
  password: string;
  name: string;
}

export interface UserLoginRequest {
  email: string;
  password: string;
}

export interface UserLoginResponse {
  token: string;
  user: User;
}

export interface UserUpdateRequest {
  name?: string;
  bio?: string;
  avatarUrl?: string;
}

// -----------------------------------------------------------------------------
// Product Types (Search Service - Rust)
// -----------------------------------------------------------------------------

export interface Product {
  id: string;
  name: string;
  description: string;
  price: number;
  category: string;
  imageUrl: string;
  stock?: number;
  rating?: number;
  reviewCount?: number;
}

export interface SearchFilters {
  category?: string;
  minPrice?: number;
  maxPrice?: number;
  sortBy?: 'relevance' | 'price_asc' | 'price_desc' | 'newest' | 'rating';
}

export interface SearchResult<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
}

// -----------------------------------------------------------------------------
// Payment Types (Payments Service - Rust)
// -----------------------------------------------------------------------------

export interface PaymentIntent {
  id: string;
  amount: number;
  currency: string;
  status: 'pending' | 'processing' | 'succeeded' | 'failed' | 'refunded';
  orderId: string;
  createdAt: string;
  updatedAt?: string;
}

export interface CreatePaymentIntentRequest {
  amount: number;
  currency: string;
  orderId: string;
}

export interface ProcessPaymentRequest {
  paymentIntentId: string;
  paymentMethodId: string;
}

export interface RefundRequest {
  reason: string;
}

// -----------------------------------------------------------------------------
// Order Types
// -----------------------------------------------------------------------------

export interface OrderItem {
  productId: string;
  quantity: number;
  price: number;
}

export interface Order {
  id: string;
  userId: string;
  items: OrderItem[];
  total: number;
  status: 'pending' | 'paid' | 'shipped' | 'delivered' | 'cancelled';
  shippingAddress: ShippingAddress;
  paymentIntentId?: string;
  createdAt: string;
}

export interface ShippingAddress {
  name: string;
  address: string;
  city: string;
  postalCode: string;
  country: string;
}

// -----------------------------------------------------------------------------
// Cart Types (Frontend-only, stored in zustand)
// -----------------------------------------------------------------------------

export interface CartItem {
  id: string;
  productId: string;
  name: string;
  price: number;
  quantity: number;
  imageUrl: string;
}

// -----------------------------------------------------------------------------
// API Response Types
// -----------------------------------------------------------------------------

export interface ApiError {
  error: string;
  message?: string;
  details?: Record<string, unknown>;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  pageSize: number;
  hasNext: boolean;
  hasPrev: boolean;
}
