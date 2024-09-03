import "./globals.css";
import { StrictMode } from 'react'
import ReactDOM from 'react-dom/client'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { ThemeProvider } from "@/components/provider/theme-provider"
import { Toaster } from "@/components/ui/toaster"

// Import the generated route tree
import { routeTree } from './routeTree.gen'
import { EventProvider } from "./components/provider/events";

// Create a new router instance
const router = createRouter({ routeTree })

// Register the router instance for type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <EventProvider>
      <ThemeProvider defaultTheme="dark">
        <RouterProvider router={router} />
        <Toaster />
      </ThemeProvider>
    </EventProvider>
  </StrictMode>,
);
