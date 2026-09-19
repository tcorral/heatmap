import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

import { getHeatmapSdk } from "../heatmap/init";

export function CheckoutPage() {
  const navigate = useNavigate();
  const [coupon, setCoupon] = useState("");

  useEffect(() => {
    const sdk = getHeatmapSdk();
    if (!sdk) {
      return;
    }
    let recorded = false;
    const onScroll = () => {
      if (recorded) {
        return;
      }
      recorded = true;
      sdk.recordScroll();
    };
    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  return (
    <section aria-labelledby="checkout-heading">
      <h1 id="checkout-heading">Checkout</h1>
      <div className="checkout-section">
        <h2>Order summary</h2>
        <p>Heatmap Pro — annual subscription</p>
        <p>Review details before you pay.</p>
      </div>
      <div className="checkout-section checkout-spacer">
        <h2>Delivery</h2>
        <p>Digital delivery — no shipping address required.</p>
        <p>Scroll to reach payment and coupon controls.</p>
      </div>
      <div className="checkout-section">
        <h2>Coupon</h2>
        <div className="coupon-row">
          <label htmlFor="coupon-code">Coupon</label>
          <input
            id="coupon-code"
            name="coupon"
            value={coupon}
            onChange={(event) => setCoupon(event.target.value)}
            autoComplete="off"
          />
          <button type="button" data-heatmap-id="checkout.coupon">
            Apply coupon
          </button>
        </div>
      </div>
      <div className="checkout-section">
        <h2>Payment</h2>
        <div className="checkout-actions">
          <button
            type="button"
            className="primary"
            data-heatmap-id="checkout.pay"
            onClick={() => navigate("/confirmation")}
          >
            Pay
          </button>
          <button type="button" onClick={() => navigate("/product")}>
            Cancel
          </button>
        </div>
      </div>
    </section>
  );
}
