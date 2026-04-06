"""Order processing logic -- VULNERABLE variant.

Reads stock with SELECT then decrements with UPDATE.  Two concurrent orders
for the last unit in stock both read stock=1, both pass the quantity check,
and both decrement -- driving stock to -1 (oversell).

Chain: concurrent POST /order -> SELECT stock -> [race window] -> UPDATE stock - quantity
Individual findings: non-atomic stock check-and-decrement (medium)
Chain finding: oversell -- negative stock, product sold more than available (CWE-362, critical)
"""
from inventory import get_db, get_stock


def place_order(product_id, quantity, user_id):
    """
    Place an order for quantity units of product_id.

    VULNERABLE: stock check and decrement are two separate statements.
    Concurrent orders can both observe sufficient stock before either decrements.
    """
    if quantity <= 0:
        return {"error": "Quantity must be positive"}, 400

    stock = get_stock(product_id)
    if stock is None:
        return {"error": "Product not found"}, 404

    # TOCTOU: another concurrent order can pass this check with the same stale stock
    if stock < quantity:
        return {"error": "Insufficient stock"}, 422

    conn = get_db()

# vuln-code-snippet start chain_inventory_oversell_vuln
    conn.execute(
        "UPDATE products SET stock = stock - ? WHERE id = ?",
        (quantity, product_id),
    )  # vuln-code-snippet vuln-line chain_inventory_oversell_vuln
# vuln-code-snippet end chain_inventory_oversell_vuln

    conn.execute(
        "INSERT INTO orders (product_id, quantity, user_id) VALUES (?, ?, ?)",
        (product_id, quantity, user_id),
    )
    conn.commit()
    conn.close()
    return {"status": "ok", "ordered": quantity}, 200
