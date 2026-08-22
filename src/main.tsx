import React from "react"
import ReactDOM from "react-dom/client"
import { RouterProvider } from "react-router-dom"
import { AppInit } from "@/components/layout/AppInit"
import { Toaster } from "@/components/ui/sonner"
import { TooltipProvider } from "@/components/ui/tooltip"
import { router } from "@/lib/router"
import "./index.css"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <TooltipProvider>
      <AppInit />
      <RouterProvider router={router} />
      <Toaster />
    </TooltipProvider>
  </React.StrictMode>
)
