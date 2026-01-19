import { createBrowserRouter } from "react-router";
import Home from "../pages/Home";
import Test from "../pages/Test";
import App from "../App";

const router = createBrowserRouter([
  {
    path: "/",
    Component: App,
    children: [
      { index: true, Component: Home },
      { path: "home", Component: Home },
      { path: "test", Component: Test },
    ],
  },
]);

export default router;