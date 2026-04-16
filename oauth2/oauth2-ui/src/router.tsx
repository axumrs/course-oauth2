import { createHashRouter } from "react-router-dom";
import { lazy } from "react";

const AuthLogin = lazy(() => import("@/pages/AuthLogin"));
const AuthRegister = lazy(() => import("@/pages/AuthRegister"));
const NewApplication = lazy(() => import("@/pages/NewApplication"));
const AuthLayout = lazy(() => import("@/components/layout/AuthLayout"));

const router = createHashRouter([
  {
    path: "/auth",
    element: <AuthLayout />,
    children: [
      {
        path: "login",
        element: <AuthLogin />,
      },
      {
        path: "register",
        element: <AuthRegister />,
      },
    ],
  },
  {
    path: "/application",
    children: [
      {
        path: "new",
        element: <NewApplication />,
      },
    ],
  },
]);

export default router;
