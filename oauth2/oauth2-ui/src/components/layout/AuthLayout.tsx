import { Outlet } from "react-router-dom";
import Container from "../Container";

export default function AuthLayout() {
  return (
    <Container className="max-w-md p-6">
      <Outlet />
    </Container>
  );
}
