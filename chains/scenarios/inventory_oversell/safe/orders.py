"""Order processing logic -- SAFE variant.

Uses a single UPDATE WHERE stock >= quantity.  The condition and the decrement
are evaluated atomically by the database engine; if rowcount is 0 the stock was
insufficient at the moment of the update (regardless of any concurrent order).

Chain: concurrent POST /order -> UPDATE stock WHERE stock >= quantity (atomic)
Individual findings: none -- check-and-decrement are atomic
Chain finding: none -- oversell is impossible with conditional UPDATE (CWE-362 mitigated)
"""
from inventory import get_db, get_stock


def place_order(product_id, quantity, user_id):
    """
    Place an order for quantity units of product_id.

    SAFE: the stock check is part of the UPDATE's WHERE clause, making it atomic.
    Concurrent orders each decrement from the actual current stock.
    """
    if quantity <= 0:
        return {"error": "Quantity must be positive"}, 400

    if get_stock(product_id) is None:
        return {"error": "Product not found"}, 404

    conn = get_db()

# vuln-code-snippet start chain_inventory_oversell_safe
    cursor = conn.execute(
        "UPDATE products SET stock = stock - ? WHERE id = ? AND stock >= ?",
        (quantity, product_id, quantity),
    )  # vuln-code-snippet safe-line chain_inventory_oversell_safe
# vuln-code-snippet end chain_inventory_oversell_safe

    if cursor.rowcount == 0:
        conn.close()
        return {"error": "Insufficient stock"}, 422

    conn.execute(
        "INSERT INTO orders (product_id, quantity, user_id) VALUES (?, ?, ?)",
        (product_id, quantity, user_id),
    )
    conn.commit()
    conn.close()
    return {"status": "ok", "ordered": quantity}, 200
