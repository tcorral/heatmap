import { BrowserRouter, Link, Route, Routes } from "react-router-dom";
import { CheckoutPage } from "./pages/CheckoutPage";
import { ConfirmationPage } from "./pages/ConfirmationPage";
import { HomePage } from "./pages/HomePage";
import { ProductPage } from "./pages/ProductPage";

export function App() {
  return (
    <BrowserRouter>
      <div className="app-shell">
        <nav className="nav-links" aria-label="Checkout journey">
          <Link to="/">Home</Link>
          <Link to="/product">Product</Link>
          <Link to="/checkout">Checkout</Link>
          <Link to="/confirmation">Confirmation</Link>
        </nav>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/product" element={<ProductPage />} />
          <Route path="/checkout" element={<CheckoutPage />} />
          <Route path="/confirmation" element={<ConfirmationPage />} />
        </Routes>
      </div>
    </BrowserRouter>
  );
}
