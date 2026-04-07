/**
 * Cart Store (Zustand)
 */

import { create } from 'zustand';
import type { CartItem } from '@anarchy/types';

interface CartState {
  items: CartItem[];
  total: number;
  addItem: (item: Omit<CartItem, 'quantity'>) => void;
  removeItem: (productId: string) => void;
  updateQuantity: (productId: string, quantity: number) => void;
  clearCart: () => void;
}

export const useCartStore = create<CartState>((set, get) => ({
  items: [],
  total: 0,

  addItem: (item) => {
    const items = get().items;
    const existing = items.find((i) => i.productId === item.productId);

    if (existing) {
      set({
        items: items.map((i) =>
          i.productId === item.productId
            ? { ...i, quantity: i.quantity + 1 }
            : i
        ),
      });
    } else {
      set({ items: [...items, { ...item, quantity: 1 }] });
    }

    // Recalculate total
    const newItems = get().items;
    set({ total: newItems.reduce((sum, i) => sum + i.price * i.quantity, 0) });
  },

  removeItem: (productId) => {
    const items = get().items.filter((i) => i.productId !== productId);
    set({ items, total: items.reduce((sum, i) => sum + i.price * i.quantity, 0) });
  },

  updateQuantity: (productId, quantity) => {
    if (quantity <= 0) {
      get().removeItem(productId);
      return;
    }

    const items = get().items.map((i) =>
      i.productId === productId ? { ...i, quantity } : i
    );
    set({ items, total: items.reduce((sum, i) => sum + i.price * i.quantity, 0) });
  },

  clearCart: () => set({ items: [], total: 0 }),
}));
