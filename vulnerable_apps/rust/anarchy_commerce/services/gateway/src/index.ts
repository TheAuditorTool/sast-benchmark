/**
 * API Gateway (BFF)
 *
 * Routes frontend requests to appropriate backend services.
 * This is the ONLY service the frontend talks to.
 *
 * Services:
 * - Users (Python/FastAPI): http://localhost:4001
 * - Payments (Rust/Actix): http://localhost:4002
 * - Search (Rust/Actix): http://localhost:4003
 * - Recommendations (Python/FastAPI): http://localhost:4004
 */

import express from 'express';
import cors from 'cors';
import helmet from 'helmet';
import morgan from 'morgan';
import axios from 'axios';

const app = express();
const PORT = process.env.PORT || 4000;

// Service URLs
const SERVICES = {
  users: process.env.USERS_SERVICE_URL || 'http://localhost:4001',
  payments: process.env.PAYMENTS_SERVICE_URL || 'http://localhost:4002',
  search: process.env.SEARCH_SERVICE_URL || 'http://localhost:4003',
  recommendations: process.env.RECOMMENDATIONS_SERVICE_URL || 'http://localhost:4004',
};

// Middleware
app.use(helmet());
app.use(cors({ origin: process.env.FRONTEND_URL || 'http://localhost:3000' }));
app.use(morgan('combined'));
app.use(express.json());

// -----------------------------------------------------------------------------
// Users Service Proxy (→ Python)
// -----------------------------------------------------------------------------

app.post('/api/users/register', async (req, res) => {
  try {
    // TAINT: User input forwarded to Python service
    const response = await axios.post(`${SERVICES.users}/register`, req.body);
    res.json(response.data);
  } catch (error: any) {
    res.status(error.response?.status || 500).json(error.response?.data || { error: 'Service unavailable' });
  }
});

app.post('/api/users/login', async (req, res) => {
  try {
    // TAINT: Credentials forwarded to Python service
    const response = await axios.post(`${SERVICES.users}/login`, req.body);
    res.json(response.data);
  } catch (error: any) {
    res.status(error.response?.status || 500).json(error.response?.data || { error: 'Service unavailable' });
  }
});

app.get('/api/users/:userId', async (req, res) => {
  try {
    // TAINT: userId flows to Python service (IDOR if not validated)
    const response = await axios.get(`${SERVICES.users}/users/${req.params.userId}`, {
      headers: { Authorization: req.headers.authorization || '' },
    });
    res.json(response.data);
  } catch (error: any) {
    res.status(error.response?.status || 500).json(error.response?.data || { error: 'Service unavailable' });
  }
});

app.put('/api/users/:userId', async (req, res) => {
  try {
    // TAINT: User-controlled body forwarded to Python service
    const response = await axios.put(`${SERVICES.users}/users/${req.params.userId}`, req.body, {
      headers: { Authorization: req.headers.authorization || '' },
    });
    res.json(response.data);
  } catch (error: any) {
    res.status(error.response?.status || 500).json(error.response?.data || { error: 'Service unavailable' });
  }
});

// -----------------------------------------------------------------------------
// Payments Service Proxy (→ Rust)
// -----------------------------------------------------------------------------

app.post('/api/payments/intent', async (req, res) => {
  try {
    // TAINT: Payment data forwarded to Rust service
    const response = await axios.post(`${SERVICES.payments}/intent`, req.body, {
      headers: { Authorization: req.headers.authorization || '' },
    });
    res.json(response.data);
  } catch (error: any) {
    res.status(error.response?.status || 500).json(error.response?.data || { error: 'Service unavailable' });
  }
});

app.post('/api/payments/process', async (req, res) => {
  try {
    // TAINT: Payment intent and method IDs forwarded to Rust service
    const response = await axios.post(`${SERVICES.payments}/process`, req.body, {
      headers: { Authorization: req.headers.authorization || '' },
    });
    res.json(response.data);
  } catch (error: any) {
    res.status(error.response?.status || 500).json(error.response?.data || { error: 'Service unavailable' });
  }
});

app.post('/api/payments/:paymentId/refund', async (req, res) => {
  try {
    // TAINT: paymentId and reason forwarded to Rust service
    const response = await axios.post(
      `${SERVICES.payments}/${req.params.paymentId}/refund`,
      req.body,
      { headers: { Authorization: req.headers.authorization || '' } }
    );
    res.json(response.data);
  } catch (error: any) {
    res.status(error.response?.status || 500).json(error.response?.data || { error: 'Service unavailable' });
  }
});

// -----------------------------------------------------------------------------
// Search Service Proxy (→ Rust)
// -----------------------------------------------------------------------------

app.get('/api/search/products', async (req, res) => {
  try {
    // TAINT: Search query forwarded to Rust search service
    const response = await axios.get(`${SERVICES.search}/products`, {
      params: req.query,
    });
    res.json(response.data);
  } catch (error: any) {
    res.status(error.response?.status || 500).json(error.response?.data || { error: 'Service unavailable' });
  }
});

app.get('/api/search/suggestions', async (req, res) => {
  try {
    // TAINT: Query forwarded for autocomplete
    const response = await axios.get(`${SERVICES.search}/suggestions`, {
      params: req.query,
    });
    res.json(response.data);
  } catch (error: any) {
    res.status(error.response?.status || 500).json(error.response?.data || { error: 'Service unavailable' });
  }
});

// -----------------------------------------------------------------------------
// Recommendations Service Proxy (→ Python)
// -----------------------------------------------------------------------------

app.get('/api/recommendations/:userId', async (req, res) => {
  try {
    // TAINT: userId forwarded to Python ML service
    const response = await axios.get(`${SERVICES.recommendations}/user/${req.params.userId}`, {
      params: req.query,
      headers: { Authorization: req.headers.authorization || '' },
    });
    res.json(response.data);
  } catch (error: any) {
    res.status(error.response?.status || 500).json(error.response?.data || { error: 'Service unavailable' });
  }
});

app.get('/api/recommendations/similar/:productId', async (req, res) => {
  try {
    const response = await axios.get(`${SERVICES.recommendations}/similar/${req.params.productId}`);
    res.json(response.data);
  } catch (error: any) {
    res.status(error.response?.status || 500).json(error.response?.data || { error: 'Service unavailable' });
  }
});

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'ok', services: SERVICES });
});

app.listen(PORT, () => {
  console.log(`Gateway running on port ${PORT}`);
  console.log('Service endpoints:', SERVICES);
});
