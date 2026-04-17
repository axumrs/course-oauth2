import { Link, Outlet } from "react-router-dom";
import Container from "../Container";

export default function ApplicationLayout() {
  return (
    <>
      <header className="bg-secondary shadow">
        <Container className="flex justify-between items-center px-3 py-1">
          <Link
            to="/application"
            className="flex justify-start items-center gap-x-2"
          >
            <img src="/logo.png" className="w-10 object-cover" />
            <h1 className="text-xl">AXUM OAuth2</h1>
          </Link>
        </Container>
      </header>
      <Container className="my-3 px-3">
        <Outlet />
      </Container>
    </>
  );
}
