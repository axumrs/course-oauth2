import { createHashRouter } from "react-router-dom";
import { lazy } from "react";

const AuthLogin = lazy(() => import("@/pages/AuthLogin"));
const AuthRegister = lazy(() => import("@/pages/AuthRegister"));
const NewApplication = lazy(() => import("@/pages/NewApplication"));
const AuthLayout = lazy(() => import("@/components/layout/AuthLayout"));
const ApplicationDetail = lazy(() => import("@/pages/ApplicationDetail"));
const ApplicationList = lazy(() => import("@/pages/ApplicationList"));
const ApplicationLayout = lazy(
  () => import("@/components/layout/ApplicationLayout"),
);

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
    element: <ApplicationLayout />,
    children: [
      {
        path: "new",
        element: <NewApplication />,
      },
      {
        path: ":id",
        element: <ApplicationDetail />,
      },
      {
        index: true,
        element: <ApplicationList />,
      },
    ],
  },
]);

export default router;
