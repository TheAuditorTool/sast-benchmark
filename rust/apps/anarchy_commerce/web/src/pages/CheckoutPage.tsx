/**
 * Checkout Page
 *
 * TAINT FLOW: Payment data → Gateway → Payments Service (Rust) → Stripe
 */

import React, { useState } from 'react';
import { useMutation } from '@tanstack/react-query';
import { createPaymentIntent, processPayment } from '../api/client';
import { useCartStore } from '../stores/cart';

export default function CheckoutPage() {
  const { items, total, clearCart } = useCartStore();
  const [step, setStep] = useState<'shipping' | 'payment' | 'confirm'>('shipping');
  const [shipping, setShipping] = useState({
    name: '',
    address: '',
    city: '',
    postalCode: '',
    country: '',
  });
  const [paymentMethod, setPaymentMethod] = useState({
    cardNumber: '',
    expiry: '',
    cvc: '',
  });

  // TAINT: Payment amount flows to Rust service
  const createIntent = useMutation({
    mutationFn: () => createPaymentIntent({
      amount: total,
      currency: 'usd',
      orderId: `order_${Date.now()}`,
    }),
  });

  // TAINT: Payment credentials flow to Rust service → Stripe
  const processPaymentMutation = useMutation({
    mutationFn: (data: { intentId: string; methodId: string }) =>
      processPayment(data.intentId, data.methodId),
    onSuccess: () => {
      clearCart();
      alert('Payment successful!');
    },
  });

  const handleShippingSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setStep('payment');
  };

  const handlePaymentSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    // Create payment intent first
    const intent = await createIntent.mutateAsync();

    // Then process payment
    // In real app, this would use Stripe.js to tokenize card
    // For testing, we simulate the flow
    await processPaymentMutation.mutateAsync({
      intentId: intent.id,
      methodId: `pm_${paymentMethod.cardNumber.slice(-4)}`,
    });
  };

  return (
    <div className="checkout-page">
      <h1>Checkout</h1>

      {/* Order Summary */}
      <div className="order-summary">
        <h2>Order Summary</h2>
        {items.map((item) => (
          <div key={item.id} className="order-item">
            <span>{item.name} x {item.quantity}</span>
            <span>${item.price * item.quantity}</span>
          </div>
        ))}
        <div className="total">
          <strong>Total: ${total}</strong>
        </div>
      </div>

      {/* Shipping Form */}
      {step === 'shipping' && (
        <form onSubmit={handleShippingSubmit}>
          <h2>Shipping Information</h2>
          {/* TAINT SOURCE: User-controlled shipping data */}
          <input
            type="text"
            placeholder="Full Name"
            value={shipping.name}
            onChange={(e) => setShipping({ ...shipping, name: e.target.value })}
            required
          />
          <input
            type="text"
            placeholder="Address"
            value={shipping.address}
            onChange={(e) => setShipping({ ...shipping, address: e.target.value })}
            required
          />
          <input
            type="text"
            placeholder="City"
            value={shipping.city}
            onChange={(e) => setShipping({ ...shipping, city: e.target.value })}
            required
          />
          <input
            type="text"
            placeholder="Postal Code"
            value={shipping.postalCode}
            onChange={(e) => setShipping({ ...shipping, postalCode: e.target.value })}
            required
          />
          <input
            type="text"
            placeholder="Country"
            value={shipping.country}
            onChange={(e) => setShipping({ ...shipping, country: e.target.value })}
            required
          />
          <button type="submit">Continue to Payment</button>
        </form>
      )}

      {/* Payment Form */}
      {step === 'payment' && (
        <form onSubmit={handlePaymentSubmit}>
          <h2>Payment Information</h2>
          {/* TAINT SOURCE: Payment card data */}
          <input
            type="text"
            placeholder="Card Number"
            value={paymentMethod.cardNumber}
            onChange={(e) => setPaymentMethod({ ...paymentMethod, cardNumber: e.target.value })}
            required
          />
          <input
            type="text"
            placeholder="MM/YY"
            value={paymentMethod.expiry}
            onChange={(e) => setPaymentMethod({ ...paymentMethod, expiry: e.target.value })}
            required
          />
          <input
            type="text"
            placeholder="CVC"
            value={paymentMethod.cvc}
            onChange={(e) => setPaymentMethod({ ...paymentMethod, cvc: e.target.value })}
            required
          />
          <button type="submit" disabled={processPaymentMutation.isPending}>
            {processPaymentMutation.isPending ? 'Processing...' : 'Pay Now'}
          </button>
        </form>
      )}
    </div>
  );
}
