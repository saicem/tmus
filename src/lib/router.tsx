import { createMemoryRouter } from "react-router-dom"
import { AppLayout } from "@/components/layout/AppLayout"
import { Application } from "@/pages/Application"
import { CategoryPage } from "@/pages/Category"
import { Detail } from "@/pages/Detail"
import { Overview } from "@/pages/Overview"
import { Setting } from "@/pages/Setting"
import { Statistic } from "@/pages/Statistic"
import { Timeline } from "@/pages/Timeline"

export const router = createMemoryRouter([
  {
    path: "/",
    element: <AppLayout />,
    children: [
      { index: true, element: <Overview /> },
      { path: "timeline", element: <Timeline /> },
      { path: "statistic", element: <Statistic /> },
      { path: "application", element: <Application /> },
      { path: "category", element: <CategoryPage /> },
      { path: "setting", element: <Setting /> },
      { path: "detail/:id", element: <Detail /> },
    ],
  },
])
