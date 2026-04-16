import { Outlet } from "react-router-dom";

export default function AuthLayout() {
  return (
    <div className="container mx-auto max-w-md p-6">
      <Outlet />
    </div>
  );
}
