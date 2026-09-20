import { useNavigate } from "react-router-dom";

export function ProductPage() {
  const navigate = useNavigate();

  return (
    <section aria-labelledby="product-heading">
      <h1 id="product-heading">Product</h1>
      <p>Activate a plan to continue to checkout.</p>
      <button
        type="button"
        data-heatmap-id="product"
        onClick={() => navigate("/checkout")}
      >
        Activate product
      </button>
    </section>
  );
}
