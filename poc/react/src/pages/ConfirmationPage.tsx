import { Link } from "react-router-dom";

export function ConfirmationPage() {
  return (
    <section aria-labelledby="confirmation-heading">
      <h1 id="confirmation-heading">Confirmation</h1>
      <p>Payment submitted. Your order is confirmed.</p>
      <Link className="button" to="/">
        Back to home
      </Link>
    </section>
  );
}
