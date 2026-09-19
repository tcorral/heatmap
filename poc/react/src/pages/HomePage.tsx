import { Link } from "react-router-dom";

export function HomePage() {
  return (
    <section aria-labelledby="home-heading">
      <h1 id="home-heading">Home</h1>
      <p>Shared checkout journey starts here.</p>
      <Link className="button" to="/product">
        Browse products
      </Link>
    </section>
  );
}
