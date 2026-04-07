from coupons import get_db, get_coupon

def redeem_coupon(code, user_id):
    coupon = get_coupon(code)
    if not coupon:
        return {"error": "Coupon not found"}, 404

    conn = get_db()

# vuln-code-snippet start ChainScenario0133A
    cursor = conn.execute(
        "UPDATE coupons SET used = 1 WHERE code = ? AND used = 0", (code,)
    )  # vuln-code-snippet target-line ChainScenario0133A
# vuln-code-snippet end ChainScenario0133A

    if cursor.rowcount == 0:
        conn.close()
        return {"error": "Coupon already redeemed"}, 409

    conn.execute(
        "INSERT INTO redemptions (coupon_code, user_id) VALUES (?, ?)",
        (code, user_id),
    )
    conn.commit()
    conn.close()

    return {"status": "ok", "discount_pct": coupon["discount_pct"]}, 200
